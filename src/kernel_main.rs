use crate::memory::{
    memory_map::MemoryMap,
    heap_allocator::HeapAllocator
};
use crate::vga_display::vga_out_stream::VGAOutStream;
use crate::asm_ops::kernel_info::KernelInfo;
use crate::collections::globals::Globals;
use crate::print;

///
/// DONE: Should move the stack to the new spot.
/// DONE: Should pass kernel info to the kernel_main function.
/// DONE: Pass bootloader memory information to kernel.
/// DONE: Implement globals for access to the memory module and VGA output from anywhere.
/// DONE: Color formatting in VGA write_char and write_str. Use \c escape character with raw string literal.
/// DONE: Implement write and format macros for vga output.
///
/// TODO: Use bootloader memory map to map out available memory.
/// TODO: Make globals thread safe.
/// TODO: Should implement a heap.
/// TODO: Implement a paging manager.
/// TODO: Implement variable that changes kernel virtual address, specifically to higher half.
/// TODO: Write a panic stack unwinding function.
///
///

fn init_globals(kernel_info: &KernelInfo) {
    let globals = Globals::get_mut();
    globals.init_memory_map(MemoryMap::new()).unwrap();

    let heap: HeapAllocator = HeapAllocator::new(globals.get_memory_map().unwrap(), kernel_info.heap_start, kernel_info.heap_end);
    globals.init_heap(heap).unwrap();

    let mbi = kernel_info.get_mbi();

    globals.get_memory_map().unwrap().set_available_memory(&mbi.get_memory_map());

    let vga_out_stream = VGAOutStream::from_buffer(globals.get_memory_map().unwrap(), kernel_info.vga_buf_ptr);
    globals.init_vga_out_stream(vga_out_stream).unwrap();
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".kernel.text")]
pub extern "C" fn kernel_main(kern_info: &KernelInfo) -> !{
    init_globals(&kern_info);

    emit_pass();

    loop{}
}

#[allow(dead_code)]
fn emit_pass(){
    print!(r"\c0f[\c2fPASS\c0f]");
}