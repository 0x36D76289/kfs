use crate::io::{inb, outb};

// PS/2 keyboard ports
const KEYBOARD_DATA_PORT: u16 = 0x60;
const KEYBOARD_STATUS_PORT: u16 = 0x64;

const EXTENDED_KEY: u8 = 0xE0;

// Scan code QWERTY
static SCAN_CODE_TABLE: [u8; 128] = [
    0, 27, b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', b'-', b'=', b'\x08',
    b'\t', b'q', b'w', b'e', b'r', b't', b'y', b'u', b'i', b'o', b'p', b'[', b']', b'\n',
    0, b'a', b's', b'd', b'f', b'g', b'h', b'j', b'k', b'l', b';', b'\'', b'`', 0,
    b'\\', b'z', b'x', b'c', b'v', b'b', b'n', b'm', b',', b'.', b'/', 0, b'*', 0,
    b' ', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, b'-', 0, 0, 0, b'+',
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
];

// Function key mappings (F1-F12)
const KEY_F1_SCANCODE: u8 = 0x3B;
const KEY_F2_SCANCODE: u8 = 0x3C;
const KEY_F3_SCANCODE: u8 = 0x3D;
const KEY_F4_SCANCODE: u8 = 0x3E;
// const KEY_F5_SCANCODE: u8 = 0x3F;
// const KEY_F6_SCANCODE: u8 = 0x40;
// const KEY_F7_SCANCODE: u8 = 0x41;
// const KEY_F8_SCANCODE: u8 = 0x42;
// const KEY_F9_SCANCODE: u8 = 0x43;
// const KEY_F10_SCANCODE: u8 = 0x44;
// const KEY_F11_SCANCODE: u8 = 0x57;
// const KEY_F12_SCANCODE: u8 = 0x58;

// Special key codes
pub const KEY_ESCAPE: u8 = 1;
pub const KEY_BACKSPACE: u8 = 14;
pub const KEY_TAB: u8 = 15;
pub const KEY_ENTER: u8 = 28;
pub const KEY_CTRL: u8 = 29;
pub const KEY_LSHIFT: u8 = 42;
pub const KEY_RSHIFT: u8 = 54;
pub const KEY_ALT: u8 = 56;

// Function key codes (custom values beyond ASCII range for special handling)
pub const KEY_F1: u8 = 128;
pub const KEY_F2: u8 = 129;
pub const KEY_F3: u8 = 130;
pub const KEY_F4: u8 = 131;
pub const KEY_F5: u8 = 132;
pub const KEY_F6: u8 = 133;
pub const KEY_F7: u8 = 134;
pub const KEY_F8: u8 = 135;
pub const KEY_F9: u8 = 136;
pub const KEY_F10: u8 = 137;
pub const KEY_F11: u8 = 138;
pub const KEY_F12: u8 = 139;

// Numpad key codes (custom values beyond ASCII range)
pub const KEY_NUMPAD_0: u8 = 140;
pub const KEY_NUMPAD_1: u8 = 141;
pub const KEY_NUMPAD_2: u8 = 142;
pub const KEY_NUMPAD_3: u8 = 143;
pub const KEY_NUMPAD_4: u8 = 144;
pub const KEY_NUMPAD_5: u8 = 145;
pub const KEY_NUMPAD_6: u8 = 146;
pub const KEY_NUMPAD_7: u8 = 147;
pub const KEY_NUMPAD_8: u8 = 148;
pub const KEY_NUMPAD_9: u8 = 149;

pub struct KeyboardState {
    shift_pressed: bool,
    ctrl_pressed: bool,
    alt_pressed: bool,
    extended_key: bool,
}

impl KeyboardState {
    pub fn new() -> KeyboardState {
        KeyboardState {
            shift_pressed: false,
            ctrl_pressed: false,
            alt_pressed: false,
            extended_key: false,
        }
    }
    
