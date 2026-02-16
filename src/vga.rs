const VGA_BUFFER_HEIGHT: usize = 25;
const VGA_BUFFER_WIDTH: usize = 80;
const VGA_BUFFER_ADDR: usize = 0xB8000;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
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
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    row_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn new() -> Writer {
        Writer {
            column_position: 0,
            row_position: 0,
            color_code: ColorCode::new(Color::White, Color::Black),
            buffer: unsafe { &mut *(VGA_BUFFER_ADDR as *mut Buffer) },
        }
    }

    pub fn set_color(&mut self, color_code: ColorCode) {
        self.color_code = color_code;
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= VGA_BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.row_position;
                let col = self.column_position;

                self.buffer.chars[row][col] = ScreenChar {
                    ascii_char: byte,
                    color_code: self.color_code,
                };
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        if self.row_position < VGA_BUFFER_HEIGHT - 1 {
            self.row_position += 1;
        } else {
            self.scroll();
        }
        self.column_position = 0;
    }

    fn scroll(&mut self) {
        for row in 1..VGA_BUFFER_HEIGHT {
            for col in 0..VGA_BUFFER_WIDTH {
                self.buffer.chars[row - 1][col] = self.buffer.chars[row][col];
            }
        }
        self.clear_row(VGA_BUFFER_HEIGHT - 1);
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_char: b' ',
            color_code: self.color_code,
        };
        for col in 0..VGA_BUFFER_WIDTH {
            self.buffer.chars[row][col] = blank;
        }
    }

    pub fn clear_screen(&mut self) {
        for row in 0..VGA_BUFFER_HEIGHT {
            self.clear_row(row);
        }
        self.column_position = 0;
        self.row_position = 0;
    }
}

#[allow(dead_code)]
pub fn strlen(s: &[u8]) -> usize {
    let mut len = 0;
    for &byte in s {
        if byte == 0 {
            break;
        }
        len += 1;
    }
    len
}

#[allow(dead_code)]
pub fn strcmp(s1: &[u8], s2: &[u8]) -> i32 {
    let len = if s1.len() < s2.len() {
        s1.len()
    } else {
        s2.len()
    };

    for i in 0..len {
        if s1[i] != s2[i] {
            return (s1[i] as i32) - (s2[i] as i32);
        }
        if s1[i] == 0 {
            return 0;
        }
    }

    (s1.len() as i32) - (s2.len() as i32)
}

#[allow(dead_code)]
pub fn memcpy(dest: &mut [u8], src: &[u8], n: usize) {
    let len = if n < dest.len() { n } else { dest.len() };
    let len = if len < src.len() { len } else { src.len() };

    for i in 0..len {
        dest[i] = src[i];
    }
}

#[allow(dead_code)]
pub fn memset(dest: &mut [u8], val: u8, n: usize) {
    let len = if n < dest.len() { n } else { dest.len() };

    for i in 0..len {
        dest[i] = val;
    }
}
