use core::error::Error;
use core::fmt::{Debug, Display, Formatter};
use core::num::NonZeroUsize;
use crate::memory::memory_map::MemoryMap;
use crate::memory::raw_mem_segment::RawMemSegment;

pub struct HeapAllocator<'a>{
    mem: RawMemSegment<'a>
}

impl<'a> HeapAllocator<'a>{
    // ***** Public Functions *****
    /*pub fn kmalloc<'b>(size: NonZeroUsize) -> Result<&'b mut [u8], HeapAllocatorError>{
        HeapAllocatorError::new(0, "Not implemented yet.")
    }*/

    // ***** Struct Init *****
    pub fn new<'b, 'c>(mem_map: &'c mut MemoryMap, start: *const u8, end: *const u8) -> HeapAllocator<'b>{
        let len: usize = end as usize - start as usize;
        
        HeapAllocator{
            mem: mem_map.borrow_segment(start, len)
        }
    }
}

#[derive(Debug)]
pub struct HeapAllocatorError{
    code: u8,
    msg: Option<&'static str>
}

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
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}

impl Error for HeapAllocatorError{}