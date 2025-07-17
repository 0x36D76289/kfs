use crate::drivers::vga;
use crate::drivers::serial;
use core::fmt::{self, Write};

pub struct PrintkWriter;

impl Write for PrintkWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        vga::print(s);
        serial::write_string(s);
        Ok(())
    }
}

pub fn printk(args: fmt::Arguments) {
    let mut writer = PrintkWriter;
    writer.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! printk {
    ($($arg:tt)*) => {
        $crate::kernel::printk::printk(format_args!($($arg)*))
    };
}
