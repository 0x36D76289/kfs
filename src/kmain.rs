use crate::gdt;
use crate::keyboard::{KeyboardState, initialize_keyboard, read_scancode};
use crate::println;
use crate::screen::{self, Color, ColorCode};
use crate::shell::Shell;
use core::arch::asm;

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    println!("42");
    println!("KFS - Kernel From Scratch");
    println!("Version 0.1.0");
    println!("----------------------------");

    // Initialize GDT first
    gdt::init_gdt();

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

        unsafe { asm!("pause") }
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    screen::change_color_code(ColorCode::new(Color::White, Color::Red));
    println!("\n{info}");

    loop {
        unsafe { asm!("hlt", options(nomem, nostack)) }
    }
}
