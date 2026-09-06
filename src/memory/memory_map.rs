use crate::memory::raw_mem_segment::RawMemSegment;

#[repr(transparent)]
pub struct MemoryMap{
}

#[allow(dead_code)]
impl MemoryMap{
    pub unsafe fn own_segment(&mut self, start: usize, len: usize) -> RawMemSegment<'_>{
        unsafe { RawMemSegment::new(start as *mut u8, len) }
    }

    pub fn new() -> MemoryMap{
        MemoryMap{
        }
    }
}