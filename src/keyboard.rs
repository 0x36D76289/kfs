use crate::io::{inb, outb};

// PS/2 keyboard ports
const KEYBOARD_DATA_PORT: u16 = 0x60;
const KEYBOARD_STATUS_PORT: u16 = 0x64;

// US QWERTY layout scan code table
static SCAN_CODE_TABLE: [u8; 128] = [
    0,          // 0x00: Error
    27,         // 0x01: Escape
    b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', b'-', b'=', 
    b'\x08',    // 0x0E: Backspace
    b'\t',      // 0x0F: Tab
    b'q', b'w', b'e', b'r', b't', b'y', b'u', b'i', b'o', b'p', b'[', b']', 
    b'\n',      // 0x1C: Enter
    0,          // 0x1D: Left Control
    b'a', b's', b'd', b'f', b'g', b'h', b'j', b'k', b'l', b';', b'\'', b'`', 
    0,          // 0x2A: Left Shift
    b'\\',      // 0x2B: Backslash
    b'z', b'x', b'c', b'v', b'b', b'n', b'm', b',', b'.', b'/', 
    0,          // 0x36: Right Shift
    b'*',       // 0x37: Keypad *
    0,          // 0x38: Left Alt
    b' ',       // 0x39: Space
    0,          // 0x3A: Caps Lock
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x3B-0x44: F1-F10
    0,          // 0x45: Num Lock
    0,          // 0x46: Scroll Lock
    b'7',       // 0x47: Keypad 7 (Home)
    b'8',       // 0x48: Keypad 8 (Up)
    b'9',       // 0x49: Keypad 9 (PgUp)
    b'-',       // 0x4A: Keypad -
    b'4',       // 0x4B: Keypad 4 (Left)
    b'5',       // 0x4C: Keypad 5
    b'6',       // 0x4D: Keypad 6 (Right)
    b'+',       // 0x4E: Keypad +
    b'1',       // 0x4F: Keypad 1 (End)
    b'2',       // 0x50: Keypad 2 (Down)
    b'3',       // 0x51: Keypad 3 (PgDn)
    b'0',       // 0x52: Keypad 0 (Ins)
    b'.',       // 0x53: Keypad . (Del)
    0, 0,       // 0x54-0x55: Alt-SysRq, Key 0x56
    0,          // 0x56: Usually backslash/pipe on non-US keyboards
    0, 0,       // 0x57-0x58: F11-F12
    0, 0, 0, 0, 0, 0, 0,          // 0x59-0x5F: Other keys
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x60-0x6F
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x70-0x7F
];

// US QWERTY layout - uppercase/shifted
static SHIFT_SCAN_CODE_TABLE: [u8; 128] = [
    0,          // 0x00: Error
    27,         // 0x01: Escape
    b'!', b'@', b'#', b'$', b'%', b'^', b'&', b'*', b'(', b')', b'_', b'+', 
    b'\x08',    // 0x0E: Backspace
    b'\t',      // 0x0F: Tab
    b'Q', b'W', b'E', b'R', b'T', b'Y', b'U', b'I', b'O', b'P', b'{', b'}', 
    b'\n',      // 0x1C: Enter
    0,          // 0x1D: Left Control
    b'A', b'S', b'D', b'F', b'G', b'H', b'J', b'K', b'L', b':', b'"', b'~', 
    0,          // 0x2A: Left Shift
    b'|',       // 0x2B: Backslash
    b'Z', b'X', b'C', b'V', b'B', b'N', b'M', b'<', b'>', b'?', 
    0,          // 0x36: Right Shift
    b'*',       // 0x37: Keypad *
    0,          // 0x38: Left Alt
    b' ',       // 0x39: Space
    0,          // 0x3A: Caps Lock
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x3B-0x44: F1-F10
    0,          // 0x45: Num Lock
    0,          // 0x46: Scroll Lock
    b'7',       // 0x47: Keypad 7 (Home)
    b'8',       // 0x48: Keypad 8 (Up)
    b'9',       // 0x49: Keypad 9 (PgUp)
    b'-',       // 0x4A: Keypad -
    b'4',       // 0x4B: Keypad 4 (Left)
    b'5',       // 0x4C: Keypad 5
    b'6',       // 0x4D: Keypad 6 (Right)
    b'+',       // 0x4E: Keypad +
    b'1',       // 0x4F: Keypad 1 (End)
    b'2',       // 0x50: Keypad 2 (Down)
    b'3',       // 0x51: Keypad 3 (PgDn)
    b'0',       // 0x52: Keypad 0 (Ins)
    b'.',       // 0x53: Keypad . (Del)
    0, 0,       // 0x54-0x55
    0,          // 0x56
    0, 0,       // 0x57-0x58: F11-F12
    0, 0, 0, 0, 0, 0, 0,          // 0x59-0x5F
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x60-0x6F
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x70-0x7F
];

#[derive(Debug, Copy, Clone)]
pub struct KeyEvent {
    pub key: u8,
    pub scancode: u8,
    pub is_release: bool,
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub is_extended: bool,
}

impl KeyEvent {
    pub fn is_function_key(&self) -> bool {
        // F1-F12: 0x3B-0x44, 0x57-0x58
        match self.scancode {
            0x3B..=0x44 | 0x57..=0x58 => true,
            _ => false
        }
    }

