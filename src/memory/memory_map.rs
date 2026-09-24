use crate::asm_ops::multiboot_memory_map::MultibootMemoryMap;
use crate::collections::linked_array::{BasicCollection, LinkedArray};
use crate::memory::raw_mem_segment::RawMemSegment;
use crate::println;

const ARR_SIZE: usize = 30;

#[allow(dead_code)]
#[derive(Debug)]
pub struct MemoryMap{
    segments_available: LinkedArray<(*const u8, usize), ARR_SIZE>,
    segments_reserved: LinkedArray<(*const u8, usize), ARR_SIZE>
}

impl MemoryMap{
    // ***** Public Functions *****
    pub fn borrow_segment<'a, 'b>(&'b mut self, start: *const u8, len: usize) -> RawMemSegment<'a>{
        //Should check to ensure memory slot is currently unreserved.
        //Should also "reserve" the segment if it is available.

        println!("Segment borrowed: Address: {:x}, Length: {:x}", start as usize, len);

        unsafe {
            RawMemSegment::new(start as *mut u8, len)
        }
    }

    pub fn return_segment<'a>(&mut self, segment: &mut RawMemSegment<'a>){
        println!("Segment returned: Address: {}, Length: {}", segment.addr(), segment.len());
    }

    /*
    pub fn set_available_memory(&mut self, _multiboot_memory_map: &MultibootMemoryMap){

    }*/

    // ***** Private Functions *****

    // ***** Struct Init *****
    pub fn from_mbi(mbi: &MultibootMemoryMap) -> MemoryMap{
        let mut available: LinkedArray<(*const u8, usize), ARR_SIZE> = LinkedArray::new();
        for item in mbi.into_iter(){
            if item.mem_type() == 1{
                available.append((item.base_addr() as *const u8, item.length() as usize));
            }
        }
        MemoryMap{
            segments_available: available,
            segments_reserved: LinkedArray::new()
        }
    }

    /*
    pub fn new() -> MemoryMap{
        MemoryMap{
            segments_available: LinkedArray::new(),
            segments_reserved: LinkedArray::new()
        }
    }*/
}