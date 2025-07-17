pub mod boot;
pub mod interrupts;
pub mod memory;
pub mod cpu;
pub mod io;
pub mod gdt;

pub fn init() {
    cpu::init();
    gdt::init();
    interrupts::init();
    memory::init();
}
