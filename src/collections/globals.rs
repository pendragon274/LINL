use core::cell::OnceCell;
use crate::memory::heap_allocator::HeapAllocator;
use crate::memory::memory_map::MemoryMap;
use crate::vga_display::vga_out_stream::VGAOutStream;

static mut HEAP_GLOBAL: OnceCell<HeapAllocator> = OnceCell::new();
static mut MEMORY_MAP_GLOBAL: OnceCell<MemoryMap> = OnceCell::new();
static mut VGA_OUT_GLOBAL: OnceCell<VGAOutStream> = OnceCell::new();

pub struct Globals;

#[allow(dead_code)]
impl Globals {
    // ***** Public Functions *****
    pub fn init_heap(heap: HeapAllocator<'static>) -> Result<(), ()>{
        unsafe {
            match HEAP_GLOBAL.set(heap) {
                Ok(_) => Ok(()),
                Err(_) => Err(())
            }
        }
    }

    pub fn init_memory_map(mem_map: MemoryMap) -> Result<(), ()>{
        unsafe {
            match MEMORY_MAP_GLOBAL.set(mem_map) {
                Ok(_) => Ok(()),
                Err(_) => Err(())
            }
        }
    }

    pub fn init_vga_out_stream(vga_out: VGAOutStream<'static>) -> Result<(), ()>{
        unsafe {
            match VGA_OUT_GLOBAL.set(vga_out) {
                Ok(_) => Ok(()),
                Err(_) => Err(())
            }
        }
    }

    pub fn get_heap<'a>() -> Option<&'a mut HeapAllocator<'static>> {
        unsafe {
            HEAP_GLOBAL.get_mut()
        }
    }

    pub fn get_memory_map<'a>() -> Option<&'a mut MemoryMap> {
        unsafe {
            MEMORY_MAP_GLOBAL.get_mut()
        }
    }

    pub fn get_vga_out_stream<'a>() -> Option<&'a mut VGAOutStream<'static>> {
        unsafe {
            VGA_OUT_GLOBAL.get_mut()
        }
    }
}