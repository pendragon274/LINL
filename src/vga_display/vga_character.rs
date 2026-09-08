#[derive(Clone)]
#[repr(u8)]
#[allow(dead_code)]
pub enum VGAColorBase{
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15
}

#[derive(Clone, Copy, Default)]
#[repr(transparent)]
pub struct VGAColorCode(pub u8);

impl VGAColorCode{
    pub fn val(&self) -> u8{
        self.0
    }

    pub fn new(text: VGAColorBase, background: VGAColorBase) -> VGAColorCode{
        VGAColorCode(((background as u8) << 4) + (text as u8))
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct VGACharacter(pub u16);

#[allow(dead_code)]
impl VGACharacter{
    pub fn bytes(&self) -> [u8; 2]{
        self.0.to_ne_bytes()
    }

    pub fn char(&self) -> char{
        let mut val = self.0;
        val &= 0xFF;
        val as u8 as char
    }

    pub fn val(&self) -> u16{
        self.0
    }

    pub fn new(c: char, color: VGAColorCode) -> VGACharacter{
        VGACharacter((((color.val()) as u16) << 8) + (c as u16))
    }
}