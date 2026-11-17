#![allow(dead_code)]

use std::alloc::{AllocError, Allocator, Global, Layout};
use std::ptr::NonNull;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Allocator which forwards to `Global` and counts the allocations it serves.
#[derive(Clone, Default)]
pub struct TestAlloc(Arc<AtomicUsize>);

impl TestAlloc {
    /// Returns the number of allocations served by this allocator and its
    /// clones.
    pub fn allocations(&self) -> usize {
        self.0.load(Ordering::Relaxed)
    }
}

// SAFETY: every block is allocated and freed by `Global`, and all clones share
// the counter only, so they can free the memory of each other.
unsafe impl Allocator for TestAlloc {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        self.0.fetch_add(1, Ordering::Relaxed);
        Global.allocate(layout)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        // SAFETY: the caller guarantees that `ptr` was returned by `allocate`
        // with the same `layout`, and `allocate` forwards to `Global`.
        unsafe { Global.deallocate(ptr, layout) }
    }
}
