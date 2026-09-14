use crate::asm_ops::multiboot_information::Tag;

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct APMTable{
    version: u16,
    cseg: u16,
    offset: u32,
    cseg_16: u16,
    dseg: u16,
    flags: u16,
    cseg_len: u16,
    cseg_16_len: u16,
    dseg_len: u16
}

impl APMTable{
    pub fn from_tag<'a>(tag: &'a Tag<'a>) -> APMTable{
        let data = tag.data();
        if data.len() < 20{
            return APMTable::default();
        }
        
        let casted = unsafe { *((data.as_ptr()) as *const APMTable) };
        casted.clone()
    }
}