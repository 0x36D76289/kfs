use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use lazy_static::lazy_static;
use crate::println;

const PIC1_CMD: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_CMD: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

const KEYBOARD_INTERRUPT_ID: u8 = 33;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);

        idt[KEYBOARD_INTERRUPT_ID as usize].set_handler_fn(keyboard_interrupt_handler);
        
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.general_protection_fault.set_handler_fn(general_protection_fault_handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
        idt.segment_not_present.set_handler_fn(segment_not_present_handler);
        idt.stack_segment_fault.set_handler_fn(stack_segment_fault_handler);
        
        idt
    };
}

pub fn init_idt() {
    IDT.load();
    init_pics();
}

fn init_pics() {
    use x86_64::instructions::port::Port;
    
    unsafe {
        let mut pic1_cmd: Port<u8> = Port::new(PIC1_CMD);
        let mut pic1_data: Port<u8> = Port::new(PIC1_DATA);
        let mut pic2_cmd: Port<u8> = Port::new(PIC2_CMD);
        let mut pic2_data: Port<u8> = Port::new(PIC2_DATA);
        
        // Start initialization sequence
        pic1_cmd.write(0x11); // ICW1: Initialize + will be sending ICW4
        pic2_cmd.write(0x11);
        
        // ICW2: Set interrupt vector offsets
        pic1_data.write(0x20); // PIC1 offset (32-39)
        pic2_data.write(0x28); // PIC2 offset (40-47)
        
        // ICW3: Configure master/slave relationship
        pic1_data.write(0x04); // Tell PIC1 there's a PIC2 at IRQ2 (binary: 00000100)
        pic2_data.write(0x02); // Tell PIC2 its cascade identity (binary: 00000010)
        
        // ICW4: Set mode
        pic1_data.write(0x01); // 8086/88 mode
        pic2_data.write(0x01);
        
        // Mask all idt initially except keyboard (IRQ1)
        pic1_data.write(0xFD); // 11111101 - enable only IRQ1 (keyboard)
        pic2_data.write(0xFF); // 11111111 - disable all PIC2 idt
    }
}

fn send_eoi(interrupt_id: u8) {
    use x86_64::instructions::port::Port;
    
    unsafe {
        let mut pic1_cmd: Port<u8> = Port::new(PIC1_CMD);
        let mut pic2_cmd: Port<u8> = Port::new(PIC2_CMD);
        
        if interrupt_id >= 0x28 {
            pic2_cmd.write(0x20u8);
        }
        pic1_cmd.write(0x20u8);
    }
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

// Double fault handler - use proper diverging signature
#[allow(dead_code)]
extern "x86-interrupt" fn double_fault_handler_wrapper(
    stack_frame: InterruptStackFrame, 
    _error_code: u64
) {
    println!("EXCEPTION: DOUBLE FAULT");
    println!("{:#?}", stack_frame);
    
    // Print stack information for debugging
    crate::arch::x86_64::gdt::print_kernel_stack();
    crate::arch::x86_64::gdt::print_call_stack();
    
    loop {
        x86_64::instructions::hlt();
    }
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;
    
    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    
    // Print stack information for debugging
    crate::arch::x86_64::gdt::print_kernel_stack();
    crate::arch::x86_64::gdt::print_call_stack();
    
    panic!("Page fault");
}

extern "x86-interrupt" fn general_protection_fault_handler(
    stack_frame: InterruptStackFrame, error_code: u64) {
    println!("EXCEPTION: GENERAL PROTECTION FAULT ({})", error_code);
    println!("{:#?}", stack_frame);
    
    // Print stack information for debugging
    crate::arch::x86_64::gdt::print_kernel_stack();
    crate::arch::x86_64::gdt::print_call_stack();
    
    panic!("General protection fault");
}

extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: INVALID OPCODE");
    println!("{:#?}", stack_frame);
    
    // Print stack information for debugging
    crate::arch::x86_64::gdt::print_kernel_stack();
    crate::arch::x86_64::gdt::print_call_stack();
    
    panic!("Invalid opcode");
}

extern "x86-interrupt" fn segment_not_present_handler(
    stack_frame: InterruptStackFrame, error_code: u64) {
    println!("EXCEPTION: SEGMENT NOT PRESENT ({})", error_code);
    println!("{:#?}", stack_frame);
    
    // Print stack information for debugging
    crate::arch::x86_64::gdt::print_kernel_stack();
    crate::arch::x86_64::gdt::print_call_stack();
    
    panic!("Segment not present");
}

extern "x86-interrupt" fn stack_segment_fault_handler(
    stack_frame: InterruptStackFrame, error_code: u64) {
    println!("EXCEPTION: STACK SEGMENT FAULT ({})", error_code);
    println!("{:#?}", stack_frame);
    
    // Print stack information for debugging
    crate::arch::x86_64::gdt::print_kernel_stack();
    crate::arch::x86_64::gdt::print_call_stack();
    
    panic!("Stack segment fault");
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // Handle keyboard interrupt
    crate::drivers::keyboard::handle_keyboard_interrupt();
    
    // Send End of Interrupt signal
    send_eoi(KEYBOARD_INTERRUPT_ID);
}

#[cfg(test)]
mod tests {
    #[test_case]
    fn test_breakpoint_exception() {
        // Déclenche une exception breakpoint
        x86_64::instructions::interrupts::int3();
        // Si on arrive ici, l'exception a été gérée correctement
    }
}
