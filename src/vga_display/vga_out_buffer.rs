use crate::vga_display::vga_character::{VGACharacter, VGAColorCode};

#[repr(C)]
#[derive(Clone)]
pub struct VGAOutBuffer{
    pub buffer: [VGACharacter; 80 * 25],
    pub image_buffer: [VGACharacter; 80 * 25],
    pub current_color: VGAColorCode,
    pub cursor_position: u8
}

impl Default for VGAOutBuffer{
    fn default()->Self{
        Self{
            buffer: [VGACharacter(0); 80 * 25],
            image_buffer: [VGACharacter(0); 80 * 25],
            current_color: VGAColorCode::default(),
            cursor_position: 0
        }
    }
}