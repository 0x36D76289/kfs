use crate::io::{inb, outb};
use crate::keyboard::{Key, Key::Character, Key::Named, NamedKey};
use crate::screen;
use crate::{print, println};
use core::arch::asm;

const MAX_CMD_LENGTH: usize = 256;

pub struct Shell {
    prompt: &'static str,
    buffer: [u8; MAX_CMD_LENGTH],
    buffer_pos: usize,
}

impl Shell {
    pub fn new() -> Shell {
        Shell {
            prompt: "kfs> ",
            buffer: [0; MAX_CMD_LENGTH],
            buffer_pos: 0,
        }
    }

    pub fn init(&mut self) {
        println!("KFS Shell initialized");
        println!("Type 'help' for a list of commands");
        self.display_prompt();
    }

    fn display_prompt(&self) {
        print!("{}", self.prompt);
    }

    fn switch_to_screen(&self, screen_id: usize) {
        screen::switch_to_screen(screen_id);
        self.display_prompt();
    }

    fn write_char(&mut self, c: char) {
        if self.buffer_pos < MAX_CMD_LENGTH - 1 {
            self.buffer[self.buffer_pos] = c as u8;
            self.buffer_pos += 1;
            print!("{}", c);
        }
    }

    pub fn handle_keypress(&mut self, key: Key) {
        match key {
            Named(NamedKey::F1) => self.switch_to_screen(0),
            Named(NamedKey::F2) => self.switch_to_screen(1),
            Named(NamedKey::F3) => self.switch_to_screen(2),
            Named(NamedKey::F4) => self.switch_to_screen(3),
            Named(NamedKey::Backspace) => {
                if self.buffer_pos > 0 {
                    self.buffer_pos -= 1;
                    self.buffer[self.buffer_pos] = 0;
                    print!("\x08 \x08");
                }
            }
            Named(NamedKey::Enter) => {
                println!();
                self.execute_command();
                self.buffer_pos = 0;
                for i in 0..MAX_CMD_LENGTH {
                    self.buffer[i] = 0;
                }
                self.display_prompt();
            }
            Named(NamedKey::Tab) => {
                for _ in 0..4 {
                    self.write_char(' ')
                }
            }
            Character(c) => self.write_char(c),
            _ => {}
        }
    }

    fn execute_command(&mut self) {
        let cmd_str = core::str::from_utf8(&self.buffer[0..self.buffer_pos])
            .unwrap_or("Invalid UTF-8")
            .trim();

        match cmd_str {
            "help" => self.cmd_help(),
            "clear" => self.cmd_clear(),
            "info" => self.cmd_info(),
            "stacktrace" => self.cmd_stacktrace(),
            "reboot" => self.cmd_reboot(),
            "halt" => self.cmd_halt(),
            "42" => println!("The answer to life, the universe, and everything!"),
            "" => {}
            _ => println!("Unknown command: {}", cmd_str),
        }
    }

    fn cmd_help(&self) {
        println!("Available commands:");
        println!("  help       - Display this help message");
        println!("  clear      - Clear the screen");
        println!("  info       - Display system information");
        println!("  stacktrace - Display kernel stack trace");
        println!("  reboot     - Reboot the system");
        println!("  halt       - Halt the system");
        println!("  42         - Display the answer");
    }

    fn cmd_clear(&self) {
        screen::clear_screen();
    }

    fn cmd_info(&self) {
        println!("KFS - Kernel From Scratch");
        println!("Version: 0.1.0");
        println!("Memory: Unknown");
    }

    fn cmd_stacktrace(&self) {
        println!("Stack trace:");
        let mut frame_ptr: usize;

        unsafe { asm!("mov {}, ebp", out(reg) frame_ptr) }

        println!("  Frame pointer: 0x{:x}", frame_ptr);

        // TODO: Implement stack trace logic
    }

    fn cmd_reboot(&self) {
        println!("Rebooting system...");
        while inb(0x64) & 2 != 0 {}
        outb(0x64, 0xFE);

        println!("Reboot failed!");
    }

    fn cmd_halt(&self) {
        println!("System halted.");
        loop {
            unsafe { asm!("hlt", options(nomem, nostack)) }
        }
    }
}
