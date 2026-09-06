use crate::memory::raw_mem_segment::RawMemSegment;
use crate::vga_display::vga_character::{VGACharacter, VGAColorBase, VGAColorCode};

pub struct VGAOutStream<'a>{
    current_color: VGAColorCode,
    cursor_position: usize,
    vga_mem: &'a mut RawMemSegment<'a>,
    buffer: [VGACharacter; 80*24],
    current_line: [VGACharacter; 80]
}

impl<'a> VGAOutStream<'a>{
    // ***** Public Functions *****
    pub fn flush_buffer(&mut self){
        let (_, buf_slice_u8, _) = unsafe { self.buffer.align_to::<u8>() };
        let (_, line_slice_u8, _) = unsafe { self.current_line.align_to::<u8>() };
        self.vga_mem.write_at(0, &buf_slice_u8);
        self.vga_mem.write_at(160 * 24, &line_slice_u8);
    }

    pub fn write_char(&mut self, c: char){
        if c == '\n'{
            self.shift_lines();
        }else{
            let character = VGACharacter::new(c, self.current_color.clone());
            self.current_line[self.cursor_position] = character;
            self.increment_cursor();
        }
    }

    pub fn write_string(&mut self, s: &str){
        for c in s.chars(){
            self.write_char(c);
        }
    }

    pub fn write_hex(&mut self, num: u64){
        self.write_char('0');
        self.write_char('x');

        let mut started: bool = false;
        for i in 0..16{
            let mask: u64 = 0xF << (4 * (15 - i));
            let significant: u8 = ((num & mask) >> (4 * (15 - i))) as u8;

            if significant != 0{
                started = true;
                let new_char: u8 = match significant {
                    0..=9 => ('0' as u8) + significant,
                    _ => ('A' as u8) + (significant - 10)
                };
                self.write_char(new_char as char);
            }else if started{
                self.write_char('0');
            }
        }

        if !started{
            self.write_char('0');
        }
    }

    pub fn set_color(&mut self, color: VGAColorCode){
        self.current_color = color;
    }

    // ***** Private Functions *****
    fn increment_cursor(&mut self){
        self.cursor_position += 1;

        if self.cursor_position >= 80 {
            self.shift_lines();
        }
    }

    fn shift_lines(&mut self){
        for i in 0..self.buffer.len(){
            if i + 80 >= self.buffer.len(){
                self.buffer[i] = self.current_line[(i+80) - self.buffer.len()];
            }else{
                self.buffer[i] = self.buffer[i + 80];
            }
        }
        self.current_line = [VGACharacter(0); 80];
        self.cursor_position = 0;
    }

    // ***** Struct Init *****
    pub fn with_segment<'c>(segment: &'c mut RawMemSegment<'c>) -> VGAOutStream<'c>{
        VGAOutStream::<'c>{
            current_color: VGAColorCode::new(VGAColorBase::White, VGAColorBase::Black),
            cursor_position: 0,
            vga_mem: segment,
            buffer: [VGACharacter(0); 80*24],
            current_line: [VGACharacter(0); 80]
        }
    }
}