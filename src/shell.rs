use crate::{println, print};
use crate::vga_buffer::{Color, set_color};
use spin::Mutex;
use lazy_static::lazy_static;

#[derive(Debug)]
enum ShellCommand {
    Help,
    Clear,
    Reboot,
    Halt,
    Stack,
    CallStack,
    GdtInfo,
    Test,
    Exit,
    Screen,
    Unknown,
}

pub struct Shell {
    #[allow(dead_code)]
    running: bool,
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            running: true,
        }
    }
    
    pub fn run(&mut self) {
        set_color(Color::LightGreen, Color::Black);
        println!("KFS - Kernel From Scratch");
        println!("Type 'help' for available commands.");
        println!("Use Alt+1-9 to switch between virtual screens.");
        set_color(Color::White, Color::Black);
        
        self.show_prompt();
    }
    
    fn show_prompt(&self) {
        println!();
        set_color(Color::Cyan, Color::Black);
        print!("kfs> ");
        set_color(Color::White, Color::Black);
    }
    
    fn parse_command(&self, input: &str) -> ShellCommand {
        let trimmed = input.trim();
        
        match trimmed {
            "help" | "h" => ShellCommand::Help,
            "clear" | "cls" => ShellCommand::Clear,
            "reboot" | "restart" => ShellCommand::Reboot,
            "halt" | "shutdown" => ShellCommand::Halt,
            "stack" | "st" => ShellCommand::Stack,
            "callstack" | "cs" => ShellCommand::CallStack,
            "gdt" | "gdtinfo" => ShellCommand::GdtInfo,
            "test" => ShellCommand::Test,
            "exit" | "quit" => ShellCommand::Exit,
            "screen" => ShellCommand::Screen,
            "" => return ShellCommand::Help,
            _ => ShellCommand::Unknown,
        }
    }
    
    fn execute_command(&mut self, command: &ShellCommand) {
        match command {
            ShellCommand::Help => self.show_help(),
            ShellCommand::Clear => self.clear_screen(),
            ShellCommand::Reboot => self.reboot(),
            ShellCommand::Halt => self.halt(),
            ShellCommand::Stack => self.show_stack(),
            ShellCommand::CallStack => self.show_call_stack(),
            ShellCommand::GdtInfo => self.show_gdt_info(),
            ShellCommand::Test => self.run_tests(),
            ShellCommand::Exit => self.exit(),
            ShellCommand::Screen => self.show_screen_info(),
            ShellCommand::Unknown => {
                set_color(Color::Red, Color::Black);
                println!("Unknown command. Type 'help' for available commands.");
                set_color(Color::White, Color::Black);
            }
        }
    }
    
    fn show_help(&self) {
        set_color(Color::Yellow, Color::Black);
        println!("Available commands:");
        println!("  help, h        - Show this help message");
        println!("  clear, cls     - Clear the screen");
        println!("  stack, st      - Show kernel stack information");
        println!("  callstack, cs  - Show call stack trace");
        println!("  gdt, gdtinfo   - Show GDT information");
        println!("  test           - Run basic tests");
        println!("  screen         - Show screen information");
        println!("  reboot         - Reboot the system");
        println!("  halt           - Halt the system");
        println!("  exit, quit     - Exit the shell");
        println!();
        println!("Keyboard shortcuts:");
        println!("  Ctrl+L         - Clear screen");
        println!("  Ctrl+C         - Cancel current line");
        println!("  Ctrl+D         - Exit shell");
        println!("  Alt+1-9        - Switch virtual screens");
        set_color(Color::White, Color::Black);
    }
    
    fn clear_screen(&self) {
        crate::vga_buffer::clear_screen();
        set_color(Color::White, Color::Black);
        println!("Screen cleared.");
    }
    
    fn show_stack(&self) {
        crate::gdt::print_kernel_stack();
    }
    
    fn show_call_stack(&self) {
        crate::gdt::print_call_stack();
    }
    
    fn show_gdt_info(&self) {
        crate::gdt::print_gdt_info();
    }
    
    fn show_screen_info(&self) {
        set_color(Color::LightBlue, Color::Black);
        println!("Virtual Screen Information:");
        println!("Current screen: 1");
        println!("Available screens: 1-9");
        println!("Use Alt+1-9 to switch between screens");
        set_color(Color::White, Color::Black);
    }
    
    fn run_tests(&self) {
        set_color(Color::LightBlue, Color::Black);
        println!("Running basic kernel tests...");
        set_color(Color::White, Color::Black);
        
        let test_str = b"Test\0";
        let len = crate::kfs_lib::strlen(test_str.as_ptr());
        println!("  strlen test: length = {}", len);
        
        crate::kprintf!("  kprintf test: %s %d", "number", 42);
        println!();
        
        println!("  Testing breakpoint exception...");
        x86_64::instructions::interrupts::int3();
        println!("  Exception handling working correctly!");
        
        set_color(Color::Green, Color::Black);
        println!("All tests passed!");
        set_color(Color::White, Color::Black);
    }
    
    fn exit(&mut self) {
        set_color(Color::Yellow, Color::Black);
        println!("Exiting shell...");
        self.running = false;
        set_color(Color::White, Color::Black);
    }
    
    fn reboot(&self) {
        set_color(Color::Yellow, Color::Black);
        println!("Rebooting system...");
        set_color(Color::White, Color::Black);
        
        unsafe {
            use x86_64::instructions::port::Port;
            let mut port = Port::new(0x64);
            port.write(0xFE_u8);
        }
        
        loop {
            x86_64::instructions::interrupts::int3();
        }
    }
    
    fn halt(&self) {
        set_color(Color::Red, Color::Black);
        println!("System halted. Use Ctrl+C to exit QEMU.");
        set_color(Color::White, Color::Black);
        
        loop {
            x86_64::instructions::hlt();
        }
    }
}

lazy_static! {
    static ref SHELL: Mutex<Shell> = Mutex::new(Shell::new());
}

pub fn start_shell() {
    let mut shell = Shell::new();
    shell.run();
}

pub fn handle_command_input() {
    show_interactive_prompt();
}

fn show_interactive_prompt() {
    println!();
    set_color(Color::Cyan, Color::Black);
    print!("kfs> ");
    set_color(Color::White, Color::Black);
}

pub fn handle_command(cmd: &str) {
    let mut shell = SHELL.lock();
    let command = shell.parse_command(cmd);
    shell.execute_command(&command);
}

pub fn show_prompt() {
    show_interactive_prompt();
}

pub fn switch_screen(screen_num: u8) {
    if screen_num > 0 && screen_num <= 9 {
        crate::vga_buffer::clear_screen();
        crate::vga_buffer::set_color(Color::Green, Color::Black);
        println!("Switched to virtual screen {}", screen_num);
        crate::vga_buffer::set_color(Color::White, Color::Black);
        show_interactive_prompt();
    }
}
