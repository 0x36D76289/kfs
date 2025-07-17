// http://www.osdever.net/FreeVGA/vga/vga.htm
// https://wiki.osdev.org/VGA_Resources
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

static mut CURSOR_X: usize = 0;
static mut CURSOR_Y: usize = 0;

pub fn init() {
    clear_screen();
}

pub fn clear_screen() {
    unsafe {
        for i in 0..(VGA_WIDTH * VGA_HEIGHT * 2) {
            *VGA_BUFFER.add(i) = 0;
        }
        CURSOR_X = 0;
        CURSOR_Y = 0;
    }
}

pub fn print(text: &str) {
    for byte in text.bytes() {
        match byte {
            b'\n' => {
                unsafe {
                    CURSOR_X = 0;
                    CURSOR_Y += 1;
                    if CURSOR_Y >= VGA_HEIGHT {
                        CURSOR_Y = VGA_HEIGHT - 1;
                        scroll_up();
                    }
                }
            }
            byte => {
                unsafe {
                    let offset = (CURSOR_Y * VGA_WIDTH + CURSOR_X) * 2;
                    *VGA_BUFFER.add(offset) = byte;
                    *VGA_BUFFER.add(offset + 1) = 0x07; // Blanc sur noir
                    
                    CURSOR_X += 1;
                    if CURSOR_X >= VGA_WIDTH {
                        CURSOR_X = 0;
                        CURSOR_Y += 1;
                        if CURSOR_Y >= VGA_HEIGHT {
                            CURSOR_Y = VGA_HEIGHT - 1;
                            scroll_up();
                        }
                    }
                }
            }
        }
    }
}

fn scroll_up() {
    unsafe {
        for y in 1..VGA_HEIGHT {
            for x in 0..VGA_WIDTH {
                let src = (y * VGA_WIDTH + x) * 2;
                let dst = ((y - 1) * VGA_WIDTH + x) * 2;
                *VGA_BUFFER.add(dst) = *VGA_BUFFER.add(src);
                *VGA_BUFFER.add(dst + 1) = *VGA_BUFFER.add(src + 1);
            }
        }
        
        // Vider la dernière ligne
        for x in 0..VGA_WIDTH {
            let offset = ((VGA_HEIGHT - 1) * VGA_WIDTH + x) * 2;
            *VGA_BUFFER.add(offset) = 0;
            *VGA_BUFFER.add(offset + 1) = 0x07;
        }
    }
}
