use crate::memory::{
    memory_map::MemoryMap,
    heap_allocator::HeapAllocator
};
use crate::vga_display::vga_character::{VGAColorBase, VGAColorCode};
use crate::vga_display::vga_out_stream::{Write, VGAOutStream};
use crate::asm_ops::kernel_info::KernelInfo;
use crate::collections::globals::Globals;

///
/// DONE: Should move the stack to the new spot.
/// DONE: Should pass kernel info to the kernel_main function.
/// DONE: Pass bootloader memory information to kernel.
/// DONE: Implement globals for access to the memory module and VGA output from anywhere.
///
/// TODO: Use bootloader memory map to map out available memory.
/// TODO: Make globals thread safe.
/// TODO: Implement write and format macros for vga output.
/// TODO: Should implement a heap.
/// TODO: Implement a paging manager.
/// TODO: Implement variable that changes kernel virtual address, specifically to higher half.
/// TODO: Write a panic stack unwinding function.
/// TODO: Color formatting in VGA write_char and write_str. Use \c escape character with raw string literal.
///
///

fn init_kernel<'a>(kernel_info: &KernelInfo) -> &'a mut Globals {
    let globals = Globals::get();
    globals.init_memory_map(MemoryMap::new()).unwrap();

    let mut mem_map = globals.get_memory_map().unwrap();
    let heap: HeapAllocator = HeapAllocator::new(mem_map, kernel_info.heap_start, kernel_info.heap_end);
    Globals::get().init_heap(heap).unwrap();            // TODO: I think this is cheating using unsafe behavior outside of unsafe.

    let mbi = kernel_info.get_mbi();

    mem_map.set_available_memory(&mbi.get_memory_map());

    let vga_out_stream = VGAOutStream::from_buffer(&mut mem_map, kernel_info.vga_buf_ptr);
    Globals::get().init_vga_out_stream(vga_out_stream).unwrap();

    globals
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".kernel.text")]
pub extern "C" fn kernel_main(kern_info: &KernelInfo) -> !{
    let globals = init_kernel(&kern_info);
    let vga_out_stream = globals.get_vga_out_stream().unwrap();

    emit_pass(vga_out_stream);

    write!(vga_out_stream, "Hey, this works!\n").unwrap();

    /*write!(vga_out_stream, "Command Line: {:?}\n", mbi.get_command_line()).unwrap();
    write!(vga_out_stream, "Boot Loader Name: {:?}\n", mbi.get_boot_loader_name()).unwrap();
    */

    //write!(vga_out_stream, "Memory Map: {:?}\n", mbi.get_memory_map()).unwrap();

    /*write!(vga_out_stream, "APMTable: {:?}\n", mbi.get_apm_table()).unwrap();
    write!(vga_out_stream, "Network Info: {:?}\n", mbi.get_network_info()).unwrap();
    write!(vga_out_stream, "IMG Load Base Addr: {:?}\n", mbi.get_img_load_addr()).unwrap();*/

    //write!(vga_out_stream, "{:?}", mbi).unwrap();

    vga_out_stream.flush();

    loop{}
}

#[allow(dead_code)]
fn emit_pass(vga_out: &mut VGAOutStream){
    let white = VGAColorCode::new(VGAColorBase::White, VGAColorBase::Black);
    let green = VGAColorCode::new(VGAColorBase::White, VGAColorBase::Green);
    vga_out.set_color(white);
    vga_out.write_char('[');
    vga_out.set_color(green);
    vga_out.write_string("PASS");
    vga_out.set_color(white);
    vga_out.write_char(']');
    vga_out.flush();
}