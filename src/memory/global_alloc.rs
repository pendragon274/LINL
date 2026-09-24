pub use core::alloc::{GlobalAlloc, Layout};
use crate::collections::globals::Globals;

#[global_allocator]
pub static mut GLOBAL_ALLOC: GlobalAllocator = GlobalAllocator{};

pub struct GlobalAllocator;

unsafe impl GlobalAlloc for GlobalAllocator{
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        Globals::get_heap().unwrap().alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        Globals::get_heap().unwrap().dealloc(ptr, layout)
    }
}