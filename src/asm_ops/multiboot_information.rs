use core::fmt::{Debug, Formatter};
use core::ptr;
use crate::asm_ops::multiboot_apm_table::APMTable;
use crate::asm_ops::multiboot_network_info::NetworkInfo;
use crate::asm_ops::multiboot_memory_map::MultibootMemoryMap;
use crate::collections::str_debug_format::StrDebugFormat;

#[repr(C, align(8))]
pub struct MultibootInformation {
    total_size: u32,
    reserved: u32,
    data: [u8]
}

impl MultibootInformation {
    pub fn get_command_line(&self) -> StrDebugFormat<'_> {
        for tag in self.into_iter(){
            match tag.tag_type{
                TagType::CommandLine => return StrDebugFormat::from_cstr(tag.current_data),
                _ => continue
            }
        }

        StrDebugFormat::from_cstr(&[])
    }

    pub fn get_boot_loader_name(&self) -> StrDebugFormat<'_>{
        for tag in self.into_iter(){
            match tag.tag_type{
                TagType::BootLoaderName => return StrDebugFormat::from_cstr(tag.current_data),
                _ => continue
            }
        }

        StrDebugFormat::from_cstr(&[])
    }

    pub fn get_memory_map(&self) -> MultibootMemoryMap<'_> {
        let mut mem_map: MultibootMemoryMap = MultibootMemoryMap::default();
        for tag in self.into_iter(){
            match tag.tag_type{
                TagType::MemoryMap => {
                    mem_map = MultibootMemoryMap::from_tag(&tag);
                    break;
                }, _ => continue
            }
        }

        mem_map
    }

    pub fn get_apm_table(&self) -> APMTable{
        let mut apm_table: APMTable = APMTable::default();
        for tag in self.into_iter(){
            match tag.tag_type{
                TagType::APMTable => {
                    apm_table = APMTable::from_tag(&tag);
                }, _ => continue
            }
        }

        apm_table
    }

    pub fn get_network_info(&self) -> NetworkInfo<'_>{
        let mut network_info = NetworkInfo::default();
        for tag in self.into_iter(){
            match tag.tag_type{
                TagType::NetworkInfo => {
                    network_info = NetworkInfo::from_tag(&tag);
                }, _ => continue
            }
        }

        network_info
    }

    pub fn get_img_load_addr(&self) -> u32{
        let mut ret = 0;
        for tag in self.into_iter(){
            match tag.tag_type{
                TagType::IMGBaseAddr => {
                    ret = u32::from_ne_bytes(tag.data()[0..4].try_into().unwrap());
                }, _ => continue
            }
        }

        ret
    }
}

impl<'a> From<*const MultibootInformation> for &'a MultibootInformation {
    fn from(value: *const MultibootInformation) -> &'a MultibootInformation {
        unsafe {
            let size = (*value).total_size;
            let mbi: *const MultibootInformation = ptr::from_raw_parts(value as *const (), size as usize);
            &*mbi
        }
    }
}

impl<'a> IntoIterator for &'a MultibootInformation {
    type Item = Tag<'a>;
    type IntoIter = MultibootInformationIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MultibootInformationIterator::from(self)
    }
}

impl Debug for MultibootInformation {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "MBI (Total Size: {}, Data Len: {}){{", self.total_size, self.data.len())?;

        for tag in self.into_iter() {
            write!(f, "{:?}, ", tag)?;
        }

        write!(f, "}}")?;

        Ok(())
    }
}

// ***** MultibootInformationIterator *****

pub struct MultibootInformationIterator<'a>{
    current: Option<Tag<'a>>
}

impl<'a> MultibootInformationIterator<'a>{
    pub fn from(info: &'a MultibootInformation) -> MultibootInformationIterator<'a> {
        MultibootInformationIterator{
            current: Tag::from_slice(&info.data)
        }
    }

    pub fn has_next(&self) -> bool{
        match self.current{
            Some(_) => true,
            None => false
        }
    }
}

impl<'a> Iterator for MultibootInformationIterator<'a> {
    type Item = Tag<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.current.take();
        let tag: Tag;

        if ret.is_none(){
            return None;
        }else {
            tag = ret.unwrap();
        }

        self.current = Tag::from_slice(&tag.remaining_data[0..]);

        Some(tag)
    }
}

// ***** Tag *****

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum TagType{
    End = 0,
    CommandLine = 1,
    BootLoaderName = 2,
    MemoryMap = 6,
    APMTable = 10,
    NetworkInfo = 16,
    IMGBaseAddr = 21,
    Unknown = 99
}

impl From<u32> for TagType{
    fn from(value: u32) -> Self {
        match value {
            0 => TagType::End,
            1 => TagType::CommandLine,
            2 => TagType::BootLoaderName,
            6 => TagType::MemoryMap,
            10 => TagType::APMTable,
            16 => TagType::NetworkInfo,
            21 => TagType::IMGBaseAddr,
            _ => TagType::Unknown
        }
    }
}

pub struct Tag<'a>{
    tag_type: TagType,
    current_data: &'a [u8],
    remaining_data: &'a [u8]
}

impl<'a> Tag<'a> {
    pub fn from_slice(data: &'a [u8]) -> Option<Tag<'a>> {
        if data.len() < 8 {
            None
        }else{
            let tag_type: u32 = u32::from_ne_bytes(data[0..4].try_into().unwrap());
            let tag_size: u32 = u32::from_ne_bytes(data[4..8].try_into().unwrap());
            let padding: u32 = 8 - tag_size % 8;

            if data.len() <= (padding + tag_size) as usize {
                None
            }else{
                Some(Tag{
                    tag_type: TagType::from(tag_type),
                    current_data: &data[8..(tag_size as usize)],
                    remaining_data: &data[((padding + tag_size) as usize)..]
                })
            }
        }
    }


    pub fn data(&self) -> &'a [u8]{
        self.current_data
    }
}

impl<'a> Debug for Tag<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self.tag_type {
            TagType::End => write!(f, "End"),
            TagType::CommandLine => write!(f, "Command Line '{:?}'", StrDebugFormat::from_cstr(self.current_data)),
            TagType::BootLoaderName => write!(f, "Boot Loader '{:?}'", StrDebugFormat::from_cstr(self.current_data)),
            TagType::MemoryMap => write!(f, "Memory Map ({} bytes)", self.current_data.len()),
            TagType::APMTable => write!(f, "APMTable ({} bytes)", self.current_data.len()),
            TagType::NetworkInfo => write!(f, "Network Info ({} bytes)", self.current_data.len()),
            TagType::IMGBaseAddr => write!(f,"IMG Load Base Addr: {:?}", StrDebugFormat::from_u32(self.current_data)),
            _ => write!(f,"Unknown Tag {:?}", self.tag_type)
        }
    }
}