use x86_64::instructions::port::Port;
use spin::Mutex;
use lazy_static::lazy_static;
use crate::{print, println};

/// PS/2 keyboard data port
const KEYBOARD_DATA_PORT: u16 = 0x60;

/// PS/2 keyboard status/command port
const KEYBOARD_STATUS_PORT: u16 = 0x64;

const BUFFER_SIZE: usize = 64;

pub struct Keyboard {
    data_port: Port<u8>,
    status_port: Port<u8>,
    shift_pressed: bool,
    ctrl_pressed: bool,
    alt_pressed: bool,
    caps_lock: bool,
    buffer: [u8; BUFFER_SIZE],
    buffer_start: usize,
    buffer_end: usize,
    #[allow(dead_code)]
    input_line: [u8; 256],
    #[allow(dead_code)]
    input_pos: usize,
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            data_port: Port::new(KEYBOARD_DATA_PORT),
            status_port: Port::new(KEYBOARD_STATUS_PORT),
            shift_pressed: false,
            ctrl_pressed: false,
            alt_pressed: false,
            caps_lock: false,
            buffer: [0; BUFFER_SIZE],
            buffer_start: 0,
            buffer_end: 0,
            input_line: [0; 256],
            input_pos: 0,
        }
    }

    pub fn read_scancode(&mut self) -> Option<u8> {
        unsafe {
            if self.status_port.read() & 1 != 0 {
                Some(self.data_port.read())
            } else {
                None
            }
        }
    }

    fn add_to_buffer(&mut self, scancode: u8) {
        let next_end = (self.buffer_end + 1) % BUFFER_SIZE;
        if next_end != self.buffer_start {
            self.buffer[self.buffer_end] = scancode;
            self.buffer_end = next_end;
        }
    }

    pub fn get_from_buffer(&mut self) -> Option<u8> {
        if self.buffer_start != self.buffer_end {
            let scancode = self.buffer[self.buffer_start];
            self.buffer_start = (self.buffer_start + 1) % BUFFER_SIZE;
            Some(scancode)
        } else {
            None
        }
    }

    pub fn handle_interrupt(&mut self) {
        if let Some(scancode) = self.read_scancode() {
            self.add_to_buffer(scancode);
            self.process_scancode(scancode);
        }
    }

    fn process_scancode(&mut self, scancode: u8) {
        match scancode {
            // Key press events
            0x2A | 0x36 => self.shift_pressed = true,  // Left/Right Shift
            0x1D => self.ctrl_pressed = true,          // Ctrl
            0x38 => self.alt_pressed = true,           // Alt
            0x3A => self.caps_lock = !self.caps_lock,  // Caps Lock

            // Key release events (scancode + 0x80)
            0xAA | 0xB6 => self.shift_pressed = false, // Left/Right Shift release
            0x9D => self.ctrl_pressed = false,         // Ctrl release
            0xB8 => self.alt_pressed = false,          // Alt release

            // Character keys
            scancode if scancode < 0x80 => {
                if let Some(character) = self.scancode_to_char(scancode) {
                    self.handle_character(character);
                }
            }
            _ => {}
        }
    }

    fn scancode_to_char(&self, scancode: u8) -> Option<char> {
        let is_uppercase = self.shift_pressed ^ self.caps_lock;
        
        match scancode {
            // Numbers row
            0x02 => Some(if self.shift_pressed { '!' } else { '1' }),
            0x03 => Some(if self.shift_pressed { '@' } else { '2' }),
            0x04 => Some(if self.shift_pressed { '#' } else { '3' }),
            0x05 => Some(if self.shift_pressed { '$' } else { '4' }),
            0x06 => Some(if self.shift_pressed { '%' } else { '5' }),
            0x07 => Some(if self.shift_pressed { '^' } else { '6' }),
            0x08 => Some(if self.shift_pressed { '&' } else { '7' }),
            0x09 => Some(if self.shift_pressed { '*' } else { '8' }),
            0x0A => Some(if self.shift_pressed { '(' } else { '9' }),
            0x0B => Some(if self.shift_pressed { ')' } else { '0' }),

            // Letters
            0x10 => Some(if is_uppercase { 'Q' } else { 'q' }),
            0x11 => Some(if is_uppercase { 'W' } else { 'w' }),
            0x12 => Some(if is_uppercase { 'E' } else { 'e' }),
            0x13 => Some(if is_uppercase { 'R' } else { 'r' }),
            0x14 => Some(if is_uppercase { 'T' } else { 't' }),
            0x15 => Some(if is_uppercase { 'Y' } else { 'y' }),
            0x16 => Some(if is_uppercase { 'U' } else { 'u' }),
            0x17 => Some(if is_uppercase { 'I' } else { 'i' }),
            0x18 => Some(if is_uppercase { 'O' } else { 'o' }),
            0x19 => Some(if is_uppercase { 'P' } else { 'p' }),
            0x1E => Some(if is_uppercase { 'A' } else { 'a' }),
            0x1F => Some(if is_uppercase { 'S' } else { 's' }),
            0x20 => Some(if is_uppercase { 'D' } else { 'd' }),
            0x21 => Some(if is_uppercase { 'F' } else { 'f' }),
            0x22 => Some(if is_uppercase { 'G' } else { 'g' }),
            0x23 => Some(if is_uppercase { 'H' } else { 'h' }),
            0x24 => Some(if is_uppercase { 'J' } else { 'j' }),
            0x25 => Some(if is_uppercase { 'K' } else { 'k' }),
            0x26 => Some(if is_uppercase { 'L' } else { 'l' }),
            0x2C => Some(if is_uppercase { 'Z' } else { 'z' }),
            0x2D => Some(if is_uppercase { 'X' } else { 'x' }),
            0x2E => Some(if is_uppercase { 'C' } else { 'c' }),
            0x2F => Some(if is_uppercase { 'V' } else { 'v' }),
            0x30 => Some(if is_uppercase { 'B' } else { 'b' }),
            0x31 => Some(if is_uppercase { 'N' } else { 'n' }),
            0x32 => Some(if is_uppercase { 'M' } else { 'm' }),

            // Special keys
            0x39 => Some(' '),  // Space
            0x1C => Some('\n'), // Enter
            0x0E => Some('\x08'), // Backspace
            0x0F => Some('\t'), // Tab

            // Symbols
            0x0C => Some(if self.shift_pressed { '_' } else { '-' }),
            0x0D => Some(if self.shift_pressed { '+' } else { '=' }),
            0x1A => Some(if self.shift_pressed { '{' } else { '[' }),
            0x1B => Some(if self.shift_pressed { '}' } else { ']' }),
            0x2B => Some(if self.shift_pressed { '|' } else { '\\' }),
            0x27 => Some(if self.shift_pressed { ':' } else { ';' }),
            0x28 => Some(if self.shift_pressed { '"' } else { '\'' }),
            0x29 => Some(if self.shift_pressed { '~' } else { '`' }),
            0x33 => Some(if self.shift_pressed { '<' } else { ',' }),
            0x34 => Some(if self.shift_pressed { '>' } else { '.' }),
            0x35 => Some(if self.shift_pressed { '?' } else { '/' }),

            _ => None,
        }
    }

    fn handle_character(&mut self, character: char) {
        if self.ctrl_pressed {
            match character {
                'l' | 'L' => {
                    // Ctrl+L - Clear screen
                    crate::vga_buffer::clear_screen();
                    crate::shell::show_prompt();
                    return;
                }
                'c' | 'C' => {
                    // Ctrl+C - Cancel current line
                    self.input_pos = 0;
                    self.input_line.fill(0);
                    println!("^C");
                    crate::shell::show_prompt();
                    return;
                }
                'd' | 'D' => {
                    // Ctrl+D - Exit/EOF
                    println!("^D");
                    crate::shell::handle_command("exit");
                    return;
                }
                _ => {}
            }
        }

        if self.alt_pressed {
            match character {
                '1'..='9' => {
                    // Alt+1-9 - Switch virtual terminals
                    let screen_num = character as u8 - b'0';
                    crate::shell::switch_screen(screen_num);
                    return;
                }
                _ => {}
            }
        }

        match character {
            '\n' => {
                println!();
                if self.input_pos > 0 {
                    // Null-terminate and process
                    self.input_line[self.input_pos] = 0;
                    let cmd_str = unsafe {
                        core::str::from_utf8_unchecked(&self.input_line[..self.input_pos])
                    };
                    crate::shell::handle_command(cmd_str);
                }
                self.input_pos = 0;
                self.input_line.fill(0);
                crate::shell::show_prompt();
            }
            '\x08' => {
                // Backspace
                if self.input_pos > 0 {
                    self.input_pos -= 1;
                    self.input_line[self.input_pos] = 0;
                    print!("\x08 \x08");
                }
            }
            '\t' => {
                // Tab completion (basic)
                print!("    "); // For now, just insert spaces
                if self.input_pos < self.input_line.len() - 4 {
                    for _ in 0..4 {
                        self.input_line[self.input_pos] = b' ';
                        self.input_pos += 1;
                    }
                }
            }
            c if c.is_ascii() && self.input_pos < self.input_line.len() - 1 => {
                self.input_line[self.input_pos] = c as u8;
                self.input_pos += 1;
                print!("{}", c);
            }
            _ => {
            }
        }
    }

    pub fn get_input_line(&self) -> &str {
        unsafe {
            core::str::from_utf8_unchecked(&self.input_line[..self.input_pos])
        }
    }

    pub fn clear_input_line(&mut self) {
        self.input_pos = 0;
        self.input_line.fill(0);
    }
}

