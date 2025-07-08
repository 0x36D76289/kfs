//! x86_64 architecture support
//! 
//! This module contains x86_64 specific implementations including:
//! - GDT (Global Descriptor Table)
//! - IDT (Interrupt Descriptor Table) 
//! - Interrupt handling
//! - Low-level assembly boot code

pub mod gdt;
pub mod idt;

// Re-export commonly used items
pub use gdt::*;
pub use idt::*;

/// Initialize all x86_64 specific components
pub fn init() {
    gdt::init_gdt();
    idt::init_idt();
}
