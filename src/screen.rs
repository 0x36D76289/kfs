use crate::vga::{VGA_WIDTH, VGA_HEIGHT, VGAChar, ColorCode, write_char, read_char, clear_screen};
use crate::io::outb;

pub struct Screen {
    cursor_x: usize,
    cursor_y: usize,
    pub color: ColorCode, // Make color field public
    buffer: [[VGAChar; VGA_WIDTH]; VGA_HEIGHT],
    is_active: bool,
}

impl Screen {
    pub fn new(color: ColorCode) -> Screen {
        let blank_char = VGAChar {
            character: b' ',
            color_code: color,
        };
        
        let mut screen = Screen {
            cursor_x: 0,
            cursor_y: 0,
            color,
            buffer: [[blank_char; VGA_WIDTH]; VGA_HEIGHT],
            is_active: false,
        };
        
        screen.clear();
        screen
    }
    
    pub fn clear(&mut self) {
        let blank_char = VGAChar {
            character: b' ',
            color_code: self.color,
        };
        
        for y in 0..VGA_HEIGHT {
            for x in 0..VGA_WIDTH {
                self.buffer[y][x] = blank_char;
            }
        }
        
        self.cursor_x = 0;
        self.cursor_y = 0;
        
        if self.is_active {
            unsafe { clear_screen(self.color); }
            self.update_cursor();
        }
    }
    
    pub fn activate(&mut self) {
        if self.is_active {
            return;
        }
        
        self.is_active = true;
        
        // Write buffer to VGA
        unsafe {
            for y in 0..VGA_HEIGHT {
                for x in 0..VGA_WIDTH {
                    write_char(x, y, self.buffer[y][x]);
                }
            }
        }
        
        self.update_cursor();
    }
    
    pub fn deactivate(&mut self) {
        if !self.is_active {
            return;
        }
        
        // Save VGA to buffer
        unsafe {
            for y in 0..VGA_HEIGHT {
                for x in 0..VGA_WIDTH {
                    self.buffer[y][x] = read_char(x, y);
                }
            }
        }
        
        self.is_active = false;
    }
    
    pub fn write_char(&mut self, c: u8) {
        match c {
            b'\n' => self.new_line(),
            b'\r' => self.cursor_x = 0,
            b'\t' => {
                // Tab is 4 spaces
                for _ in 0..4 {
                    self.write_char(b' ');
                }
            },
            b'\x08' => self.backspace(), // Backspace
            _ => {
                if self.cursor_x >= VGA_WIDTH {
                    self.new_line();
                }
                
                let char_to_write = VGAChar {
                    character: c,
                    color_code: self.color,
                };
                
                self.buffer[self.cursor_y][self.cursor_x] = char_to_write;
                
                if self.is_active {
                    unsafe { write_char(self.cursor_x, self.cursor_y, char_to_write); }
                }
                
                self.cursor_x += 1;
            }
        }
        
        if self.is_active {
            self.update_cursor();
        }
    }
    
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_char(byte);
        }
    }
    
    fn new_line(&mut self) {
        self.cursor_x = 0;
        self.cursor_y += 1;
        
        if self.cursor_y >= VGA_HEIGHT {
            self.scroll();
        }
    }
    
    fn backspace(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
            
            // Clear the character
            let blank_char = VGAChar {
                character: b' ',
                color_code: self.color,
            };
            
            self.buffer[self.cursor_y][self.cursor_x] = blank_char;
            
            if self.is_active {
                unsafe { write_char(self.cursor_x, self.cursor_y, blank_char); }
            }
        } else if self.cursor_y > 0 {
            // Go to the end of previous line
            self.cursor_y -= 1;
            self.cursor_x = VGA_WIDTH - 1;
            
            // Clear the character
            let blank_char = VGAChar {
                character: b' ',
                color_code: self.color,
            };
            
            self.buffer[self.cursor_y][self.cursor_x] = blank_char;
            
            if self.is_active {
                unsafe { write_char(self.cursor_x, self.cursor_y, blank_char); }
            }
        }
    }
    
    fn scroll(&mut self) {
        // Move everything up one line
        for y in 1..VGA_HEIGHT {
            for x in 0..VGA_WIDTH {
                self.buffer[y - 1][x] = self.buffer[y][x];
            }
        }
        
        // Clear the last line
        let blank_char = VGAChar {
            character: b' ',
            color_code: self.color,
        };
        
        for x in 0..VGA_WIDTH {
            self.buffer[VGA_HEIGHT - 1][x] = blank_char;
        }
        
        // Update cursor
        self.cursor_y = VGA_HEIGHT - 1;
        
        if self.is_active {
            // Update VGA buffer
            unsafe {
                for y in 0..VGA_HEIGHT {
                    for x in 0..VGA_WIDTH {
                        write_char(x, y, self.buffer[y][x]);
                    }
                }
            }
        }
    }
    
    fn update_cursor(&self) {
        let pos = self.cursor_y * VGA_WIDTH + self.cursor_x;
        
        unsafe {
            outb(0x3D4, 0x0F);
            outb(0x3D5, (pos & 0xFF) as u8);
            outb(0x3D4, 0x0E);
            outb(0x3D5, ((pos >> 8) & 0xFF) as u8);
        }
    }
    
    pub fn set_color(&mut self, color: ColorCode) {
        self.color = color;
    }
}
