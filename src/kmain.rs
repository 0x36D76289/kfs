use crate::keyboard::{KeyboardState, initialize_keyboard, read_scancode};
use crate::println;
use crate::screen::{self, Color, ColorCode};
use crate::shell::Shell;
use core::arch::asm;

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    println!("42");
    println!("KFS_2 - Global Descriptor Table & Stack");
    println!("Version 2.0.0");
    println!("----------------------------");

    crate::gdt::init();
    
    println!("Initial system state:");
    crate::stack_trace::print_stack_trace_with_title("Boot Stack Trace");

    if !initialize_keyboard() {
        println!("Warning: Keyboard initialization failed!");
    } else {
        println!("Keyboard initialized successfully.");
    }
    
    let mut keyboard_state = KeyboardState::new();

    let mut shell = Shell::new();
    shell.init();

    loop {
        if let Some(scancode) = read_scancode() {
            if let Some(key_event) = keyboard_state.handle_scancode(scancode) {
                shell.handle_keypress(key_event);
            }
        }

        unsafe { asm!("pause") }
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    screen::change_color_code(ColorCode::new(Color::White, Color::Red));
    println!("\n=== KERNEL PANIC ===");
    println!("{info}");
    
    // Show stack trace on panic
    crate::stack_trace::print_stack_trace_with_title("Panic Stack Trace");
    
    // Emergency halt
    crate::power::emergency_halt();
}
