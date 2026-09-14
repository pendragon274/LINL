use crate::asm_ops::multiboot_information::Tag;

#[derive(Debug)]
pub struct NetworkInfo<'a>{
    dhcp_ack: &'a [u8]
}

impl<'a> NetworkInfo<'a>{
    pub fn from_tag(tag: &Tag<'a>) -> NetworkInfo<'a>{
        NetworkInfo{
            dhcp_ack: tag.data()
        }
    }
}

impl<'a> Default for NetworkInfo<'a>{
    fn default() -> Self {
        NetworkInfo{
            dhcp_ack: &[]
        }
    }
}