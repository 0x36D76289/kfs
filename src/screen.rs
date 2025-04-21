use crate::io::outb;
use crate::printk::init_printk;
use crate::vga::{
    Color, ColorCode, VGA_HEIGHT, VGA_WIDTH, VGAChar, clear_screen, read_char, write_char,
};
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct Screen {
    cursor_x: usize,
    cursor_y: usize,
    pub color: ColorCode,
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
            unsafe {
                clear_screen(self.color);
            }
            self.update_cursor();
        }
    }

    pub fn activate(&mut self) {
        if self.is_active {
            return;
        }

        self.is_active = true;

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
                for _ in 0..4 {
                    self.write_char(b' ');
                }
            }
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
                    unsafe {
                        write_char(self.cursor_x, self.cursor_y, char_to_write);
                    }
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

            let blank_char = VGAChar {
                character: b' ',
                color_code: self.color,
            };

            self.buffer[self.cursor_y][self.cursor_x] = blank_char;

            if self.is_active {
                unsafe {
                    write_char(self.cursor_x, self.cursor_y, blank_char);
                }
            }
        } else if self.cursor_y > 0 {
            self.cursor_y -= 1;
            self.cursor_x = VGA_WIDTH - 1;

            let blank_char = VGAChar {
                character: b' ',
                color_code: self.color,
            };

            self.buffer[self.cursor_y][self.cursor_x] = blank_char;

            if self.is_active {
                unsafe {
                    write_char(self.cursor_x, self.cursor_y, blank_char);
                }
            }
        }
    }

    fn scroll(&mut self) {
        for y in 1..VGA_HEIGHT {
            for x in 0..VGA_WIDTH {
                self.buffer[y - 1][x] = self.buffer[y][x];
            }
        }

        let blank_char = VGAChar {
            character: b' ',
            color_code: self.color,
        };

        for x in 0..VGA_WIDTH {
            self.buffer[VGA_HEIGHT - 1][x] = blank_char;
        }

        self.cursor_y = VGA_HEIGHT - 1;

        if self.is_active {
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

pub const MAX_SCREENS: usize = 4;

static ACTIVE_SCREEN: AtomicUsize = AtomicUsize::new(0);
static mut SCREEN_INITIALIZED: [bool; MAX_SCREENS] = [false, false, false, false];
static mut SCREENS: [Option<Screen>; MAX_SCREENS] = [None, None, None, None];

pub fn init_screens() {
    unsafe {
        SCREENS[0] = Some(Screen::new(ColorCode::new(Color::White, Color::Black)));

        SCREENS[1] = Some(Screen::new(ColorCode::new(Color::Green, Color::Black)));

        SCREENS[2] = Some(Screen::new(ColorCode::new(Color::Cyan, Color::Blue)));

        SCREENS[3] = Some(Screen::new(ColorCode::new(Color::Black, Color::LightGray)));

        SCREEN_INITIALIZED[0] = true;

        if let Some(ref mut screen) = SCREENS[0] {
            screen.activate();
            init_printk(screen as *mut Screen);
        }
    }
}

pub fn switch_to_screen(screen_idx: usize) -> bool {
    if screen_idx >= MAX_SCREENS {
        return false;
    }

    let current_idx = ACTIVE_SCREEN.load(Ordering::SeqCst);

    if current_idx == screen_idx {
        return false;
    }

    unsafe {
        if let Some(ref mut screen) = SCREENS[current_idx] {
            screen.deactivate();
        }

        if let Some(ref mut screen) = SCREENS[screen_idx] {
            screen.activate();

            init_printk(screen as *mut Screen);

            ACTIVE_SCREEN.store(screen_idx, Ordering::SeqCst);

            let first_time = !SCREEN_INITIALIZED[screen_idx];
            SCREEN_INITIALIZED[screen_idx] = true;

            return first_time;
        }
    }

    false
}

pub fn get_active_screen() -> usize {
    ACTIVE_SCREEN.load(Ordering::SeqCst)
}

pub unsafe fn get_active_screen_ref() -> Option<&'static mut Screen> {
    let idx = get_active_screen();
    unsafe {
        if let Some(ref mut screen) = SCREENS[idx] {
            Some(screen)
        } else {
            None
        }
    }
}
