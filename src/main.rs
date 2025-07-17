#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod arch;
mod kernel;
mod drivers;
mod utils;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    drivers::serial::init();
    drivers::vga::init();
    kernel::gdt::init();
    kernel::idt::init();
    arch::i386::init();
    
    printk!("GDT initialized successfully!\n");
    printk!("Kernel stack analysis:\n");
    
    kernel::stack::print_kernel_stack();
    kernel::stack::print_stack_trace();
    
    drivers::serial::write_string("Architecture initialized\n");
    
    loop {
        arch::i386::cpu::halt();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    printk!("KERNEL PANIC: {}\n", _info);
    kernel::stack::print_kernel_stack();
    loop {
        arch::i386::cpu::halt();
    }
}
