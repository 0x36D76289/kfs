use crate::vga::VGA_HEIGHT;
use crate::{printk, printkln};
use crate::keyboard::KeyEvent;
use crate::screens;
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
        printkln!("KFS Shell initialized");
        printkln!("Type 'help' for a list of commands");
        self.display_prompt();
    }
    
    fn display_prompt(&self) {
        printk!("{}", self.prompt);
    }
    
    pub fn handle_keypress(&mut self, key_event: KeyEvent) {
        if key_event.is_function_key() {
            if let Some(fnum) = key_event.function_key_num() {
                if fnum >= 1 && fnum <= screens::MAX_SCREENS as u8 {
                    let screen_idx = (fnum - 1) as usize;

                    if screens::switch_to_screen(screen_idx) {
                        self.display_prompt();
                    }
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
                    printk!("{}", ascii_digit as char);
                }
            }
            return;
        }
        
        // Handle regular keys
        match key_event.key {
            b'\n' => {
                printkln!();
                self.execute_command();
                self.buffer_pos = 0;
                for i in 0..MAX_CMD_LENGTH {
                    self.buffer[i] = 0;
                }
                self.display_prompt();
            },
            b'\x08' => { // Backspace
                if self.buffer_pos > 0 {
                    self.buffer_pos -= 1;
                    self.buffer[self.buffer_pos] = 0;
                    printk!("\x08 \x08");
                }
            },
            _ => {
                if self.buffer_pos < MAX_CMD_LENGTH - 1 {
                    self.buffer[self.buffer_pos] = key_event.key;
                    self.buffer_pos += 1;
                    printk!("{}", key_event.key as char);
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
            "42" => printkln!("The answer to life, the universe, and everything!"),
            "" => {},
            _ => printkln!("Unknown command: {}", cmd_str),
        }
    }
    
    fn cmd_help(&self) {
        printkln!("Available commands:");
        printkln!("  help       - Display this help message");
        printkln!("  clear      - Clear the screen");
        printkln!("  info       - Display system information");
        printkln!("  stacktrace - Display kernel stack trace");
        printkln!("  reboot     - Reboot the system");
        printkln!("  halt       - Halt the system");
        printkln!("  42         - Display the answer");
    }
    
    fn cmd_clear(&self) {
        for _ in 0..VGA_HEIGHT {
            printkln!();
        }
    }
    
    fn cmd_info(&self) {
        printkln!("KFS - Kernel From Scratch");
        printkln!("Version: 0.1.0");
        printkln!("Memory: Unknown");
    }
    
    fn cmd_stacktrace(&self) {
        printkln!("Stack trace:");
        unsafe {
            let mut frame_ptr: usize;
            asm!("mov {}, ebp", out(reg) frame_ptr);
            
            printkln!("  Frame pointer: 0x{:x}", frame_ptr);

            // TODO: Implement stack trace logic
        }
    }
    
    fn cmd_reboot(&self) {
        printkln!("Rebooting system...");
        unsafe {
            while inb(0x64) & 2 != 0 {}
            outb(0x64, 0xFE);
            
            printkln!("Reboot failed!");
        }
    }
    
    fn cmd_halt(&self) {
        printkln!("System halted.");
        unsafe {
            loop {
                asm!("hlt", options(nomem, nostack));
            }
        }
    }
}

unsafe fn inb(port: u16) -> u8 {
    let result: u8;
    unsafe {
        asm!("in al, dx", out("al") result, in("dx") port, options(nomem, nostack));
    }
    result
}

unsafe fn outb(port: u16, value: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack));
    }
}
