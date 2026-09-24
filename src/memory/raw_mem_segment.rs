use core::ops::{Index, IndexMut};
use crate::collections::globals::Globals;

#[repr(transparent)]
pub struct RawMemSegment<'a>{
    mem: &'a mut [u8]
}

impl<'a> RawMemSegment<'a>{
    // ***** Public Functions *****
    pub fn write_at(&mut self, index: usize, to_write: &[u8]){
        unsafe { core::ptr::copy_nonoverlapping(to_write.as_ptr(), ((self.mem.as_mut_ptr() as usize) + index) as *mut u8, to_write.len()) ; }
    }

    pub fn clear(&mut self){
        for i in 0..self.mem.len(){
            self.mem[i] = 0;
        }
    }
    
    pub fn addr(&self) -> usize{
        self.mem.as_ptr() as usize
    }
    
    pub fn len(&self) -> usize{
        self.mem.len()
    }

    // ***** Struct Init *****
    pub unsafe fn new<'b>(start: *mut u8, len: usize) -> RawMemSegment<'b>{
        unsafe {
            RawMemSegment{
                mem: core::slice::from_raw_parts_mut(start, len)
            }
        }
    }
}

impl Index<usize> for RawMemSegment<'_>{
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.mem[index]
    }
}

impl IndexMut<usize> for RawMemSegment<'_>{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.mem[index]
    }
}

impl Drop for RawMemSegment<'_>{
    fn drop(&mut self){
        Globals::get_memory_map().unwrap().return_segment(self);
    }
}