    pub fn handle_scancode(&mut self, scancode: u8) -> Option<KeyEvent> {
        // Handle extended key prefix
        if scancode == EXTENDED_KEY {
            self.extended_key = true;
            return None;
        }
        
        let released = scancode & 0x80 != 0;
        let code = scancode & 0x7F;
        
        if self.extended_key {
            self.extended_key = false;
            
            match code {
                0x35 => { // Numpad /
                    if !released { return Some(KeyEvent::new(b'/', code, self)); }
                    None
                },
                0x1C => { // Numpad Enter
                    if !released { return Some(KeyEvent::new(b'\n', code, self)); }
                    None
                },
                _ => None,
            }
        } else {
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
                // Function keys
                KEY_F1_SCANCODE => {
                    if !released { return Some(KeyEvent::new(KEY_F1, code, self)); }
                    None
                },
                KEY_F2_SCANCODE => {
                    if !released { return Some(KeyEvent::new(KEY_F2, code, self)); }
                    None
                },
                KEY_F3_SCANCODE => {
                    if !released { return Some(KeyEvent::new(KEY_F3, code, self)); }
                    None
                },
                KEY_F4_SCANCODE => {
                    if !released { return Some(KeyEvent::new(KEY_F4, code, self)); }
                    None
                },
                // Numpad keys without NumLock handling for simplicity
                0x52 => { // Numpad 0
                    if !released { return Some(KeyEvent::new(KEY_NUMPAD_0, code, self)); }
                    None
                },
                0x4F => { // Numpad 1
                    if !released { return Some(KeyEvent::new(KEY_NUMPAD_1, code, self)); }
                    None
                },
                0x50 => { // Numpad 2
                    if !released { return Some(KeyEvent::new(KEY_NUMPAD_2, code, self)); }
                    None
                },
                0x51 => { // Numpad 3
                    if !released { return Some(KeyEvent::new(KEY_NUMPAD_3, code, self)); }
                    None
                },
                0x4B => { // Numpad 4
                    if !released { return Some(KeyEvent::new(KEY_NUMPAD_4, code, self)); }
                    None
                },
                0x4C => { // Numpad 5
                    if !released { return Some(KeyEvent::new(KEY_NUMPAD_5, code, self)); }
                    None
                },
                0x4D => { // Numpad 6
                    if !released { return Some(KeyEvent::new(KEY_NUMPAD_6, code, self)); }
                    None
                },
                0x47 => { // Numpad 7
                    if !released { return Some(KeyEvent::new(KEY_NUMPAD_7, code, self)); }
                    None
                },
                0x48 => { // Numpad 8
                    if !released { return Some(KeyEvent::new(KEY_NUMPAD_8, code, self)); }
                    None
                },
                0x49 => { // Numpad 9
                    if !released { return Some(KeyEvent::new(KEY_NUMPAD_9, code, self)); }
                    None
                },
                _ if released => None,
                _ => {
                    if code < SCAN_CODE_TABLE.len() as u8 {
                        let c = SCAN_CODE_TABLE[code as usize];
                        if c == 0 {
                            None
                        } else {
                            Some(KeyEvent::new(c, code, self))
                        }
                    } else {
                        None
                    }
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

impl KeyEvent {
    // Helper constructor
    fn new(key: u8, scancode: u8, state: &KeyboardState) -> Self {
        KeyEvent {
            key,
            scancode,
            shift_pressed: state.shift_pressed,
            ctrl_pressed: state.ctrl_pressed,
            alt_pressed: state.alt_pressed,
        }
    }
    
    pub fn is_function_key(&self) -> bool {
        self.key >= KEY_F1 && self.key <= KEY_F12
    }
    
    pub fn is_numpad_key(&self) -> bool {
        self.key >= KEY_NUMPAD_0 && self.key <= KEY_NUMPAD_9
    }
    
    pub fn function_key_num(&self) -> Option<u8> {
        if self.is_function_key() {
            Some(self.key - KEY_F1 + 1)
        } else {
            None
        }
    }
    
    pub fn numpad_digit(&self) -> Option<u8> {
        if self.is_numpad_key() {
            Some(self.key - KEY_NUMPAD_0)
        } else {
            None
        }
    }
}

pub fn initialize_keyboard() {
    unsafe {
        while inb(KEYBOARD_STATUS_PORT) & 2 != 0 {}
        
        outb(KEYBOARD_DATA_PORT, 0xFF);
        
        while inb(KEYBOARD_STATUS_PORT) & 1 == 0 {}
        let _ = inb(KEYBOARD_DATA_PORT);
    }
}

pub fn read_scancode() -> Option<u8> {
    unsafe {
        if inb(KEYBOARD_STATUS_PORT) & 1 != 0 {
            Some(inb(KEYBOARD_DATA_PORT))
        } else {
            None
        }
    }
}
