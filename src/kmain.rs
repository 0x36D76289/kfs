use crate::vga::{ColorCode, Color, clear_screen};
use crate::keyboard::{KeyboardState, initialize_keyboard, read_scancode};
use crate::{printk, printkln};
use crate::printk::print_error;
use crate::shell::Shell;
use crate::screens;

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    unsafe {
        clear_screen(ColorCode::new(Color::White, Color::Black));
    }
    
    screens::init_screens();
    
    printkln!("KFS - Kernel From Scratch");
    printkln!("Version 0.1.0");
    printkln!("----------------------------");
    
    initialize_keyboard();
    let mut keyboard_state = KeyboardState::new();
    
    let mut shell = Shell::new();
    shell.init();
    
    loop {
        if let Some(scancode) = read_scancode() {
            if let Some(key_event) = keyboard_state.handle_scancode(scancode) {
                shell.handle_keypress(key_event);
            }
        }
        
        unsafe {
            core::arch::asm!("pause");
        }
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    print_error("\nKERNEL PANIC: ");
    
    if let Some(location) = info.location() {
        printk!("at {}:{}:{}", location.file(), location.line(), location.column());
    }
    
    let message = info.message();
    printk!(" {}", message);
    
    printkln!("\nSystem halted");
    
    loop {
        unsafe {
            core::arch::asm!("hlt", options(nomem, nostack));
        }
    }
}
