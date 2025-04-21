use crate::vga::{ColorCode, Color, clear_screen};
use crate::screen::Screen;
use crate::printk::{init_printk, print_error};
use crate::keyboard::{KeyboardState, initialize_keyboard, read_scancode};
use crate::shell::Shell;
use crate::{printk, printkln};

// Kernel entry point - exported for boot.asm to call
#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    // Initialize hardware
    unsafe {
        // Clear screen with default colors
        clear_screen(ColorCode::new(Color::White, Color::Black));
    }
    
    // Create and activate the screen
    let mut screen = Screen::new(ColorCode::new(Color::White, Color::Black));
    screen.activate();
    
    // Initialize printk with our screen
    init_printk(&mut screen as *mut Screen);
    
    // Print welcome message
    printkln!("KFS - Kernel From Scratch");
    printkln!("Version 0.1.0");
    printkln!("----------------------------");
    
    // Initialize keyboard
    initialize_keyboard();
    let mut keyboard_state = KeyboardState::new();
    
    // Initialize shell
    let mut shell = Shell::new();
    shell.init();
    
    // Main loop
    loop {
        // Check for keyboard input
        if let Some(scancode) = read_scancode() {
            if let Some(key_event) = keyboard_state.handle_scancode(scancode) {
                shell.handle_keypress(key_event);
            }
        }
        
        // Simple CPU yield (could use hlt instruction in a real kernel)
        unsafe {
            core::arch::asm!("pause");
        }
    }
}

// Panic handler - required for no_std
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    print_error("\nKERNEL PANIC: ");
    
    // Print message directly without trying to use Option pattern
    if let Some(location) = info.location() {
        printk!("at {}:{}:{}", location.file(), location.line(), location.column());
    }
    
    // Use regular let instead of if let since info.message() always returns a value
    let message = info.message();
    printk!(" {}", message);
    
    printkln!("\nSystem halted");
    
    // Halt the CPU
    loop {
        unsafe {
            core::arch::asm!("hlt", options(nomem, nostack));
        }
    }
}
