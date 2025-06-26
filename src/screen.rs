use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

use crate::io::{inb, outb};

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new());
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::screen::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

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

/// The backgroud and foreground color contained in 8 bits
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

pub const MAX_SCREEN: usize = 4;
pub const VGA_BUFFER_HEIGHT: usize = 25;
pub const VGA_BUFFER_WIDTH: usize = 80;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT],
}

impl Buffer {
    fn new() -> Self {
        Self {
            chars: [[ScreenChar {
                ascii_character: b' ',
                color_code: ColorCode::new(Color::White, Color::Black),
            }; VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT],
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Screen {
    column_position: usize,
    row_position: usize,
    color_code: ColorCode,
    buffer: Buffer,
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            column_position: 0,
            row_position: 0,
            color_code: ColorCode::new(Color::White, Color::Black),
            buffer: Buffer::new(),
        }
    }
}

pub struct Writer {
    screens: [Screen; MAX_SCREEN],
    current_screen_id: usize,
    column_position: usize,
    row_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    /// Initialize a new Writer with proper cursor setup
    pub fn new() -> Self {
        let writer = Writer {
            screens: [Screen::default(); MAX_SCREEN],
            current_screen_id: 0,
            column_position: 0,
            row_position: 0,
            color_code: ColorCode::new(Color::White, Color::Black),
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        };
        
        // Enable hardware cursor with default scanlines (14-15 for typical VGA)
        writer.enable_cursor(14, 15);
        writer.update_cursor();
        writer
    }

    /// Create a new Writer instance, clearing the screen
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            b'\r' => self.column_position = 0,
            b'\t' => {
                for _ in 0..4 {
                    self.write_byte(b' ');
                }
            }
            b'\x08' => self.backspace(),
            byte => {
                if self.column_position >= VGA_BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.row_position;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };
                self.column_position += 1;
            }
        }
    }

    /// Write a string to the screen, handling non-printable characters
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' | b'\r' | b'\t' | b'\x08' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
        self.update_cursor();
    }

    /// Write a single character to the screen
    fn new_line(&mut self) {
        if self.row_position < VGA_BUFFER_HEIGHT - 1 {
            self.row_position += 1;
            self.column_position = 0;
        } else {
            for row in 1..VGA_BUFFER_HEIGHT {
                for col in 0..VGA_BUFFER_WIDTH {
                    let character = self.buffer.chars[row][col];
                    self.buffer.chars[row - 1][col] = character;
                }
            }
            self.clear_row(VGA_BUFFER_HEIGHT - 1);
            self.column_position = 0;
        }
        self.update_cursor();
    }

    /// Handle backspace by removing the last character
    fn backspace(&mut self) {
        let blank_char = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };

        if self.column_position > 0 {
            self.column_position -= 1;
            self.buffer.chars[self.row_position][self.column_position] = blank_char;
        } else if self.row_position > 0 {
            self.row_position -= 1;
            self.column_position = VGA_BUFFER_WIDTH - 1;

            self.buffer.chars[self.row_position][self.column_position] = blank_char;
        }
    }

    /// Clear a specific row by filling it with blank characters
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };

        for col in 0..VGA_BUFFER_WIDTH {
            self.buffer.chars[row][col] = blank;
        }
    }

    /// Clear the entire screen
    fn clear_screen(&mut self) {
        for row in 0..VGA_BUFFER_HEIGHT {
            self.clear_row(row);
        }
        self.row_position = 0;
        self.column_position = 0;
    }

    /// Get a mutable reference to the current screen
    fn curr_screen(&mut self) -> &mut Screen {
        &mut self.screens[self.current_screen_id]
    }

    /// Switch to a different screen by ID
    fn switch_to_screen(&mut self, screen_id: usize) {
        if screen_id >= MAX_SCREEN {
            panic!(
                "Tried to switch to screen {screen_id} but the max is {}",
                MAX_SCREEN - 1
            );
        }

        for row in 0..VGA_BUFFER_HEIGHT {
            for col in 0..VGA_BUFFER_WIDTH {
                self.curr_screen().buffer.chars[row][col] = self.buffer.chars[row][col];
            }
        }
        self.curr_screen().row_position = self.row_position;
        self.curr_screen().column_position = self.column_position;
        self.curr_screen().color_code = self.color_code;

        self.current_screen_id = screen_id;

        for row in 0..VGA_BUFFER_HEIGHT {
            for col in 0..VGA_BUFFER_WIDTH {
                self.buffer.chars[row][col] = self.curr_screen().buffer.chars[row][col];
            }
        }
        self.row_position = self.curr_screen().row_position;
        self.column_position = self.curr_screen().column_position;
        self.color_code = self.curr_screen().color_code;

        self.clear_row(self.row_position);
        self.column_position = 0;
        self.update_cursor();
    }

    /// Update the hardware cursor position based on current row and column
    fn update_cursor(&self) {
        let pos = self.row_position * VGA_BUFFER_WIDTH + self.column_position;

        outb(0x3D4, 0x0F);
        outb(0x3D5, (pos & 0xFF) as u8);
        outb(0x3D4, 0x0E);
        outb(0x3D5, ((pos >> 8) & 0xFF) as u8);
    }

    /// Enable hardware cursor with specified start and end scanlines
    pub fn enable_cursor(&self, cursor_start: u8, cursor_end: u8) {
        outb(0x3D4, 0x0A);
        outb(0x3D5, (inb(0x3D5) & 0xC0) | cursor_start);

        outb(0x3D4, 0x0B);
        outb(0x3D5, (inb(0x3D5) & 0xE0) | cursor_end);
    }

    /// Disable hardware cursor
    pub fn disable_cursor(&self) {
        outb(0x3D4, 0x0A);
        outb(0x3D5, 0x20);
    }

    /// Get current cursor position
    pub fn get_cursor_position(&self) -> u16 {
        let mut pos: u16 = 0;
        outb(0x3D4, 0x0F);
        pos |= inb(0x3D5) as u16;
        outb(0x3D4, 0x0E);
        pos |= (inb(0x3D5) as u16) << 8;
        pos
    }

    pub fn set_color(&mut self, color_code: ColorCode) {
        self.color_code = color_code;
    }
}

/// Implement fmt::Write for Writer to allow using it with format! macros
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

/// Write a byte to the current screen
pub fn switch_to_screen(screen_id: usize) {
    WRITER.lock().switch_to_screen(screen_id);
}

/// Clear the current screen
pub fn clear_screen() {
    WRITER.lock().clear_screen();
}

/// Change the color code for the current screen
pub fn change_color_code(color_code: ColorCode) {
    WRITER.lock().color_code = color_code;
}

/// Enable hardware cursor with default settings
pub fn enable_cursor() {
    WRITER.lock().enable_cursor(14, 15);
}

/// Enable hardware cursor with custom scanlines
pub fn enable_cursor_custom(start: u8, end: u8) {
    WRITER.lock().enable_cursor(start, end);
}

/// Disable hardware cursor
pub fn disable_cursor() {
    WRITER.lock().disable_cursor();
}

/// Get current cursor position
pub fn get_cursor_position() -> u16 {
    WRITER.lock().get_cursor_position()
}
