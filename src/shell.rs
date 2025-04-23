use crate::keyboard::KeyEvent;
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

    pub fn handle_keypress(&mut self, key_event: KeyEvent) {
        if key_event.is_function_key() {
            if let Some(fnum) = key_event.function_key_num() {
                if fnum >= 1 && fnum <= screen::MAX_SCREEN as u8 {
                    let screen_idx = (fnum - 1) as usize;

                    screen::switch_to_screen(screen_idx);
                    self.display_prompt();
                    return;
                }
            }
            return;
        }

        if key_event.is_numpad_key() {
            if let Some(digit) = key_event.numpad_digit() {
                let ascii_digit = b'0' + digit;
                if self.buffer_pos < MAX_CMD_LENGTH - 1 {
                    self.buffer[self.buffer_pos] = ascii_digit;
                    self.buffer_pos += 1;
                    print!("{}", ascii_digit as char);
                }
            }
            return;
        }

        match key_event.key {
            b'\n' => {
                println!();
                self.execute_command();
                self.buffer_pos = 0;
                for i in 0..MAX_CMD_LENGTH {
                    self.buffer[i] = 0;
                }
                self.display_prompt();
            }
            b'\x08' => {
                // Backspace
                if self.buffer_pos > 0 {
                    self.buffer_pos -= 1;
                    self.buffer[self.buffer_pos] = 0;
                    print!("\x08 \x08");
                }
            }
            _ => {
                if self.buffer_pos < MAX_CMD_LENGTH - 1 {
                    self.buffer[self.buffer_pos] = key_event.key;
                    self.buffer_pos += 1;
                    print!("{}", key_event.key as char);
                }
            }
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
        for _ in 0..screen::VGA_BUFFER_HEIGHT {
            println!();
        }
    }

    fn cmd_info(&self) {
        println!("KFS - Kernel From Scratch");
        println!("Version: 0.1.0");
        println!("Memory: Unknown");
    }

    fn cmd_stacktrace(&self) {
        println!("Stack trace:");
        let mut frame_ptr: usize;

        unsafe {
            asm!("mov {}, ebp", out(reg) frame_ptr)
        }

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
            unsafe {
                asm!("hlt", options(nomem, nostack))
            }
        }
    }
}

fn inb(port: u16) -> u8 {
    let result: u8;
    unsafe {
        asm!("in al, dx", out("al") result, in("dx") port, options(nomem, nostack))
    }
    result
}

fn outb(port: u16, value: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack))
    }
}
