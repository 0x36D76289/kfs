#![no_std]

pub mod vga;
pub mod screen;
pub mod keyboard;
pub mod printk;
pub mod io;
pub mod shell;
pub mod kmain;
pub mod screens;

#[macro_export]
macro_rules! printk {
    ($($arg:tt)*) => ($crate::printk::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! printkln {
    () => ($crate::printk!("\n"));
    ($($arg:tt)*) => ($crate::printk!("{}\n", format_args!($($arg)*)));
}
