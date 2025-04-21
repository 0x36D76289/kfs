use crate::io::{inb, outb};

// PS/2 keyboard ports
const KEYBOARD_DATA_PORT: u16 = 0x60;
const KEYBOARD_STATUS_PORT: u16 = 0x64;

// Scan code to ASCII mappings (US QWERTY layout)
static SCAN_CODE_TABLE: [u8; 128] = [
    0, 27, b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', b'-', b'=', b'\x08',
    b'\t', b'q', b'w', b'e', b'r', b't', b'y', b'u', b'i', b'o', b'p', b'[', b']', b'\n',
    0, b'a', b's', b'd', b'f', b'g', b'h', b'j', b'k', b'l', b';', b'\'', b'`', 0,
    b'\\', b'z', b'x', b'c', b'v', b'b', b'n', b'm', b',', b'.', b'/', 0, b'*', 0,
    b' ', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, b'-', 0, 0, 0, b'+',
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
]; // Added extra 0 to make it 128 elements

// Special key codes
pub const KEY_ESCAPE: u8 = 1;
pub const KEY_BACKSPACE: u8 = 14;
pub const KEY_TAB: u8 = 15;
pub const KEY_ENTER: u8 = 28;
pub const KEY_CTRL: u8 = 29;
pub const KEY_LSHIFT: u8 = 42;
pub const KEY_RSHIFT: u8 = 54;
pub const KEY_ALT: u8 = 56;
pub const KEY_F1: u8 = 59;
pub const KEY_F2: u8 = 60;
pub const KEY_F3: u8 = 61;
pub const KEY_F4: u8 = 62;

pub struct KeyboardState {
    shift_pressed: bool,
    ctrl_pressed: bool,
    alt_pressed: bool,
}

impl KeyboardState {
    pub fn new() -> KeyboardState {
        KeyboardState {
            shift_pressed: false,
            ctrl_pressed: false,
            alt_pressed: false,
        }
    }
    
    pub fn handle_scancode(&mut self, scancode: u8) -> Option<KeyEvent> {
        let released = scancode & 0x80 != 0;
        let code = scancode & 0x7F;
        
        match code {
            KEY_LSHIFT | KEY_RSHIFT => {
                self.shift_pressed = !released;
                None
            },
            KEY_CTRL => {
                self.ctrl_pressed = !released;
                None
            },
            KEY_ALT => {
                self.alt_pressed = !released;
                None
            },
            _ if released => None,
            _ => {
                let c = SCAN_CODE_TABLE[code as usize];
                if c == 0 {
                    None
                } else {
                    Some(KeyEvent {
                        key: c,
                        scancode: code,
                        shift_pressed: self.shift_pressed,
                        ctrl_pressed: self.ctrl_pressed,
                        alt_pressed: self.alt_pressed,
                    })
                }
            }
        }
    }
}

pub struct KeyEvent {
    pub key: u8,
    pub scancode: u8,
    pub shift_pressed: bool,
    pub ctrl_pressed: bool,
    pub alt_pressed: bool,
}

pub fn initialize_keyboard() {
    // Simple keyboard initialization - just checking if keyboard exists
    unsafe {
        // Wait for input buffer to be empty
        while inb(KEYBOARD_STATUS_PORT) & 2 != 0 {}
        
        // Reset the keyboard
        outb(KEYBOARD_DATA_PORT, 0xFF);
        
        // Wait for acknowledgment
        while inb(KEYBOARD_STATUS_PORT) & 1 == 0 {}
        let _ = inb(KEYBOARD_DATA_PORT);
    }
}

pub fn read_scancode() -> Option<u8> {
    unsafe {
        // Check if there's a key press waiting
        if inb(KEYBOARD_STATUS_PORT) & 1 != 0 {
            Some(inb(KEYBOARD_DATA_PORT))
        } else {
            None
        }
    }
}
