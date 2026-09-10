use core::ops::{Index, IndexMut};

#[repr(transparent)]
pub struct RawMemSegment<'a>{
    mem: &'a mut [u8]
}

impl<'a> RawMemSegment<'a>{
    /*pub fn write_at(&mut self, index: usize, to_write: &[u8]){
        let mut i = 0;
        for byte in to_write{
            self.mem[i + index] = byte.clone();
            i += 1;
        }
    }*/

    pub fn write_at(&mut self, index: usize, to_write: &[u8]){
        unsafe { core::ptr::copy_nonoverlapping(to_write.as_ptr(), ((self.mem.as_mut_ptr() as usize) + index) as *mut u8, to_write.len()) ; }
    }

    pub fn clear(&mut self){
        for i in 0..self.mem.len(){
            self.mem[i] = 0;
        }
    }

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