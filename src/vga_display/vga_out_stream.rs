pub use core::fmt::Write;
use crate::memory::memory_map::MemoryMap;
use crate::memory::raw_mem_segment::RawMemSegment;
use crate::vga_display::vga_character::{VGACharacter, VGAColorCode};
use crate::vga_display::vga_out_buffer::VGAOutBuffer;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        use crate::collections::globals::Globals;
        let vga_stream = Globals::get_mut().get_vga_out_stream().unwrap();
        vga_stream.write_fmt(format_args!($($arg)*)).unwrap();
        vga_stream.flush();
    });
}

#[macro_export]
macro_rules! println {
    () => {
        use crate::print;
        print!("\n");
    };
    ($fmt:expr) => {
        use crate::print;
        print!(concat!($fmt, "\n"));
    };
    ($fmt:expr, $($arg:tt)*) => {
        use crate::print;
        print!(concat!($fmt, "\n"), $($arg)*);
    };
}

#[allow(dead_code)]
pub struct VGAOutStream<'a>{
    vga_mem: RawMemSegment<'a>,
    out_buffer: VGAOutBuffer,
    reading_escape: bool,
    escape_len: usize,
    escape_code_read: [char; 3]
}

#[allow(dead_code)]
impl<'a> VGAOutStream<'a>{
    // ***** Public Functions *****
    pub fn flush(&mut self){
        let (_, img_slice_u8, _) = unsafe { self.out_buffer.image_buffer.align_to::<u8>() };
        self.vga_mem.write_at(0, &img_slice_u8);

        for (idx, c) in self.out_buffer.buffer.iter().enumerate() {
            if c.val() != 0 && c.char() != '\n' && c.char() != ' '{
                self.vga_mem.write_at(idx * 2, &c.bytes());
            }
        }
    }

    pub fn flush_no_image(&mut self){
        for (idx, c) in self.out_buffer.buffer.iter().enumerate() {
            self.vga_mem.write_at(idx * 2, &c.bytes());
        }
    }

    pub fn clear(&mut self){
        self.vga_mem.clear();
    }

    pub fn write_vga_char(&mut self, vc: VGACharacter){
        if vc.char() == '\n' {
            self.shift_lines();
        }else {
            self.out_buffer.buffer[(80*24) + (self.out_buffer.cursor_position/2) as usize] = vc;
            self.increment_cursor();
        }
    }

    pub fn write_string(&mut self, s: &str){
        for c in s.chars(){
            self.write_char(c);
        }
    }

    pub fn write_char(&mut self, c: char) {
        if c == '\n' {
            self.shift_lines();
        }else if self.reading_escape {
            if self.escape_len == 0{
                if c == 'c' {
                    self.escape_code_read[0] = c;
                    self.escape_len += 1;
                }else{
                    self.reading_escape = false;
                }
            }else if self.escape_len == 1{
                if (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f'){
                    self.escape_code_read[1] = c;
                    self.escape_len += 1;
                }else{
                    self.reading_escape = false;
                }
            }else if self.escape_len == 2{
                if (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f'){
                    self.escape_code_read[2] = c;
                    self.escape_len += 1;

                    let val: u8;
                    let higher: u8 = Self::single_hex_to_u8(self.escape_code_read[1]);
                    let lower: u8 = Self::single_hex_to_u8(self.escape_code_read[2]);
                    val = (higher * 16) + lower;

                    self.set_color(VGAColorCode(val));
                    self.reading_escape = false;
                }else{
                    self.reading_escape = false;
                }
            }
        }else if c == '\\'{
            self.escape_len = 0;
            self.reading_escape = true;
        }else{
            let character = VGACharacter::new(c, self.out_buffer.current_color);
            self.out_buffer.buffer[(80*24) + (self.out_buffer.cursor_position/2) as usize] = character;
            self.increment_cursor();
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
        self.out_buffer.current_color = color;
    }

    pub fn color(&self) -> VGAColorCode{
        self.out_buffer.current_color
    }

    pub fn set_cursor_position(&mut self, new_pos: u8) {
        self.out_buffer.cursor_position = new_pos;
    }

    pub fn cursor_position(&self) -> usize{
        self.out_buffer.cursor_position as usize
    }

    // ***** Private Functions *****
    fn increment_cursor(&mut self){
        self.out_buffer.cursor_position += 2;

        if self.out_buffer.cursor_position >= 160{
            self.shift_lines();
        }
    }

    fn shift_lines(&mut self){
        for i in 0..self.out_buffer.buffer.len(){
            if i + 80 >= self.out_buffer.buffer.len(){
                self.out_buffer.buffer[i] = VGACharacter(0);
            }else{
                self.out_buffer.buffer[i] = self.out_buffer.buffer[i+80];
            }
        }

        self.out_buffer.cursor_position = 0;
    }

    fn single_hex_to_u8(c: char) -> u8{
        if c >= '0' && c <= '9'{
            c as u8 - '0' as u8
        }else if c >= 'a' && c <= 'f'{
            c as u8 - 'a' as u8 + 10
        }else{
            c as u8 - 'A' as u8 + 10
        }
    }

    // ***** Struct Init *****
    pub fn with_segment<'c>(segment: RawMemSegment<'c>) -> VGAOutStream<'c>{
        VGAOutStream::<'c>{
            vga_mem: segment,
            out_buffer: VGAOutBuffer::default(),
            reading_escape: false,
            escape_len: 0,
            escape_code_read: [0 as char, 0 as char, 0 as char]
        }
    }

    pub fn with_segment_and_buffer<'c>(segment: RawMemSegment<'c>, buf: &VGAOutBuffer) -> VGAOutStream<'c>{
        VGAOutStream::<'c>{
            vga_mem: segment,
            out_buffer: buf.clone(),
            reading_escape: false,
            escape_len: 0,
            escape_code_read: [0 as char, 0 as char, 0 as char]
        }
    }

    pub fn from_buffer<'b>(mem_map: &mut MemoryMap, buf: &VGAOutBuffer) -> VGAOutStream<'b>{
        VGAOutStream {
            vga_mem: mem_map.borrow_segment(0xb8000 as *const u8, 160 * 25),
            out_buffer: buf.clone(),
            reading_escape: false,
            escape_len: 0,
            escape_code_read: [0 as char, 0 as char, 0 as char]
        }
    }

    pub fn from_map(mem_map: &mut MemoryMap) -> VGAOutStream<'_>{
        VGAOutStream{
            vga_mem: mem_map.borrow_segment(0xb8000 as *const u8, 160 * 25),
            out_buffer: VGAOutBuffer::default(),
            reading_escape: false,
            escape_len: 0,
            escape_code_read: [0 as char, 0 as char, 0 as char]
        }
    }
}

impl Write for VGAOutStream<'_>{
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars(){
            self.write_char(c);
        }

        Ok(())
    }
}