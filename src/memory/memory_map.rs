use core::ffi::c_void;
use crate::collections::linked_array::LinkedArray;
use crate::memory::raw_mem_segment::RawMemSegment;

/*#[repr(transparent)]
pub struct MemoryMap<'a>{
    my_segment: [RawMemSegment<'a>; 10]
}

#[allow(dead_code)]
impl<'a> MemoryMap<'a>{
    pub unsafe fn own_segment(&mut self, start: usize, len: usize) -> RawMemSegment<'_>{
        unsafe { RawMemSegment::new(start as *mut u8, len) }
    }

    pub unsafe fn borrow_segment(&mut self, start: usize, len: usize) -> &'a mut RawMemSegment<'a> {
        let segment = RawMemSegment::new(start as *mut u8, len);
        self.
        self.my_segment = &mut RawMemSegment::new(start as *mut u8, len);
        unsafe { &mut *self.my_segment }
    }

    pub fn new<'b>() -> MemoryMap<'b>{
        MemoryMap{
            my_segment: 0 as *mut RawMemSegment
        }
    }
}*/

/*pub struct MemoryMap<'a>{
    segments: LinkedArray<RawMemSegment<'a>, 10>
}

impl<'a> MemoryMap<'a> {
    pub fn new() -> MemoryMap<'a> {
        MemoryMap{
            segments: LinkedArray::new()
        }
    }
}*/

#[allow(dead_code)]
pub struct MemoryMap{
    segments_reserved: LinkedArray<(*const c_void, *const c_void), 10>
}

impl MemoryMap{
    pub fn borrow_segment<'a>(&mut self, start: usize, len: usize) -> RawMemSegment<'a>{
        //Should check to ensure memory slot is currently unreserved.
        //Should also "reserve" the segment if it is available.

        unsafe {
            RawMemSegment::new(start as *mut u8, len)
        }
    }

    pub fn new() -> MemoryMap{
        MemoryMap{
            segments_reserved: LinkedArray::new()
        }
    }
}