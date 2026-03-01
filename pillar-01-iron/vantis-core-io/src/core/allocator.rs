//! Custom memory allocator for Vantis-Core-IO

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Custom allocator that tracks memory usage
pub struct VantisAllocator {
    system: System,
    allocated: AtomicUsize,
}

unsafe impl GlobalAlloc for VantisAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = self.system.alloc(layout);
        if !ptr.is_null() {
            self.allocated.fetch_add(layout.size(), Ordering::SeqCst);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.system.dealloc(ptr, layout);
        self.allocated.fetch_sub(layout.size(), Ordering::SeqCst);
    }
}

impl VantisAllocator {
    pub const fn new() -> Self {
        VantisAllocator {
            system: System,
            allocated: AtomicUsize::new(0),
        }
    }

    pub fn get_allocated(&self) -> usize {
        self.allocated.load(Ordering::SeqCst)
    }
}

/// Initialize the custom allocator
pub fn init() -> anyhow::Result<()> {
    // Allocator would be set up here
    Ok(())
}
