use crate::screen::Screen;
use crate::vga::{Color, ColorCode};
use core::fmt;
use core::sync::atomic::{AtomicPtr, Ordering};

static KERNEL_SCREEN: AtomicPtr<Screen> = AtomicPtr::new(core::ptr::null_mut());

pub fn init_printk(screen: *mut Screen) {
    KERNEL_SCREEN.store(screen, Ordering::SeqCst);
}

pub fn _putchar(c: u8) {
    let screen_ptr = KERNEL_SCREEN.load(Ordering::SeqCst);
    if !screen_ptr.is_null() {
        unsafe {
            (*screen_ptr).write_char(c);
        }
    }
}

pub fn _puts(s: &str) {
    let screen_ptr = KERNEL_SCREEN.load(Ordering::SeqCst);
    if !screen_ptr.is_null() {
        unsafe {
            (*screen_ptr).write_string(s);
        }
    }
}

struct PrintkWriter;

impl fmt::Write for PrintkWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        _puts(s);
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    PrintkWriter.write_fmt(args).unwrap();
}

pub fn printk_color(fg: Color, bg: Color, s: &str) {
    let screen_ptr = KERNEL_SCREEN.load(Ordering::SeqCst);
    if !screen_ptr.is_null() {
        unsafe {
            let current_color = (*screen_ptr).color;

            (*screen_ptr).set_color(ColorCode::new(fg, bg));
            (*screen_ptr).write_string(s);
            (*screen_ptr).set_color(current_color);
        }
    }
}

pub fn print_error(s: &str) {
    printk_color(Color::White, Color::Red, s);
}

pub fn print_warning(s: &str) {
    printk_color(Color::Black, Color::Yellow, s);
}
