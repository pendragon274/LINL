use crate::asm_ops::multiboot_information::MultibootInformation;
use crate::vga_display::vga_out_buffer::VGAOutBuffer;

#[repr(C)]
pub struct KernelInfo<'a>{
    pub vga_buf_ptr: &'a VGAOutBuffer,
    pub multiboot_information_ptr: *const MultibootInformation
}

impl<'a> KernelInfo<'a> {
    pub fn get_mbi(&self) -> &'a MultibootInformation {
        self.multiboot_information_ptr.into()
    }
}