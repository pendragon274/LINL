use core::cell::OnceCell;
use crate::memory::heap_allocator::HeapAllocator;
use crate::memory::memory_map::MemoryMap;
use crate::vga_display::vga_out_stream::VGAOutStream;

static mut GLOBALS: OnceCell<Globals> = OnceCell::new();

pub struct Globals{
    heap: OnceCell<HeapAllocator<'static>>,
    memory_map: OnceCell<MemoryMap>,
    vga_out_stream: OnceCell<VGAOutStream<'static>>,
}

#[allow(dead_code)]
impl Globals{
    // ***** Public Functions *****
    pub fn init_memory_map(&mut self, mem_map: MemoryMap) -> Result<(), ()>{
        match self.memory_map.set(mem_map){
            Ok(()) => Ok(()),
            Err(_) => Err(())
        }
    }

    pub fn init_heap(&mut self, heap: HeapAllocator<'static>) -> Result<(), ()>{
        match self.heap.set(heap){
            Ok(()) => Ok(()),
            Err(_) => Err(())
        }
    }

    pub fn init_vga_out_stream(&mut self, vga_out_stream: VGAOutStream<'static>) -> Result<(), ()>{
        match self.vga_out_stream.set(vga_out_stream){
            Ok(()) => Ok(()),
            Err(_) => Err(())
        }
    }

    pub fn get_memory_map(&mut self) -> Option<&mut MemoryMap> {
        self.memory_map.get_mut()
    }

    pub fn get_heap(&mut self) -> Option<&mut HeapAllocator<'static>> {
        self.heap.get_mut()
    }

    pub fn get_vga_out_stream(&mut self) -> Option<&mut VGAOutStream<'static>> {
        self.vga_out_stream.get_mut()
    }

    // ***** Private Functions *****
    fn init() -> Globals{
        Globals{
            heap: OnceCell::new(),
            memory_map: OnceCell::new(),
            vga_out_stream: OnceCell::new()
        }
    }

    // ***** Struct Init *****
    pub fn get<'a>() -> &'a Globals{
        unsafe { GLOBALS.get_or_init(Globals::init) }
    }

    pub fn get_mut<'a>() -> &'a mut Globals{
        // TODO: Make this safe.
        // It is currently very much unsafe.
        unsafe { GLOBALS.get_mut_or_init(Globals::init) }
    }
}

/*pub struct GlobalError{

}

impl Debug for GlobalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}

impl Display for GlobalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}

impl Error for GlobalError{

}*/