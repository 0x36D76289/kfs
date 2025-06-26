use core::arch::asm;

/// Maximum number of stack frames to trace
const MAX_STACK_FRAMES: usize = 32;

/// Stack frame structure for x86 CDECL calling convention
#[repr(C)]
struct StackFrame {
    ebp: *const StackFrame,  // Previous frame pointer
    eip: u32,                // Return address
}

/// Stack trace information
#[derive(Debug)]
pub struct StackTrace {
    frames: [u32; MAX_STACK_FRAMES],
    count: usize,
}

impl StackTrace {
    /// Create a new empty stack trace
    pub fn new() -> Self {
        StackTrace {
            frames: [0; MAX_STACK_FRAMES],
            count: 0,
        }
    }

    /// Capture the current stack trace
    pub fn capture() -> Self {
        let mut trace = StackTrace::new();
        trace.walk_stack();
        trace
    }

    /// Walk the stack and collect return addresses
    fn walk_stack(&mut self) {
        let mut current_frame: *const StackFrame;
        
        // Get current EBP
        unsafe {
            asm!("mov {}, ebp", out(reg) current_frame);
        }

        // Walk the stack frames
        self.count = 0;
        while !current_frame.is_null() && self.count < MAX_STACK_FRAMES {
            unsafe {
                // Check if the frame pointer is valid
                if !self.is_valid_address(current_frame as usize) {
                    break;
                }

                let frame = &*current_frame;
                
                // Store the return address
                self.frames[self.count] = frame.eip;
                self.count += 1;

                // Move to the previous frame
                current_frame = frame.ebp;

                // Break if we hit a null frame pointer (end of stack)
                if current_frame.is_null() {
                    break;
                }
            }
        }
    }

    /// Check if an address is valid for stack walking
    fn is_valid_address(&self, addr: usize) -> bool {
        // Basic validity checks:
        // - Not null
        // - In reasonable kernel memory range
        // - Aligned to 4-byte boundary
        addr != 0 && 
        addr >= 0x100000 &&    // Above 1MB
        addr < 0x40000000 &&   // Below 1GB (reasonable for kernel)
        addr % 4 == 0          // 4-byte aligned
    }

    /// Print the stack trace in a human-friendly format
    pub fn print(&self) {
        self.print_with_title("Stack Trace");
    }

    /// Print the stack trace with a custom title
    pub fn print_with_title(&self, title: &str) {
        use crate::println;

        println!("=== {} ===", title);
        
        if self.count == 0 {
            println!("No stack frames captured");
            return;
        }

        println!("Call stack ({} frames):", self.count);
        
        for (i, &address) in self.frames[..self.count].iter().enumerate() {
            // Try to resolve function name (basic implementation)
            if let Some(name) = self.resolve_symbol(address) {
                println!("  #{}: 0x{:08X} - {}", i, address, name);
            } else {
                println!("  #{}: 0x{:08X} - <unknown>", i, address);
            }
        }
        println!("=== End {} ===", title);
    }

    /// Basic symbol resolution (can be enhanced with symbol table but its not for mon bro samsam...)
    fn resolve_symbol(&self, address: u32) -> Option<&'static str> {
        match address {
            0x100000..0x102000 => Some("kmain area"),
            0x102000..0x104000 => Some("screen module"),
            0x104000..0x106000 => Some("keyboard module"),
            0x106000..0x108000 => Some("shell module"),
            0x108000..0x10A000 => Some("gdt module"),
            _ => None,
        }
    }

    /// Get the number of frames in this trace
    pub fn frame_count(&self) -> usize {
        self.count
    }

    /// Get a specific frame address
    pub fn get_frame(&self, index: usize) -> Option<u32> {
        if index < self.count {
            Some(self.frames[index])
        } else {
            None
        }
    }
}

/// Print the current stack trace (convenience function)
pub fn print_stack_trace() {
    let trace = StackTrace::capture();
    trace.print();
}

/// Print the current stack trace with a custom title
pub fn print_stack_trace_with_title(title: &str) {
    let trace = StackTrace::capture();
    trace.print_with_title(title);
}

/// Get current stack pointer value
pub fn get_stack_pointer() -> u32 {
    let esp: u32;
    unsafe {
        asm!("mov {}, esp", out(reg) esp);
    }
    esp
}

/// Get current base pointer value
pub fn get_base_pointer() -> u32 {
    let ebp: u32;
    unsafe {
        asm!("mov {}, ebp", out(reg) ebp);
    }
    ebp
}

/// Print detailed stack information
pub fn print_stack_info() {
    use crate::println;
    
    let esp = get_stack_pointer();
    let ebp = get_base_pointer();
    
    println!("=== Stack Information ===");
    println!("Stack Pointer (ESP): 0x{:08X}", esp);
    println!("Base Pointer (EBP):  0x{:08X}", ebp);
    println!("Stack Size (approx): {} bytes", ebp.wrapping_sub(esp));
    
    // Print stack trace
    println!();
    print_stack_trace_with_title("Current Call Stack");
    
    // Print some stack contents
    println!();
    println!("Stack Contents (near ESP):");
    unsafe {
        let stack_ptr = esp as *const u32;
        for i in 0..8 {
            let addr = esp + (i * 4);
            let value = core::ptr::read_volatile(stack_ptr.offset(i as isize));
            println!("  0x{:08X}: 0x{:08X}", addr, value);
        }
    }
}

/// Advanced stack analysis function
pub fn analyze_stack() {
    use crate::println;
    
    println!("=== Advanced Stack Analysis ===");
    
    let trace = StackTrace::capture();
    
    println!("Call depth: {} levels", trace.frame_count());
    
    if trace.frame_count() > 0 {
        println!("Top of stack: 0x{:08X}", trace.get_frame(0).unwrap_or(0));
        
        if trace.frame_count() > 1 {
            println!("Called from: 0x{:08X}", trace.get_frame(1).unwrap_or(0));
        }
    }
    
    // Analyze stack usage
    let esp = get_stack_pointer();
    let _ebp = get_base_pointer();
    
    // Estimate stack usage based on ESP
    let estimated_stack_base = 0x200000u32; // Estimated stack base
    let stack_used = estimated_stack_base.saturating_sub(esp);
    
    println!("Estimated stack usage: {} bytes", stack_used);
    
    // Check for high stack usage
    if stack_used > 4096 {
        println!("WARNING: High stack usage detected!");
    }
    
    trace.print();
}
