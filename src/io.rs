use core::arch::asm;

// Output a byte to a port
pub fn outb(port: u16, value: u8) {
    unsafe { asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack)) }
}

// Input a byte from a port
pub fn inb(port: u16) -> u8 {
    let result: u8;
    unsafe { asm!("in al, dx", out("al") result, in("dx") port, options(nomem, nostack)) }
    result
}

// Wait for a short time using I/O port
pub fn io_wait() {
    outb(0x80, 0);
}
