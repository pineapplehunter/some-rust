use core::ptr::NonNull;

use linked_list_allocator::Heap;

use crate::sync::Mutex;

pub(crate) struct CustomLockedHeap(Mutex<Heap>);

impl core::ops::Deref for CustomLockedHeap {
    type Target = Mutex<Heap>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

unsafe impl alloc::alloc::GlobalAlloc for CustomLockedHeap {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        self.lock().allocate_first_fit(layout).unwrap().as_ptr()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        self.lock().deallocate(NonNull::new(ptr).unwrap(), layout)
    }
}

impl CustomLockedHeap {
    pub const fn empty() -> Self {
        Self(Mutex::new(Heap::empty()))
    }
}
