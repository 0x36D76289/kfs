// VGA text mode constants
pub const VGA_BUFFER: *mut u16 = 0xb8000 as *mut u16;
pub const VGA_WIDTH: usize = 80;
pub const VGA_HEIGHT: usize = 25;

// VGA colors
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
    LightMagenta = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(pub u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

// VGA character representation (character + color attribute)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct VGAChar {
    pub character: u8,
    pub color_code: ColorCode,
}

// Functions to directly manipulate the VGA buffer
pub unsafe fn write_char(x: usize, y: usize, c: VGAChar) {
    if x < VGA_WIDTH && y < VGA_HEIGHT {
        let idx = y * VGA_WIDTH + x;
        unsafe {
            *VGA_BUFFER.add(idx) = ((c.color_code.0 as u16) << 8) | (c.character as u16);
        }
    }
}

pub unsafe fn read_char(x: usize, y: usize) -> VGAChar {
    let idx = y * VGA_WIDTH + x;
    let vga_value;
    unsafe {
        vga_value = *VGA_BUFFER.add(idx);
    }
    
    VGAChar {
        character: (vga_value & 0xFF) as u8,
        color_code: ColorCode((vga_value >> 8) as u8),
    }
}

pub unsafe fn clear_screen(color_code: ColorCode) {
    let blank = VGAChar {
        character: b' ',
        color_code,
    };
    
    for y in 0..VGA_HEIGHT {
        for x in 0..VGA_WIDTH {
            unsafe {
                write_char(x, y, blank);
            }
        }
    }
}