    pub fn function_key_num(&self) -> Option<u8> {
        match self.scancode {
            0x3B => Some(1),  // F1
            0x3C => Some(2),  // F2
            0x3D => Some(3),  // F3
            0x3E => Some(4),  // F4
            0x3F => Some(5),  // F5
            0x40 => Some(6),  // F6
            0x41 => Some(7),  // F7
            0x42 => Some(8),  // F8
            0x43 => Some(9),  // F9
            0x44 => Some(10), // F10
            0x57 => Some(11), // F11
            0x58 => Some(12), // F12
            _ => None,
        }
    }

    pub fn is_numpad_key(&self) -> bool {
        // Keypad 0-9 and operations
        match self.scancode {
            0x47..=0x53 => true,
            _ => false
        }
    }

    pub fn numpad_digit(&self) -> Option<u8> {
        match self.scancode {
            0x47 => Some(7), // Keypad 7
            0x48 => Some(8), // Keypad 8
            0x49 => Some(9), // Keypad 9
            0x4B => Some(4), // Keypad 4
            0x4C => Some(5), // Keypad 5
            0x4D => Some(6), // Keypad 6
            0x4F => Some(1), // Keypad 1
            0x50 => Some(2), // Keypad 2
            0x51 => Some(3), // Keypad 3
            0x52 => Some(0), // Keypad 0
            _ => None,
        }
    }
}

pub struct KeyboardState {
    shift_pressed: bool,
    ctrl_pressed: bool,
    alt_pressed: bool,
    caps_lock: bool,
    num_lock: bool,
    scroll_lock: bool,
    extended: bool,
}

impl KeyboardState {
    pub fn new() -> Self {
        KeyboardState {
            shift_pressed: false,
            ctrl_pressed: false,
            alt_pressed: false,
            caps_lock: false,
            num_lock: true, // Default on
            scroll_lock: false,
            extended: false,
        }
    }

    pub fn handle_scancode(&mut self, scancode: u8) -> Option<KeyEvent> {
        if scancode == 0xE0 {
            self.extended = true;
            return None;
        }

        let is_release = scancode & 0x80 != 0;
        let scancode = scancode & 0x7F; // Clear the high bit

        match scancode {
            0x1D => { // Ctrl
                self.ctrl_pressed = !is_release;
                return None;
            },
            0x2A | 0x36 => { // Left/Right Shift
                self.shift_pressed = !is_release;
                return None;
            },
            0x38 => { // Alt
                self.alt_pressed = !is_release;
                return None;
            },
            0x3A => { // Caps Lock
                if !is_release {
                    self.caps_lock = !self.caps_lock;
                    self.update_leds();
                }
                return None;
            },
            0x45 => { // Num Lock
                if !is_release {
                    self.num_lock = !self.num_lock;
                    self.update_leds();
                }
                return None;
            },
            0x46 => { // Scroll Lock
                if !is_release {
                    self.scroll_lock = !self.scroll_lock;
                    self.update_leds();
                }
                return None;
            },
            _ => {}
        }

        if is_release {
            self.extended = false;
            return None;
        }

        let key = if self.shift_pressed || (self.caps_lock && scancode >= 0x10 && scancode <= 0x32) {
            SHIFT_SCAN_CODE_TABLE[scancode as usize]
        } else {
            SCAN_CODE_TABLE[scancode as usize]
        };

        let key_event = KeyEvent {
            key,
            scancode,
            is_release,
            shift: self.shift_pressed,
            ctrl: self.ctrl_pressed,
            alt: self.alt_pressed,
            is_extended: self.extended,
        };

        self.extended = false;

        Some(key_event)
    }

    fn update_leds(&self) {
        unsafe {
            while inb(KEYBOARD_STATUS_PORT) & 2 != 0 {}
            outb(KEYBOARD_DATA_PORT, 0xED);

            while inb(KEYBOARD_STATUS_PORT) & 2 != 0 {}
            let led_state = ((self.scroll_lock as u8) << 0) |
                           ((self.num_lock as u8) << 1) |
                           ((self.caps_lock as u8) << 2);
            outb(KEYBOARD_DATA_PORT, led_state);
        }
    }
}

pub fn initialize_keyboard() {
    unsafe {
        // Reset the keyboard
        while inb(KEYBOARD_STATUS_PORT) & 2 != 0 {}
        outb(KEYBOARD_DATA_PORT, 0xFF);
        
        // Wait for ACK
        while inb(KEYBOARD_DATA_PORT) != 0xFA {}
        
        // Set default parameters
        while inb(KEYBOARD_STATUS_PORT) & 2 != 0 {}
        outb(KEYBOARD_DATA_PORT, 0xF6);
        
        // Wait for ACK
        while inb(KEYBOARD_DATA_PORT) != 0xFA {}
        
        // Enable scanning
        while inb(KEYBOARD_STATUS_PORT) & 2 != 0 {}
        outb(KEYBOARD_DATA_PORT, 0xF4);
        
        // Wait for ACK
        while inb(KEYBOARD_DATA_PORT) != 0xFA {}
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