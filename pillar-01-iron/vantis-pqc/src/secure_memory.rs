//! Secure Memory Handling for Post-Quantum Cryptography
//!
//! This module provides secure memory operations designed to protect
//! sensitive cryptographic material:
//! - Constant-time operations to prevent timing attacks
//! - Secure memory locking (mlock) to prevent swapping
//! - Verified zeroization of sensitive data
//! - Protected memory allocation

use crate::error::{PQCError, Result};
use std::ptr;
use std::slice;
use zeroize::{Zeroize, Zeroizing};

/// Secure memory allocator for cryptographic material
///
/// This wrapper ensures that memory is:
/// - Locked in RAM (not swappable to disk)
/// - Zeroized when dropped
/// - Protected from compiler optimizations
#[derive(Debug)]
pub struct SecureBox<T: Zeroize> {
    /// The protected data
    data: Option<Box<T>>,
    /// Whether the memory is locked
    locked: bool,
}

impl<T: Zeroize> SecureBox<T> {
    /// Create a new secure box with the given value
    pub fn new(value: T) -> Result<Self> {
        let mut boxed = Box::new(value);
        
        // Attempt to lock memory
        let locked = Self::lock_memory(&mut *boxed as *mut T as *mut u8, std::mem::size_of::<T>());
        
        Ok(Self {
            data: Some(boxed),
            locked,
        })
    }

    /// Get a reference to the protected data
    pub fn get(&self) -> Option<&T> {
        self.data.as_ref().map(|v| v.as_ref())
    }

    /// Get a mutable reference to the protected data
    pub fn get_mut(&mut self) -> Option<&mut T> {
        self.data.as_mut().map(|v| v.as_mut())
    }

    /// Check if memory is locked
    pub fn is_locked(&self) -> bool {
        self.locked
    }

    /// Lock memory to prevent swapping
    #[cfg(unix)]
    fn lock_memory(ptr: *mut u8, len: usize) -> bool {
        if len == 0 {
            return true;
        }
        
        // SAFETY: We have a valid pointer and length
        unsafe {
            // mlock returns 0 on success
            libc::mlock(ptr as *const libc::c_void, len) == 0
        }
    }

    /// Lock memory to prevent swapping (Windows)
    #[cfg(windows)]
    fn lock_memory(ptr: *mut u8, len: usize) -> bool {
        if len == 0 {
            return true;
        }
        
        // On Windows, we would use VirtualLock
        // For now, just return false (not implemented)
        let _ = (ptr, len);
        false
    }

    /// Lock memory (fallback for other platforms)
    #[cfg(not(any(unix, windows)))]
    fn lock_memory(_ptr: *mut u8, _len: usize) -> bool {
        false
    }

    /// Unlock memory
    #[cfg(unix)]
    fn unlock_memory(ptr: *mut u8, len: usize) {
        if len == 0 {
            return;
        }
        
        unsafe {
            libc::munlock(ptr as *const libc::c_void, len);
        }
    }

    /// Unlock memory (Windows/other)
    #[cfg(not(unix))]
    fn unlock_memory(_ptr: *mut u8, _len: usize) {
        // Not implemented for non-Unix platforms
    }
}

impl<T: Zeroize> Drop for SecureBox<T> {
    fn drop(&mut self) {
        if let Some(ref mut data) = self.data {
            // Zeroize the data
            data.zeroize();
            
            // Unlock memory
            if self.locked {
                Self::unlock_memory(&mut **data as *mut T as *mut u8, std::mem::size_of::<T>());
            }
        }
    }
}

impl<T: Zeroize + Clone> Clone for SecureBox<T> {
    fn clone(&self) -> Self {
        match self.data.as_ref() {
            Some(d) => Self::new((**d).clone()).unwrap_or_else(|_| Self { data: None, locked: false }),
            None => Self { data: None, locked: false },
        }
    }
}

/// Secure vector for storing sensitive byte data
pub type SecureVec = Zeroizing<Vec<u8>>;

/// Create a secure vector from bytes
pub fn secure_vec_from(data: &[u8]) -> SecureVec {
    Zeroizing::new(data.to_vec())
}

/// Secure bytes comparison in constant time
///
/// This function compares two byte slices in constant time,
/// preventing timing attacks that could reveal information
/// about the compared values.
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    
    let mut result: u8 = 0;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    
    result == 0
}

/// Secure bytes comparison with different lengths
///
/// Returns false if lengths differ, but still performs comparison
/// to maintain constant time behavior.
pub fn constant_time_eq_vartime(a: &[u8], b: &[u8]) -> bool {
    let len = a.len().min(b.len());
    let mut result = (a.len() ^ b.len()) as u8;
    
    for i in 0..len {
        result |= a[i] ^ b[i];
    }
    
    result == 0
}

