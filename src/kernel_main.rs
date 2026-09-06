use crate::memory::memory_map::MemoryMap;
use crate::vga_display::vga_character::{VGAColorBase, VGAColorCode};
use crate::vga_display::vga_out_stream::VGAOutStream;
use crate::asm_ops::kernel_info::KernelInfo;

///
/// Should move the stack to the new spot.
/// Should pass kernel info to the kernel_main function.
///

#[unsafe(no_mangle)]
#[unsafe(link_section = ".kernel.text")]
pub extern "C" fn kernel_main(kern_info: &KernelInfo) -> !{
    let mut mem_map = MemoryMap::new();
    let mut segment = unsafe{
        mem_map.own_segment(0xb8000, 160 * 25)
    };

    let mut vga_out_stream = VGAOutStream::with_segment(&mut segment);
    vga_out_stream.set_color(VGAColorCode::new(VGAColorBase::White, VGAColorBase::Green));
    vga_out_stream.write_hex((kern_info as *const KernelInfo) as u64);
    vga_out_stream.write_char('\n');

    //vga_out_stream.write_hex(kern_info as u64);
    //vga_out_stream.write_char('\n');
    unsafe {
        vga_out_stream.write_hex(kern_info.magic_number as u64);
        vga_out_stream.write_char('\n');
        vga_out_stream.write_hex(kern_info.vga_buf_ptr as u64);
        vga_out_stream.write_char('\n');
        vga_out_stream.write_hex(kern_info.vga_buf_len as u64);
        vga_out_stream.write_char('\n');
    }

    vga_out_stream.flush_buffer();

    loop{}
}