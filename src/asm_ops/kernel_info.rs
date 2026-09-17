use crate::asm_ops::multiboot_information::MultibootInformation;
use crate::vga_display::vga_out_buffer::VGAOutBuffer;

#[repr(C)]
pub struct KernelInfo<'a>{
    pub vga_buf_ptr: &'a VGAOutBuffer,
    pub multiboot_information_ptr: *const u32,
    pub stack_bottom: *const u8,
    pub stack_top: *const u8,
    pub heap_start: *const u8,
    pub heap_end: *const u8
}

impl<'a> KernelInfo<'a> {
    pub fn get_mbi(&self) -> &'a MultibootInformation {
        unsafe {
            let size: u32 = *(self.multiboot_information_ptr);
            &*(core::ptr::from_raw_parts(self.multiboot_information_ptr, size as usize))
        }
    }
}