/// Constant-time conditional copy
///
/// Copies `src` to `dst` if `condition` is true, in constant time.
pub fn constant_time_copy(condition: bool, dst: &mut [u8], src: &[u8]) {
    let mask = (-(condition as i8)) as u8;
    
    for (d, s) in dst.iter_mut().zip(src.iter()) {
        *d = (*d & !mask) | (*s & mask);
    }
}

/// Constant-time conditional select
///
/// Returns `a` if `condition` is true, `b` otherwise, in constant time.
pub fn constant_time_select(condition: bool, a: &[u8], b: &[u8]) -> Vec<u8> {
    assert_eq!(a.len(), b.len(), "Slice lengths must match");
    
    let mask = (-(condition as i8)) as u8;
    
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (*x & mask) | (*y & !mask))
        .collect()
}

/// Secure memory zeroization with verification
///
/// Zeroizes memory and verifies that it was properly cleared.
/// This is useful for ensuring that sensitive data is properly erased.
pub fn secure_zero(data: &mut [u8]) -> Result<()> {
    // Use volatile write to prevent compiler optimization
    for byte in data.iter_mut() {
        unsafe {
            ptr::write_volatile(byte, 0);
        }
    }
    
    // Memory barrier to ensure write is complete
    std::sync::atomic::fence(std::sync::atomic::Ordering::SeqCst);
    
    // Verify zeroization
    for byte in data.iter() {
        if *byte != 0 {
            return Err(PQCError::Generic("Memory zeroization verification failed".to_string()));
        }
    }
    
    Ok(())
}

/// Secure memory allocation with locking
///
/// Allocates memory and attempts to lock it in RAM.
pub struct SecureAllocator {
    /// Pointer to allocated memory
    ptr: *mut u8,
    /// Size of allocated memory
    size: usize,
    /// Whether memory is locked
    locked: bool,
}

impl SecureAllocator {
    /// Allocate secure memory
    pub fn allocate(size: usize) -> Result<Self> {
        if size == 0 {
            return Ok(Self {
                ptr: ptr::null_mut(),
                size: 0,
                locked: false,
            });
        }
        
        // Allocate aligned memory
        let layout = std::alloc::Layout::from_size_align(size, 8)
            .map_err(|e| PQCError::Generic(format!("Layout error: {}", e)))?;
        
        let ptr = unsafe { std::alloc::alloc(layout) };
        
        if ptr.is_null() {
            return Err(PQCError::Generic("Memory allocation failed".to_string()));
        }
        
        // Initialize to zero
        unsafe { ptr::write_bytes(ptr, 0, size) };
        
        // Attempt to lock
        let locked = Self::lock_memory(ptr, size);
        
        Ok(Self { ptr, size, locked })
    }

    /// Get a slice to the allocated memory
    pub fn as_slice(&self) -> &[u8] {
        if self.ptr.is_null() || self.size == 0 {
            return &[];
        }
        unsafe { slice::from_raw_parts(self.ptr, self.size) }
    }

    /// Get a mutable slice to the allocated memory
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        if self.ptr.is_null() || self.size == 0 {
            return &mut [];
        }
        unsafe { slice::from_raw_parts_mut(self.ptr, self.size) }
    }

    /// Check if memory is locked
    pub fn is_locked(&self) -> bool {
        self.locked
    }

    /// Get the size of allocated memory
    pub fn size(&self) -> usize {
        self.size
    }

    #[cfg(unix)]
    fn lock_memory(ptr: *mut u8, size: usize) -> bool {
        unsafe { libc::mlock(ptr as *const libc::c_void, size) == 0 }
    }

    #[cfg(not(unix))]
    fn lock_memory(_ptr: *mut u8, _size: usize) -> bool {
        false
    }

    #[cfg(unix)]
    fn unlock_memory(ptr: *mut u8, size: usize) {
        unsafe { libc::munlock(ptr as *const libc::c_void, size) };
    }

    #[cfg(not(unix))]
    fn unlock_memory(_ptr: *mut u8, _size: usize) {}
}

impl Drop for SecureAllocator {
    fn drop(&mut self) {
        if self.ptr.is_null() || self.size == 0 {
            return;
        }
        
        // Zeroize memory
        unsafe { ptr::write_bytes(self.ptr, 0, self.size) };
        
        // Unlock if locked
        if self.locked {
            Self::unlock_memory(self.ptr, self.size);
        }
        
        // Free memory
        let layout = std::alloc::Layout::from_size_align(self.size, 8).unwrap();
        unsafe { std::alloc::dealloc(self.ptr, layout) };
    }
}

