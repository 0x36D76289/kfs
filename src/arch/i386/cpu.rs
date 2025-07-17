use core::arch::asm;

pub fn init() {
    // Initialisation CPU spécifique i386
}

#[allow(dead_code)]
pub fn halt() {
    unsafe {
        asm!("hlt", options(nomem, nostack)); // Halt the CPU until the next interrupt
    }
}

#[allow(dead_code)]
pub fn disable_interrupts() {
    unsafe {
        asm!("cli", options(nomem, nostack)); // Disable interrupts
    }
}

#[allow(dead_code)]
pub fn enable_interrupts() {
    unsafe {
        asm!("sti", options(nomem, nostack)); // Enable interrupts
    }
}
