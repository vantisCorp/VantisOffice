//! Memory management utilities

use std::ptr;

/// Zero-copy memory buffer
pub struct ZeroCopyBuffer {
    ptr: *mut u8,
    len: usize,
}

impl ZeroCopyBuffer {
    /// Create a new zero-copy buffer
    pub fn new(len: usize) -> anyhow::Result<Self> {
        let ptr = unsafe {
            std::alloc::alloc(std::alloc::Layout::from_size_align(len, 8)?)
        };
        
        if ptr.is_null() {
            anyhow::bail!("Failed to allocate memory");
        }
        
        Ok(ZeroCopyBuffer { ptr, len })
    }

    /// Get pointer to data
    pub fn as_ptr(&self) -> *const u8 {
        self.ptr
    }

    /// Get mutable pointer to data
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.ptr
    }

    /// Get length
    pub fn len(&self) -> usize {
        self.len
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl Drop for ZeroCopyBuffer {
    fn drop(&mut self) {
        unsafe {
            std::alloc::dealloc(
                self.ptr,
                std::alloc::Layout::from_size_align_unchecked(self.len, 8),
            );
        }
    }
}

unsafe impl Send for ZeroCopyBuffer {}
unsafe impl Sync for ZeroCopyBuffer {}
