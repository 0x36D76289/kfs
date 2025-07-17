// https://wiki.osdev.org/Stack
use crate::printk;

pub fn print_kernel_stack() {
    let mut stack_ptr: u32;
    
    unsafe {
        core::arch::asm!("mov {}, esp", out(reg) stack_ptr);
    }
    
    printk!("=== KERNEL STACK DUMP ===\n");
    printk!("Current ESP: 0x{:08x}\n", stack_ptr);
    printk!("Stack contents (showing 64 bytes):\n");
    
    let stack_start = stack_ptr & !0xF; // Align to 16 bytes
    
    for i in 0..4 {
        let addr = stack_start + (i * 16);
        printk!("0x{:08x}: ", addr);
        
        for j in 0..4 {
            let word_addr = addr + (j * 4);
            let value = unsafe { *(word_addr as *const u32) };
            printk!("{:08x} ", value);
        }
        printk!("\n");
    }
    
    printk!("=== END STACK DUMP ===\n");
}

pub fn print_stack_trace() {
    let mut ebp: u32;
    
    unsafe {
        core::arch::asm!("mov {}, ebp", out(reg) ebp);
    }
    
    printk!("=== STACK TRACE ===\n");
    printk!("Current EBP: 0x{:08x}\n", ebp);
    
    let mut frame_count = 0;
    let mut current_ebp = ebp;
    
    while current_ebp != 0 && frame_count < 10 {
        let return_addr = unsafe { *((current_ebp + 4) as *const u32) };
        printk!("Frame {}: EBP=0x{:08x}, Return=0x{:08x}\n", 
                frame_count, current_ebp, return_addr);
        
        current_ebp = unsafe { *(current_ebp as *const u32) };
        frame_count += 1;
        
        if current_ebp <= ebp {
            break;
        }
    }
    
    printk!("=== END STACK TRACE ===\n");
}
