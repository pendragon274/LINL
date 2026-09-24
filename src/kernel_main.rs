use crate::collections::kbox::Box;
use crate::memory::{
    memory_map::MemoryMap,
    heap_allocator::HeapAllocator
};
use crate::vga_display::vga_out_stream::VGAOutStream;
use crate::asm_ops::kernel_info::KernelInfo;
use crate::collections::globals::Globals;

use crate::{print, println};
use crate::asm_ops::multiboot_information::MultibootInformation;

///
/// DONE: Should move the stack to the new spot.
/// DONE: Should pass kernel info to the kernel_main function.
/// DONE: Pass bootloader memory information to kernel.
/// DONE: Implement globals for access to the memory module and VGA output from anywhere.
/// DONE: Color formatting in VGA write_char and write_str. Use \c escape character with raw string literal.
/// DONE: Implement write and format macros for vga output.
/// DONE: Use bootloader memory map to map out available memory.
/// DONE: Should implement a heap.
///
/// TODO: Write an implementation for the interrupt descriptor table.
/// TODO: Make globals thread safe.
/// TODO: Should make the heap allocator better.
/// TODO: Implement a paging manager.
/// TODO: Implement variable that changes kernel virtual address, specifically to higher half.
/// TODO: Write a panic stack unwinding function.
/// TODO: Test heap allocator.
/// TODO: Test collections structures.
/// TODO: Handle CPU interrupts for receiving keyboard input.
///
///

fn box_test(){
    let my_box = Box::from("This is an even much longer str than that.");
    let my_box_len = Box::new(my_box.len());
    //let my_box_deref = *my_box;
    println!("{:?}, {:?}", my_box, my_box_len);
    //Drops here.
}

fn init_globals(kernel_info: &KernelInfo) {
    let mbi = kernel_info.get_mbi();

    Globals::init_memory_map(MemoryMap::from_mbi(&mbi.get_memory_map())).unwrap();

    let heap: HeapAllocator = HeapAllocator::new(Globals::get_memory_map().unwrap(), kernel_info.heap_start, kernel_info.heap_end);
    Globals::init_heap(heap).unwrap();

    let vga_out_stream = VGAOutStream::from_buffer(Globals::get_memory_map().unwrap(), kernel_info.vga_buf_ptr);
    Globals::init_vga_out_stream(vga_out_stream).unwrap();
}

fn mbi_copy_test(mbi: &MultibootInformation) {
    let mbi_box = Box::from(mbi);
    let mbi = mbi_box.as_ref();
    println!("MBI Copy: {:?}", mbi);
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".kernel.text")]
pub extern "C" fn kernel_main(kern_info: &KernelInfo) -> !{
    init_globals(&kern_info);

    emit_pass();

    println!("Kernel main called!");

    box_test();

    mbi_copy_test(kern_info.get_mbi());

    loop{}
}

#[allow(dead_code)]
fn emit_pass(){
    print!(r"\c0f[\c2fPASS\c0f]");
}