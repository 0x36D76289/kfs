use crate::keyboard::{KeyboardState, initialize_keyboard, read_scancode};
use crate::shell::Shell;
use crate::screen::{ColorCode, Color, WRITER};
use crate::println;

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    println!("42");
    println!("KFS - Kernel From Scratch");
    println!("Version 0.1.0");
    println!("----------------------------");

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
    WRITER.lock().set_color(ColorCode::new(Color::Red, Color::Black));
    println!("\n{info}");
    
    loop {
        unsafe {
            core::arch::asm!("hlt", options(nomem, nostack));
        }
    }
}
