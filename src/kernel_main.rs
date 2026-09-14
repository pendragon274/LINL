use crate::memory::memory_map::MemoryMap;
use crate::vga_display::vga_character::{VGAColorBase, VGAColorCode};
use crate::vga_display::vga_out_stream::{Write, VGAOutStream};
use crate::asm_ops::kernel_info::KernelInfo;

///
/// DONE: Should move the stack to the new spot.
/// DONE: Should pass kernel info to the kernel_main function.
///
/// TODO: Implement thread safe globals for access to the memory module and VGA output from anywhere.
/// TODO: Pass bootloader memory information to kernel.
/// TODO: Implement write and format macros for vga output.
/// TODO: Should implement a heap.
/// TODO: Implement a paging manager.
/// TODO: Implement variable that changes kernel virtual address, specifically to higher half.
/// TODO: Write a panic stack unwinding function.
/// TODO: Color formatting in VGA write_char and write_str. Use \c escape character with raw string literal.
///
///

//static blah: OnceCell<Mutex<MemoryMap>> = OnceLock<MemoryMap>;

#[unsafe(no_mangle)]
#[unsafe(link_section = ".kernel.text")]
pub extern "C" fn kernel_main(kern_info: &KernelInfo) -> !{
    let mbi = kern_info.get_mbi();

    let mut mem_map = MemoryMap::new();
    
    let mut vga_out_stream = VGAOutStream::from_buffer(&mut mem_map, kern_info.vga_buf_ptr);
    emit_pass(&mut vga_out_stream);

    /*write!(vga_out_stream, "Command Line: {:?}\n", mbi.get_command_line()).unwrap();
    write!(vga_out_stream, "Boot Loader Name: {:?}\n", mbi.get_boot_loader_name()).unwrap();
    */

    write!(vga_out_stream, "Memory Map: {:?}\n", mbi.get_memory_map()).unwrap();

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