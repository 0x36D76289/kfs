use crate::io::{inb, outb};
use crate::keyboard::Key::{Character, Named};

// PS/2 keyboard ports
const KEYBOARD_DATA_PORT: u16 = 0x60;
const KEYBOARD_STATUS_PORT: u16 = 0x64;

#[derive(Debug, Copy, Clone)]
pub enum NamedKey {
    Alt,
    CapsLock,
    Control,
    NumLock,
    ScrollLock,
    Shift,
    Enter,
    Tab,
    Space,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    End,
    Home,
    PageDown,
    PageUp,
    Backspace,
    Delete,
    Insert,
    Paste,
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
}

#[derive(Debug, Copy, Clone)]
pub enum Key {
    Named(NamedKey),
    Character(char),
}

impl Key {
    pub fn from_scan_code(scan_code: u8, shift_pressed: bool) -> Key {
        match scan_code {
            // Alphanumeric keys
            0x02 => Character(if shift_pressed { '!' } else { '1' }),
            0x03 => Character(if shift_pressed { '@' } else { '2' }),
            0x04 => Character(if shift_pressed { '#' } else { '3' }),
            0x05 => Character(if shift_pressed { '$' } else { '4' }),
            0x06 => Character(if shift_pressed { '%' } else { '5' }),
            0x07 => Character(if shift_pressed { '^' } else { '6' }),
            0x08 => Character(if shift_pressed { '&' } else { '7' }),
            0x09 => Character(if shift_pressed { '*' } else { '8' }),
            0x0A => Character(if shift_pressed { '(' } else { '9' }),
            0x0B => Character(if shift_pressed { ')' } else { '0' }),
            0x0C => Character(if shift_pressed { '_' } else { '-' }),
            0x0D => Character(if shift_pressed { '+' } else { '=' }),
            0x10 => Character(if shift_pressed { 'Q' } else { 'q' }),
            0x11 => Character(if shift_pressed { 'W' } else { 'w' }),
            0x12 => Character(if shift_pressed { 'E' } else { 'e' }),
            0x13 => Character(if shift_pressed { 'R' } else { 'r' }),
            0x14 => Character(if shift_pressed { 'T' } else { 't' }),
            0x15 => Character(if shift_pressed { 'Y' } else { 'y' }),
            0x16 => Character(if shift_pressed { 'U' } else { 'u' }),
            0x17 => Character(if shift_pressed { 'I' } else { 'i' }),
            0x18 => Character(if shift_pressed { 'O' } else { 'o' }),
            0x19 => Character(if shift_pressed { 'P' } else { 'p' }),
            0x1A => Character(if shift_pressed { '{' } else { '[' }),
            0x1B => Character(if shift_pressed { '}' } else { ']' }),
            0x1E => Character(if shift_pressed { 'A' } else { 'a' }),
            0x1F => Character(if shift_pressed { 'S' } else { 's' }),
            0x20 => Character(if shift_pressed { 'D' } else { 'd' }),
            0x21 => Character(if shift_pressed { 'F' } else { 'f' }),
            0x22 => Character(if shift_pressed { 'G' } else { 'g' }),
            0x23 => Character(if shift_pressed { 'H' } else { 'h' }),
            0x24 => Character(if shift_pressed { 'J' } else { 'j' }),
            0x25 => Character(if shift_pressed { 'K' } else { 'k' }),
            0x26 => Character(if shift_pressed { 'L' } else { 'l' }),
            0x27 => Character(if shift_pressed { ':' } else { ';' }),
            0x28 => Character(if shift_pressed { '"' } else { '\'' }),
            0x29 => Character(if shift_pressed { '~' } else { '`' }),
            0x2B => Character(if shift_pressed { '|' } else { '\\' }),
            0x2C => Character(if shift_pressed { 'Z' } else { 'z' }),
            0x2D => Character(if shift_pressed { 'X' } else { 'x' }),
            0x2E => Character(if shift_pressed { 'C' } else { 'c' }),
            0x2F => Character(if shift_pressed { 'V' } else { 'v' }),
            0x30 => Character(if shift_pressed { 'B' } else { 'b' }),
            0x31 => Character(if shift_pressed { 'N' } else { 'n' }),
            0x32 => Character(if shift_pressed { 'M' } else { 'm' }),
            0x33 => Character(if shift_pressed { '<' } else { ',' }),
            0x34 => Character(if shift_pressed { '>' } else { '.' }),
            0x35 => Character(if shift_pressed { '?' } else { '/' }),

            // Numpad
            0x39 => Character(' '),
            0x47 => Character('7'),
            0x48 => Character('8'),
            0x49 => Character('9'),
            0x4B => Character('4'),
            0x4C => Character('5'),
            0x4D => Character('6'),
            0x4F => Character('1'),
            0x50 => Character('2'),
            0x51 => Character('3'),
            0x52 => Character('0'),
            0x53 => Character('.'),
            0x37 => Character('*'),
            0x4A => Character('-'),
            0x4E => Character('+'),
            0x5C => Character('/'),

            // Named keys
            0x01 => Named(NamedKey::Escape),
            0x0E => Named(NamedKey::Backspace),
            0x0F => Named(NamedKey::Tab),
            0x1C => Named(NamedKey::Enter),
            0x1D => Named(NamedKey::Control),
            0x2A => Named(NamedKey::Shift),
            0x36 => Named(NamedKey::Shift),
            0x38 => Named(NamedKey::Alt),
            0x3A => Named(NamedKey::CapsLock),
            0x45 => Named(NamedKey::NumLock),
            0x46 => Named(NamedKey::ScrollLock),

            // Function keys
            0x3B => Named(NamedKey::F1),
            0x3C => Named(NamedKey::F2),
            0x3D => Named(NamedKey::F3),
            0x3E => Named(NamedKey::F4),
            0x3F => Named(NamedKey::F5),
            0x40 => Named(NamedKey::F6),
            0x41 => Named(NamedKey::F7),
            0x42 => Named(NamedKey::F8),
            0x43 => Named(NamedKey::F9),
            0x44 => Named(NamedKey::F10),
            0x57 => Named(NamedKey::F11),
            0x58 => Named(NamedKey::F12),

            // Unhandled keys
            _ => Character(scan_code as char),
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

    pub fn handle_scancode(&mut self, scancode: u8) -> Option<Key> {
        if scancode == 0xE0 {
            self.extended = true;
            return None;
        }

        let is_release = scancode & 0x80 != 0;
        let scan_code = scancode & 0x7F; // Clear the high bit

        let key = Key::from_scan_code(scan_code, self.shift_pressed);

        match key {
            Named(NamedKey::Control) => self.ctrl_pressed = !is_release,
            Named(NamedKey::Shift) => self.shift_pressed = !is_release,
            Named(NamedKey::Alt) => self.alt_pressed = !is_release,
            Named(NamedKey::CapsLock) => {
                if !is_release {
                    self.caps_lock = !self.caps_lock;
                    self.update_leds();
                }
            }
            Named(NamedKey::NumLock) => {
                if !is_release {
                    self.num_lock = !self.num_lock;
                    self.update_leds();
                }
            }
            Named(NamedKey::ScrollLock) => {
                if !is_release {
                    self.scroll_lock = !self.scroll_lock;
                    self.update_leds();
                }
            }
            _ => {}
        }

        if is_release {
            self.extended = false;
            return None;
        }

        Some(key)
    }

    fn update_leds(&self) {
        while inb(KEYBOARD_STATUS_PORT) & 2 != 0 {}
        outb(KEYBOARD_DATA_PORT, 0xED);

        while inb(KEYBOARD_STATUS_PORT) & 2 != 0 {}
        let led_state = ((self.scroll_lock as u8) << 0)
            | ((self.num_lock as u8) << 1)
            | ((self.caps_lock as u8) << 2);
        outb(KEYBOARD_DATA_PORT, led_state);
    }
}

pub fn initialize_keyboard() {
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

pub fn read_scancode() -> Option<u8> {
    if inb(KEYBOARD_STATUS_PORT) & 1 != 0 {
        Some(inb(KEYBOARD_DATA_PORT))
    } else {
        None
    }
}
