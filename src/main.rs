#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use kfs::drivers::vga_buffer::{Color, set_color, clear_screen};

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    clear_screen();
    set_color(Color::White, Color::Black);
    
    kfs::init();
    
    x86_64::instructions::interrupts::enable();
    
    kfs::ui::shell::start_shell();
    
    loop {
        x86_64::instructions::hlt();
    }
}

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
    kfs::arch::x86_64::gdt::print_kernel_stack();
    kfs::arch::x86_64::gdt::print_call_stack();
    
    loop {
        x86_64::instructions::hlt();
    }
}