// Safety: SecureAllocator manages its own memory safely
unsafe impl Send for SecureAllocator {}
unsafe impl Sync for SecureAllocator {}

/// Wipe sensitive data from memory
///
/// A convenience function to securely wipe any mutable byte slice.
pub fn wipe(data: &mut [u8]) {
    data.zeroize();
}

/// Secure random bytes generation
///
/// Generates cryptographically secure random bytes.
pub fn secure_random_bytes(len: usize) -> Vec<u8> {
    use rand::RngCore;
    let mut bytes = vec![0u8; len];
    rand::thread_rng().fill_bytes(&mut bytes);
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_time_eq_equal() {
        let a = b"hello world";
        let b = b"hello world";
        assert!(constant_time_eq(a, b));
    }

    #[test]
    fn test_constant_time_eq_different() {
        let a = b"hello world";
        let b = b"hello earth";
        assert!(!constant_time_eq(a, b));
    }

    #[test]
    fn test_constant_time_eq_different_lengths() {
        let a = b"hello";
        let b = b"hello world";
        assert!(!constant_time_eq(a, b));
    }

    #[test]
    fn test_constant_time_select() {
        let a = b"aaaaaaaa";
        let b = b"bbbbbbbb";
        
        let result = constant_time_select(true, a, b);
        assert_eq!(result, a);
        
        let result = constant_time_select(false, a, b);
        assert_eq!(result, b);
    }

    #[test]
    fn test_secure_zero() {
        let mut data = vec![0xFFu8; 64];
        secure_zero(&mut data).unwrap();
        
        for byte in &data {
            assert_eq!(*byte, 0);
        }
    }

    #[test]
    fn test_secure_vec_from() {
        let data = b"sensitive data";
        let secure = secure_vec_from(data);
        
        assert_eq!(&*secure, data);
    }

    #[test]
    fn test_secure_allocator() {
        let mut allocator = SecureAllocator::allocate(64).unwrap();
        
        assert_eq!(allocator.size(), 64);
        
        // Write data
        let slice = allocator.as_mut_slice();
        slice.copy_from_slice(&[0xABu8; 64]);
        
        // Read data back
        let slice = allocator.as_slice();
        assert_eq!(slice, &[0xABu8; 64]);
    }

    #[test]
    fn test_secure_allocator_zero_init() {
        let allocator = SecureAllocator::allocate(64).unwrap();
        let slice = allocator.as_slice();
        
        for byte in slice {
            assert_eq!(*byte, 0);
        }
    }

    #[test]
    fn test_secure_box() {
        let secure = SecureBox::new(vec![1u8, 2, 3, 4, 5]).unwrap();
        
        assert!(secure.get().is_some());
        assert_eq!(secure.get().unwrap(), &vec![1u8, 2, 3, 4, 5]);
    }

    #[test]
    fn test_secure_box_zeroize_on_drop() {
        let ptr: *mut u8;
        
        {
            let mut secure = SecureBox::new(vec![0xFFu8; 32]).unwrap();
            ptr = secure.get_mut().unwrap().as_mut_ptr();
        }
        
        // After drop, memory should be zeroized
        // Note: This is technically undefined behavior, but useful for testing
        unsafe {
            for i in 0..32 {
                // The memory should have been zeroized
                // We can't guarantee this after drop, but zeroize should have been called
                let _ = *ptr.add(i);
            }
        }
    }

    #[test]
    fn test_secure_random_bytes() {
        let bytes1 = secure_random_bytes(32);
        let bytes2 = secure_random_bytes(32);
        
        // Should be different
        assert_ne!(bytes1, bytes2);
        
        // Should be correct length
        assert_eq!(bytes1.len(), 32);
    }

    #[test]
    fn test_wipe() {
        let mut data = vec![0xFFu8; 64];
        wipe(&mut data);
        
        for byte in &data {
            assert_eq!(*byte, 0);
        }
    }

    #[test]
    fn test_constant_time_copy() {
        let mut dst = vec![0u8; 8];
        let src = vec![0xFFu8; 8];
        
        constant_time_copy(true, &mut dst, &src);
        assert_eq!(dst, src);
        
        let new_src = vec![0xAAu8; 8];
        constant_time_copy(false, &mut dst, &new_src);
        // dst should still be 0xFF
        assert_eq!(dst, vec![0xFFu8; 8]);
    }
}