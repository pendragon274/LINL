#[repr(C)]
pub struct KernelInfo{
    pub magic_number: u64,
    pub vga_buf_ptr: *mut u8,
    pub vga_buf_len: u32
}