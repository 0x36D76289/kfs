#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use kfs::vga_buffer::{Color, set_color, clear_screen};

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    clear_screen();
    set_color(Color::White, Color::Black);
    
    kfs::init();
    kfs::keyboard::init();
    
    x86_64::instructions::interrupts::enable();
    
    kfs::shell::start_shell();
    
    loop {
        x86_64::instructions::hlt();
    }
}

// fn demo_features() {
//     set_color(Color::LightGreen, Color::Black);
//     kfs::println!("=== KFS Enhanced I/O Interface Demo ===");
//     set_color(Color::White, Color::Black);
    
//     kfs::println!();
//     set_color(Color::Yellow, Color::Black);
//     kfs::println!("1. Printf-like functions:");
//     set_color(Color::White, Color::Black);
    
//     kfs::kprintf!("Testing kprintf: %s = %d\n", "answer", 42);
//     kfs::debug_print!("Debug message with number: %d", 100);
//     kfs::error_print!("Error message example");
    
//     kfs::println!();
//     set_color(Color::Yellow, Color::Black);
//     kfs::println!("2. Cursor control:");
//     set_color(Color::White, Color::Black);
    
//     let (row, col) = kfs::vga_buffer::get_cursor_position();
//     kfs::println!("Current cursor position: row {}, col {}", row, col);
    
//     kfs::vga_buffer::set_cursor_position(10, 20);
//     kfs::print!("Text at (10,20)");
//     kfs::vga_buffer::set_cursor_position(row + 3, 0);
    
//     kfs::println!();
//     set_color(Color::Yellow, Color::Black);
//     kfs::println!("3. Color support:");
    
//     let colors = [
//         (Color::Red, "RED"),
//         (Color::Green, "GREEN"), 
//         (Color::Blue, "BLUE"),
//         (Color::Cyan, "CYAN"),
//         (Color::Magenta, "MAGENTA"),
//         (Color::Yellow, "YELLOW"),
//     ];
    
//     for (color, name) in &colors {
//         set_color(*color, Color::Black);
//         kfs::print!("{} ", name);
//     }
//     set_color(Color::White, Color::Black);
//     kfs::println!();
    
//     kfs::println!();
//     set_color(Color::Yellow, Color::Black);
//     kfs::println!("4. Keyboard support enabled");
//     set_color(Color::White, Color::Black);
//     kfs::println!("  - Character input handling");
//     kfs::println!("  - Ctrl+L: Clear screen");
//     kfs::println!("  - Ctrl+C: Cancel line");
//     kfs::println!("  - Ctrl+D: Exit");
//     kfs::println!("  - Alt+1-9: Switch virtual screens");
    
//     kfs::println!();
//     set_color(Color::LightGreen, Color::Black);
//     kfs::println!("All enhanced features initialized successfully!");
//     set_color(Color::White, Color::Black);
//     kfs::println!();
// }

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    kernel_main()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    set_color(Color::Red, Color::Black);
    kfs::println!();
    kfs::println!("KERNEL PANIC!");
    kfs::println!("{}", info);
    
    kfs::println!();
    kfs::println!("Stack information at panic:");
    kfs::gdt::print_kernel_stack();
    kfs::gdt::print_call_stack();
    
    loop {
        x86_64::instructions::hlt();
    }
}
