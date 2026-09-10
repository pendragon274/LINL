use crate::vga_display::vga_out_stream::{Write, VGAOutStream};
use core::panic::PanicInfo;
use crate::memory::memory_map::MemoryMap;
use crate::vga_display::vga_character::{VGAColorBase, VGAColorCode};

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> !{
    let mut mem_map = MemoryMap::new();
    let mut vga_out_stream = VGAOutStream::from_map(&mut mem_map);
    vga_out_stream.set_color(VGAColorCode::new(VGAColorBase::White, VGAColorBase::Red));
    write!(vga_out_stream, "ERROR: KERNEL PANIC OCCURRED:\n").unwrap();
    vga_out_stream.set_color(VGAColorCode::new(VGAColorBase::White, VGAColorBase::Black));
    write!(vga_out_stream, "{:?}", panic_info).unwrap();
    vga_out_stream.flush();
    loop{}
}