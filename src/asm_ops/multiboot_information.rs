use core::fmt::{Debug, Formatter};

#[allow(dead_code)]
pub struct MultibootInformationStorage{
    information: MultibootInformation
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MultibootInformation{
    flags: u32,                 //0

    mem_lower: u32,             //4
    mem_upper: u32,             //8

    boot_device: u32,           //12

    cmdline: u32,               //16

    mods_count: u32,            //20
    mods_addr: u32,             //24

    syms: Syms,                 //28 - 40

    mmap_length: u32,           //44
    mmap_addr: u32,             //48

    drives_length: u32,         //52
    drives_addr: u32,           //56

    config_table: u32,          //60

    boot_loader_name: u32,      //64

    apm_table: u32,             //68

    vbe_control_info: u32,      //72
    vbe_mode_info: u32,         //76
    vbe_mode: u16,              //80
    vbe_interface_seg: u16,     //82
    vbe_interface_off: u16,     //84
    vbe_interface_len: u16,     //86

    framebuffer_addr: u64,      //88
    framebuffer_pitch: u32,     //96
    framebuffer_width: u32,     //100
    framebuffer_height: u32,    //104
    framebuffer_bpp: u8,        //108
    framebuffer_type: u8,       //109
    color_info1: u16,           //110
    color_info2: u16,           //112
    color_info3: u16,           //114
}

impl MultibootInformation {
    #[allow(dead_code)]
    pub fn deep_clone(&self) -> MultibootInformationStorage {
        todo!()
    }
}

impl Debug for MultibootInformation {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "MBI:{{")?;

        let (mem_valid, boot_device_valid, cmdline_valid, mods_valid,
            sym_opt1, sym_opt2, mmap_valid, drives_valid,
            config_table_valid, boot_loader_name_valid, apm_table_valid, vbe_table_valid,
            framebuffer_valid) =
            (((1 << 0) & self.flags) != 0, ((1 << 1) & self.flags) != 0,
             ((1 << 2) & self.flags) != 0, ((1 << 3) & self.flags) != 0,
             ((1 << 4) & self.flags) != 0, ((1 << 5) & self.flags) != 0,
             ((1 << 6) & self.flags) != 0, ((1 << 7) & self.flags) != 0,
             ((1 << 8) & self.flags) != 0, ((1 << 9) & self.flags) != 0,
             ((1 << 10) & self.flags) != 0, ((1 << 11) & self.flags) != 0,
             ((1 << 12) & self.flags) != 0);

        if mem_valid {
            write!(f,"Mem{{Lower: {}, Upper{}}}, ", self.mem_lower, self.mem_upper)?;
        }

        if boot_device_valid {
            write!(f,"BootDevice{{{}}}, ", self.boot_device)?;
        }

        if cmdline_valid {
            write!(f,"Cmdline{{{:x}}}, ", self.cmdline)?;
        }

        if mods_valid {
            write!(f,"Mods{{Count: {}, Addr: {:x}}}, ", self.mods_count, self.mods_addr)?;
        }

        if sym_opt1{
            write!(f,"SymsOpt1{{")?;
            self.syms.fmt_opt1(f)?;
            write!(f,"}}, ")?;
        }

        if sym_opt2{
            write!(f,"SymsOpt2{{")?;
            self.syms.fmt_opt2(f)?;
            write!(f,"}}, ")?;
        }

        if mmap_valid {
            write!(f,"MMap{{Length: {}, Addr: 0x{:x}}}, ", self.mmap_length, self.mmap_addr)?;
        }

        if drives_valid {
            write!(f, "Drives{{Length: {}, Addr: {:x}}}, ", self.drives_length, self.drives_addr)?;
        }

        if config_table_valid {
            write!(f, "ConfigTable{{}}, ")?;
        }

        write!(f, "}}")?;
        Ok(())
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union Syms{
    pub syms_opt1: SymOpt1,
    pub syms_opt2: SymOpt2
}

impl Syms{
    fn fmt_opt1(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        unsafe { write!(f, "{:?}", self.syms_opt1) }
    }

    fn fmt_opt2(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        unsafe { write!(f, "{:?}", self.syms_opt2) }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SymOpt1{
    tab_size: u32,
    str_size: u32,
    addr: u32,
    _discard: u32
}

impl Debug for SymOpt1 {
    fn fmt(&self, _f: &mut Formatter<'_>) -> core::fmt::Result {
        Ok(())
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SymOpt2{
    num: u32,
    size: u32,
    addr: u32,
    shndx: u32
}

impl Debug for SymOpt2 {
    fn fmt(&self, _f: &mut Formatter<'_>) -> core::fmt::Result {
        Ok(())
    }
}