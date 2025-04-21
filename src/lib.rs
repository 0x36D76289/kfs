#![no_std]

pub mod io;
pub mod keyboard;
pub mod kmain;
pub mod printk;
pub mod screen;
pub mod shell;
pub mod vga;

#[macro_export]
macro_rules! printk {
    ($($arg:tt)*) => ($crate::printk::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! printkln {
    () => ($crate::printk!("\n"));
    ($($arg:tt)*) => ($crate::printk!("{}\n", format_args!($($arg)*)));
}
