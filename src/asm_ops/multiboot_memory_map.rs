use core::fmt::{Debug, Formatter};
use crate::asm_ops::multiboot_information::Tag;

pub struct MultibootMemoryMap<'a>{
    entry_size: u32,
    entry_version: u32,
    entries: &'a [u8]
}

impl<'a> MultibootMemoryMap<'a>{
    pub fn from_tag(tag: &Tag<'a>) -> MultibootMemoryMap<'a>{
        let tag_data = tag.data();

        if tag_data.len() >= 8{
            MultibootMemoryMap{
                entry_size: u32::from_ne_bytes(tag_data[0..4].try_into().unwrap()),
                entry_version: u32::from_ne_bytes(tag_data[4..8].try_into().unwrap()),
                entries: &tag_data[8..]
            }
        }else{
            MultibootMemoryMap::default()
        }
    }
}

impl<'a> Default for MultibootMemoryMap<'a>{
    fn default() -> MultibootMemoryMap<'a>{
        MultibootMemoryMap{
            entry_size: 8,
            entry_version: 0,
            entries: &[]
        }
    }
}

impl<'a> Debug for MultibootMemoryMap<'a>{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "MBootMemMap {{Entry Size: {}, Entry Version: {}, Entries: {{", self.entry_size, self.entry_version)?;

        for entry in self.into_iter(){
            write!(f, "{:?}, ", entry)?;
        }

        write!(f, "}}}}")?;
        Ok(())
    }
}

impl<'a> IntoIterator for &'a MultibootMemoryMap<'a>{
    type Item = MultibootMemoryMapEntry<'a>;
    type IntoIter = MultibootMemoryMapIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MultibootMemoryMapIterator::from(self)
    }
}

// ***** MultibootMemoryMapIterator *****

pub struct MultibootMemoryMapIterator<'a>{
    current: Option<MultibootMemoryMapEntry<'a>>
}

impl<'a> MultibootMemoryMapIterator<'a>{
    pub fn from(mem_map: &'a MultibootMemoryMap<'a>) -> MultibootMemoryMapIterator<'a>{
        MultibootMemoryMapIterator{
            current: MultibootMemoryMapEntry::from_slice(mem_map.entries)
        }
    }
}


impl<'a> Iterator for MultibootMemoryMapIterator<'a>{
    type Item = MultibootMemoryMapEntry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.current.take();
        let entry: MultibootMemoryMapEntry;

        if ret.is_none(){
            return None;
        }else{
            entry = ret.unwrap();
        }

        self.current = MultibootMemoryMapEntry::from_slice(&entry.remaining_data[0..]);

        Some(entry)
    }
}

// ***** MultibootMemoryMapEntry *****

pub struct MultibootMemoryMapEntry<'a>{
    base_addr: u64,
    length: u64,
    mem_type: u32,
    remaining_data: &'a [u8]
}

impl<'a> MultibootMemoryMapEntry<'a>{
    pub fn mem_type(&self) -> u32{
        self.mem_type
    }

    pub fn base_addr(&self) -> u64{
        self.base_addr
    }

    pub fn length(&self) -> u64{
        self.length
    }

    pub fn from_slice(data: &'a [u8]) -> Option<MultibootMemoryMapEntry<'a>>{
        if data.len() < 24{
            None
        }else{
            let base_addr: u64 = u64::from_ne_bytes(data[0..8].try_into().unwrap());
            let length: u64 = u64::from_ne_bytes(data[8..16].try_into().unwrap());
            let mem_type: u32 = u32::from_ne_bytes(data[16..20].try_into().unwrap());

            Some(MultibootMemoryMapEntry{
                base_addr,
                length,
                mem_type,
                remaining_data: &data[24..]
            })
        }
    }
}

impl<'a> Debug for MultibootMemoryMapEntry<'a>{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Type {} [0x{:x} - 0x{:x}]", self.mem_type, self.base_addr, self.base_addr + self.length)?;

        Ok(())
    }
}