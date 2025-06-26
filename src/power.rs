use crate::io::{inb, outb, outw};
use core::arch::asm;

/// System power management functionality
pub struct PowerManager;

impl PowerManager {
    /// Reboot the system using the 8042 keyboard controller method
    pub fn reboot() -> ! {
        use crate::println;
        
        println!("Rebooting system...");
        
        // Disable interrupts
        unsafe {
            asm!("cli");
        }
        
        // Method 1: 8042 keyboard controller reset
        Self::keyboard_controller_reset();
        
        // Method 2: If that fails, try triple fault
        Self::triple_fault();
    }
    
    /// Shutdown the system (attempt various methods)
    pub fn shutdown() -> ! {
        use crate::println;
        
        println!("Shutting down system...");
        
        // Disable interrupts
        unsafe {
            asm!("cli");
        }
        
        Self::try_emulator_shutdown();
        
        println!("Shutdown failed - system will halt");
        Self::halt();
    }
    
    /// Halt the system (infinite loop with HLT instruction)
    pub fn halt() -> ! {
        use crate::println;
        
        println!("System halted");
        
        unsafe {
            asm!("cli"); // Disable interrupts
        }
        
        loop {
            unsafe {
                asm!("hlt"); // Halt until interrupt
            }
        }
    }
    
    /// Reset using 8042 keyboard controller
    fn keyboard_controller_reset() {
        // Wait for keyboard controller to be ready
        while (inb(0x64) & 0x02) != 0 {
            // Wait for input buffer to be empty
        }
        
        // Send reset command to keyboard controller
        outb(0x64, 0xFE);
        
        // Give it some time
        for _ in 0..1000000 {
            unsafe {
                asm!("nop");
            }
        }
    }
    
    /// Cause a triple fault to reset the system
    fn triple_fault() -> ! {
        unsafe {
            // Load invalid IDT to cause triple fault
            #[repr(C, packed)]
            struct InvalidIdt {
                limit: u16,
                base: u32,
            }
            
            let invalid_idt = InvalidIdt {
                limit: 0,
                base: 0,
            };
            
            asm!("lidt [{}]", in(reg) &invalid_idt);
            asm!("int $0x03"); // Trigger interrupt with invalid IDT
        }
        
        // Should never reach here
        loop {
            unsafe {
                asm!("hlt");
            }
        }
    }
    
    /// Try emulator-specific shutdown methods
    fn try_emulator_shutdown() {
        // QEMU (newer versions)
        outw(0x604, 0x2000);
        
        // Small delay
        for _ in 0..1000 {
            unsafe { asm!("nop"); }
        }
        
        // Bochs and older QEMU
        outw(0xB004, 0x2000);
        
        // Small delay
        for _ in 0..1000 {
            unsafe { asm!("nop"); }
        }
        
        // VirtualBox
        outw(0x4004, 0x3400);
        
        // Small delay
        for _ in 0..1000 {
            unsafe { asm!("nop"); }
        }
        
        // Cloud Hypervisor
        outb(0x600, 0x34);
    }
}

/// Convenience function for rebooting
pub fn reboot() -> ! {
    PowerManager::reboot()
}

/// Convenience function for shutdown
pub fn shutdown() -> ! {
    PowerManager::shutdown()
}

/// Convenience function for halt
pub fn halt() -> ! {
    PowerManager::halt()
}

/// Emergency halt function (for panic handler)
pub fn emergency_halt() -> ! {
    unsafe {
        asm!("cli"); // Disable interrupts immediately
    }
    
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
