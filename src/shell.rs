use crate::vga::VGA_HEIGHT;
use crate::{printk, printkln};
use crate::keyboard::KeyEvent;
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
                    printk!("\x08 \x08"); // Backspace, space, backspace
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
        // Convert buffer to string and trim whitespace
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
        // This assumes printk uses the current screen
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
        // This is a simplified version that just prints the current frame pointer
        unsafe {
            let mut frame_ptr: usize;
            asm!("mov {}, ebp", out(reg) frame_ptr);
            
            printkln!("  Frame pointer: 0x{:x}", frame_ptr);
            
            // In a real implementation, you would walk the stack frames
            // For a simple demo, just print the current frame
        }
    }
    
    fn cmd_reboot(&self) {
        printkln!("Rebooting system...");
        unsafe {
            // Send reset command to keyboard controller
            while inb(0x64) & 2 != 0 {} // Wait for input buffer to empty
            outb(0x64, 0xFE);           // Reset command
            
            // If we get here, the reboot failed
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

// Helper function for cmd_reboot
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
