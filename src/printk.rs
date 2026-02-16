use crate::vga::{Color, ColorCode, Writer};
use core::fmt::{self, Write};

static mut KERNEL_WRITER: Option<Writer> = None;

pub fn init() {
    unsafe {
        KERNEL_WRITER = Some(Writer::new());
    }
}

fn get_writer() -> &'static mut Writer {
    unsafe {
        if KERNEL_WRITER.is_none() {
            KERNEL_WRITER = Some(Writer::new());
        }
        KERNEL_WRITER.as_mut().unwrap()
    }
}

pub fn print(s: &str) {
    get_writer().write_string(s);
}

pub fn println(s: &str) {
    let writer = get_writer();
    writer.write_string(s);
    writer.write_string("\n");
}

pub fn clear() {
    get_writer().clear_screen();
}

pub fn set_color(fg: Color, bg: Color) {
    get_writer().set_color(ColorCode::new(fg, bg));
}

pub fn reset_color() {
    get_writer().set_color(ColorCode::new(Color::White, Color::Black));
}

pub struct KernelWriter;

impl Write for KernelWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        print(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! printk {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let _ = write!($crate::printk::KernelWriter, $($arg)*);
    });
}

#[macro_export]
macro_rules! printkln {
    () => ($crate::printk::print("\n"));
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let _ = write!($crate::printk::KernelWriter, $($arg)*);
        $crate::printk::print("\n");
    });
}

pub fn print_hex(value: u32) {
    let writer = get_writer();
    writer.write_string("0x");

    let hex_chars: [u8; 16] = *b"0123456789ABCDEF";
    let mut buffer = [b'0'; 8];
    let mut v = value;

    for i in (0..8).rev() {
        buffer[i] = hex_chars[(v & 0xF) as usize];
        v >>= 4;
    }

    let mut started = false;
    for i in 0..8 {
        if buffer[i] != b'0' || i == 7 {
            started = true;
        }
        if started {
            writer.write_byte(buffer[i]);
        }
    }
}

pub fn print_hex_padded(value: u32) {
    let writer = get_writer();
    writer.write_string("0x");

    let hex_chars: [u8; 16] = *b"0123456789ABCDEF";

    for i in (0..8).rev() {
        let nibble = ((value >> (i * 4)) & 0xF) as usize;
        writer.write_byte(hex_chars[nibble]);
    }
}

pub fn print_dec(value: u32) {
    let writer = get_writer();

    if value == 0 {
        writer.write_byte(b'0');
        return;
    }

    let mut buffer = [b'0'; 10];
    let mut v = value;
    let mut i = 9;

    while v > 0 {
        buffer[i] = b'0' + (v % 10) as u8;
        v /= 10;
        if i > 0 {
            i -= 1;
        }
    }

    for j in (i + 1)..10 {
        writer.write_byte(buffer[j]);
    }
}

pub fn print_byte_hex(value: u8) {
    let writer = get_writer();
    let hex_chars: [u8; 16] = *b"0123456789ABCDEF";
    writer.write_byte(hex_chars[(value >> 4) as usize]);
    writer.write_byte(hex_chars[(value & 0xF) as usize]);
}
