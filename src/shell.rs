use crate::keyboard::{Key, Key::Character, Key::Named, NamedKey};
use crate::screen;
use crate::{print, println};

// Maximum command length for the shell
const MAX_CMD_LENGTH: usize = 256;

// Shell structure to handle user input and commands
pub struct Shell {
    prompt: &'static str,
    buffer: [u8; MAX_CMD_LENGTH],
    buffer_pos: usize,
}

// Implement methods for the Shell
impl Shell {
    /// Create a new Shell instance
    pub fn new() -> Shell {
        Shell {
            prompt: "kfs> ",
            buffer: [0; MAX_CMD_LENGTH],
            buffer_pos: 0,
        }
    }

    /// Initialize the shell, displaying the prompt and instructions
    pub fn init(&mut self) {
        println!("KFS Shell initialized");
        println!("Type 'help' for a list of commands");
        self.display_prompt();
    }

    /// Display the shell prompt
    fn display_prompt(&self) {
        print!("{}", self.prompt);
    }

    /// Switch to a different screen by ID
    fn switch_to_screen(&self, screen_id: usize) {
        screen::switch_to_screen(screen_id);
        self.display_prompt();
    }

    /// Write a character to the shell buffer and display it
    fn write_char(&mut self, c: char) {
        if self.buffer_pos < MAX_CMD_LENGTH - 1 {
            self.buffer[self.buffer_pos] = c as u8;
            self.buffer_pos += 1;
            print!("{}", c);
        }
    }

    /// Handle keypress events, updating the shell state accordingly
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

    /// Execute the command stored in the buffer
    fn execute_command(&mut self) {
        // Convert the buffer to a string and trim whitespace
        let cmd_str = core::str::from_utf8(&self.buffer[0..self.buffer_pos])
            .unwrap_or("Invalid UTF-8")
            .trim();

        match cmd_str {
            "help" => self.cmd_help(),
            "clear" => self.cmd_clear(),
            "info" => self.cmd_info(),
            "stacktrace" => self.cmd_stacktrace(),
            "stack" => self.cmd_stack_info(),
            "gdt" => self.cmd_gdt_info(),
            "gdttest" => self.cmd_gdt_test(),
            "trigger_panic" => self.trigger_panic(),
            "reboot" => self.cmd_reboot(),
            "shutdown" => self.cmd_shutdown(),
            "halt" => self.cmd_halt(),
            "42" => println!("The answer to life, the universe, and everything!"),
            "" => {}
            _ => println!("Unknown command: {}", cmd_str),
        }
    }

    /// Command implementations
    fn cmd_help(&self) {
        println!("Available commands:");
        println!("  help          - Display this help message");
        println!("  clear         - Clear the screen");
        println!("  info          - Display system information");
        println!("  stacktrace    - Show kernel stack trace");
        println!("  stack         - Show detailed stack information");
        println!("  gdt           - Show GDT information");
        println!("  gdttest       - Test GDT functionality");
        println!("  trigger_panic - Trigger a panic for testing");
        println!("  reboot        - Reboot the system");
        println!("  shutdown      - Shutdown the system");
        println!("  halt          - Halt the system");
        println!("  42            - Display the answer");
    }

    /// Clear the screen
    fn cmd_clear(&self) {
        screen::clear_screen();
    }

    /// Display system information including stack and GDT details
    fn cmd_info(&self) {
        println!("KFS - Kernel From Scratch");
        println!("Version: 0.2.0 (KFS_2 - GDT & Stack)");
        println!("Architecture: i386 (x86)");
        
        // Show memory info
        let esp = crate::stack_trace::get_stack_pointer();
        let ebp = crate::stack_trace::get_base_pointer();
        println!("Stack Pointer: 0x{:08X}", esp);
        println!("Base Pointer:  0x{:08X}", ebp);
        
        // Show GDT info
        if let Some(gdt) = crate::gdt::get_gdt() {
            println!("GDT Base: 0x{:08X}", gdt as *const _ as u32);
            println!("GDT loaded and active");
        } else {
            println!("GDT not initialized");
        }
    }

    /// Print the current stack trace
    fn cmd_stacktrace(&self) {
        crate::stack_trace::print_stack_trace();
    }

    /// Print detailed stack information
    fn cmd_stack_info(&self) {
        crate::stack_trace::print_stack_trace_with_title("Shell Stack Trace");
    }

    /// Print GDT information
    fn cmd_gdt_info(&self) {
        if let Some(gdt) = crate::gdt::get_gdt() {
            gdt.print_gdt_info();
        } else {
            println!("GDT not initialized");
        }
    }

    /// Test GDT functionality
    fn cmd_gdt_test(&self) {
        crate::gdt::test_gdt_functionality();
    }

    /// Trigger a panic for testing purposes
    fn trigger_panic(&self) {
        println!("Triggering panic...");
        panic!("This is a test panic!");
    }

    /// Reboot the system
    fn cmd_reboot(&self) {
        crate::power::reboot();
    }

    /// Shutdown the system
    fn cmd_shutdown(&self) {
        crate::power::shutdown();
    }

    /// Halt the system
    fn cmd_halt(&self) {
        crate::power::halt();
    }
}
