use core::fmt::{Debug, Formatter};

pub struct StrDebugFormat<'a> {
    int_format: bool,
    data: &'a [u8]
}

impl<'a> StrDebugFormat<'a> {
    pub fn from_u32(data: &'a [u8]) -> StrDebugFormat<'a> {
        StrDebugFormat {
            int_format: true,
            data
        }
    }

    pub fn from_cstr(data: &'a [u8]) -> StrDebugFormat<'a> {
        for (idx, &c) in data.iter().enumerate(){
            if c == 0{
                return StrDebugFormat {
                    int_format: false,
                    data: &data[0..idx]
                };
            }
        }

        StrDebugFormat{
            int_format: false,
            data
        }
    }

    pub fn from(data: &'a [u8]) -> StrDebugFormat<'a> {
        StrDebugFormat{
            int_format: false,
            data
        }
    }
}

impl Debug for StrDebugFormat<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self.int_format {
            true => {
                write!(f, "{}", u32::from_ne_bytes(self.data[0..4].try_into().unwrap()))?;
            }, false => {
                for byte in self.data {
                    write!(f, "{}", *byte as char)?;
                }
            }
        }

        Ok(())
    }
}