lazy_static! {
    pub static ref KEYBOARD: Mutex<Keyboard> = Mutex::new(Keyboard::new());
}

pub fn init() {
}

pub fn handle_keyboard_interrupt() {
    KEYBOARD.lock().handle_interrupt();
}

pub fn get_char() -> Option<char> {
    let mut keyboard = KEYBOARD.lock();
    if let Some(scancode) = keyboard.get_from_buffer() {
        keyboard.scancode_to_char(scancode)
    } else {
        None
    }
}

pub fn wait_for_char() -> char {
    loop {
        if let Some(c) = get_char() {
            return c;
        }
        x86_64::instructions::hlt();
    }
}

pub fn read_line(buffer: &mut [u8]) -> usize {
    let mut pos = 0;
    
    loop {
        let c = wait_for_char();
        match c {
            '\n' => {
                println!();
                break;
            }
            '\x08' => {
                // Backspace
                if pos > 0 {
                    pos -= 1;
                    print!("\x08 \x08");
                }
            }
            c if c.is_ascii() && pos < buffer.len() - 1 => {
                buffer[pos] = c as u8;
                pos += 1;
                print!("{}", c);
            }
            _ => {}
        }
    }
    
    buffer[pos] = 0; // Null terminate
    pos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn test_keyboard_creation() {
        let keyboard = Keyboard::new();
        assert_eq!(keyboard.shift_pressed, false);
        assert_eq!(keyboard.ctrl_pressed, false);
        assert_eq!(keyboard.caps_lock, false);
    }

    #[test_case]
    fn test_scancode_conversion() {
        let keyboard = Keyboard::new();
        assert_eq!(keyboard.scancode_to_char(0x1E), Some('a'));
        assert_eq!(keyboard.scancode_to_char(0x39), Some(' '));
        assert_eq!(keyboard.scancode_to_char(0x1C), Some('\n'));
    }
}
