use core::alloc::Layout;
use core::error::Error;
use core::fmt::{Debug, Display, Formatter};
use crate::memory::memory_map::MemoryMap;
use crate::memory::raw_mem_segment::RawMemSegment;
use crate::println;

#[allow(dead_code)]
pub struct HeapAllocator<'a>{
    mem: RawMemSegment<'a>,
    cursor: *mut u8
}

impl<'a> HeapAllocator<'a>{
    // ***** Public Functions *****
    /*pub fn malloc<'b>(&mut self, size: NonZeroUsize) -> Result<&'b mut [u8], HeapAllocatorError>{
        let ptr = ptr::from_raw_parts_mut(self.cursor, size.get());
        self.cursor = (self.cursor as usize + size.get()) as *mut u8;

        println!("Allocating size {}", size.get());

        unsafe { Ok(&mut *ptr) }
    }

    pub fn demalloc(&mut self, ptr: &mut [u8]){
        println!("Deallocating ptr {}", ptr.len());
    }*/

    pub fn alloc(&mut self, layout: Layout) -> *mut u8{
        println!("Allocating size {}", layout.size());

        let ptr = self.cursor;
        self.cursor = (self.cursor as usize + layout.size()) as *mut u8;

        ptr
    }

    pub fn dealloc(&mut self, ptr: *mut u8, layout: Layout){
        println!("Deallocating ptr {:x} with size {}", ptr as usize, layout.size());
    }

    // ***** Struct Init *****
    pub fn new<'b, 'c>(mem_map: &'c mut MemoryMap, start: *const u8, end: *const u8) -> HeapAllocator<'b>{
        let len: usize = end as usize - start as usize;

        HeapAllocator{
            mem: mem_map.borrow_segment(start, len),
            cursor: start as *mut u8
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct HeapAllocatorError{
    code: u8,
    msg: Option<&'static str>
}

#[allow(dead_code)]
impl HeapAllocatorError{
    pub fn from_code<T>(code: u8) -> Result<T, HeapAllocatorError>{
        Err(
            HeapAllocatorError{
                code,
                msg: None
            }
        )
    }

    pub fn new<T>(code: u8, new_msg: &'static str) -> Result<T, HeapAllocatorError>{
        Err(
            HeapAllocatorError{
                code,
                msg: Some(new_msg)
            }
        )
    }
}

impl Display for HeapAllocatorError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}

impl Error for HeapAllocatorError{}