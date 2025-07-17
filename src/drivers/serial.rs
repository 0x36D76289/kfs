// https://wiki.osdev.org/Serial_Ports
use crate::arch::i386::io::{inb, outb};

const SERIAL_PORT: u16 = 0x3f8;

pub fn init() {
    // Initialize serial port
    outb(SERIAL_PORT + 1, 0x00); // Disable all interrupts
    outb(SERIAL_PORT + 3, 0x80); // Enable DLAB (set baud rate divisor)
    outb(SERIAL_PORT + 0, 0x03); // Set divisor to 3 (lo byte) 38400 baud
    outb(SERIAL_PORT + 1, 0x00); //                  (hi byte)
    outb(SERIAL_PORT + 3, 0x03); // 8 bits, no parity, one stop bit
    outb(SERIAL_PORT + 2, 0xC7); // Enable FIFO, clear them, with 14-byte threshold
    outb(SERIAL_PORT + 4, 0x0B); // IRQs enabled, RTS/DSR set
}

pub fn write_char(c: char) {
    // Wait for transmit holding register to be empty
    while (inb(SERIAL_PORT + 5) & 0x20) == 0 {}
    outb(SERIAL_PORT, c as u8);
}

pub fn write_string(s: &str) {
    for c in s.chars() {
        write_char(c);
    }
}