use core::arch::asm;

// Stack frame structure for tracing
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct StackFrame {
    pub ebp: u32,
    pub eip: u32,
}

impl StackFrame {
    pub fn new(ebp: u32, eip: u32) -> Self {
        StackFrame { ebp, eip }
    }
}

// Get current stack pointer
pub fn get_stack_pointer() -> u32 {
    let esp: u32;
    unsafe {
        asm!("mov {}, esp", out(reg) esp);
    }
    esp
}

// Get current base pointer
pub fn get_base_pointer() -> u32 {
    let ebp: u32;
    unsafe {
        asm!("mov {}, ebp", out(reg) ebp);
    }
    ebp
}

// Get current instruction pointer (approximately)
pub fn get_instruction_pointer() -> u32 {
    let eip: u32;
    unsafe {
        asm!(
            "call 2f",
            "2: pop {}",
            out(reg) eip
        );
    }
    eip
}

// Walk the stack and collect stack frames
pub fn walk_stack(max_frames: usize) -> Vec<StackFrame> {
    let mut frames = Vec::new();
    let mut current_ebp = get_base_pointer();
    
    for _ in 0..max_frames {
        if current_ebp == 0 || !is_valid_address(current_ebp) {
            break;
        }
        
        unsafe {
            // Read the saved EBP and EIP from the stack frame
            let saved_ebp = *(current_ebp as *const u32);
            let saved_eip = *((current_ebp + 4) as *const u32);
            
            frames.push(StackFrame::new(current_ebp, saved_eip));
            
            // Move to the previous frame
            current_ebp = saved_ebp;
            
            // Sanity check: ensure we're not going backwards in memory
            if saved_ebp != 0 && saved_ebp <= current_ebp {
                break;
            }
        }
    }
    
    frames
}

// Simple Vec implementation for no_std environment
pub struct Vec<T> {
    data: [Option<T>; 16], // Fixed size array for simplicity
    len: usize,
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Vec {
            data: [None, None, None, None, None, None, None, None,
                   None, None, None, None, None, None, None, None],
            len: 0,
        }
    }
    
    pub fn push(&mut self, item: T) {
        if self.len < self.data.len() {
            self.data[self.len] = Some(item);
            self.len += 1;
        }
    }
    
    pub fn len(&self) -> usize {
        self.len
    }
    
    pub fn iter(&self) -> VecIter<'_, T> {
        VecIter {
            vec: self,
            index: 0,
        }
    }
}

pub struct VecIter<'a, T> {
    vec: &'a Vec<T>,
    index: usize,
}

impl<'a, T> Iterator for VecIter<'a, T> {
    type Item = &'a T;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len {
            if let Some(ref item) = self.vec.data[self.index] {
                self.index += 1;
                Some(item)
            } else {
                None
            }
        } else {
            None
        }
    }
}

// Check if an address is valid (basic sanity checks)
fn is_valid_address(addr: u32) -> bool {
    // Basic checks: not null, not too low, reasonable upper bound
    addr > 0x1000 && addr < 0x80000000 && (addr & 0x3) == 0 // Aligned to 4 bytes
}

// Print detailed stack information
pub fn print_stack_info() {
    let esp = get_stack_pointer();
    let ebp = get_base_pointer();
    let eip = get_instruction_pointer();
    
    crate::println!("=== Kernel Stack Information ===");
    crate::println!("Current ESP (Stack Pointer): 0x{:08x}", esp);
    crate::println!("Current EBP (Base Pointer):  0x{:08x}", ebp);
    crate::println!("Current EIP (Instruction):   0x{:08x}", eip);
    crate::println!("Stack Direction: Growing down (ESP decreases)");
    
    if ebp > esp {
        crate::println!("Stack Size (approx):         {} bytes", ebp - esp);
    }
    
    crate::println!("");
}

// Print stack memory dump
pub fn print_stack_dump(words: usize) {
    let esp = get_stack_pointer();
    
    crate::println!("=== Stack Memory Dump ===");
    crate::println!("Dumping {} words from ESP (0x{:08x}):", words, esp);
    crate::println!("");
    
    unsafe {
        for i in 0..words {
            let addr = esp + (i * 4) as u32;
            if is_valid_address(addr) {
                let value = *(addr as *const u32);
                crate::println!("0x{:08x}: 0x{:08x}", addr, value);
            } else {
                crate::println!("0x{:08x}: <invalid>", addr);
                break;
            }
        }
    }
    crate::println!("");
}

// Print full stack trace
pub fn print_stack_trace() {
    crate::println!("=== Kernel Stack Trace ===");
    
    let frames = walk_stack(10);
    
    if frames.len() == 0 {
        crate::println!("No stack frames found or unable to walk stack.");
        return;
    }
    
    crate::println!("Found {} stack frames:", frames.len());
    crate::println!("");
    
    for (i, frame) in frames.iter().enumerate() {
        crate::println!("Frame {}: EBP=0x{:08x}, EIP=0x{:08x}", 
                       i, frame.ebp, frame.eip);
    }
    crate::println!("");
}

// Print stack trace with custom title
pub fn print_stack_trace_with_title(title: &str) {
    crate::println!("=== {} ===", title);
    print_stack_info();
    print_stack_trace();
    print_stack_dump(8);
}

// Get stack usage statistics
pub fn get_stack_stats() -> StackStats {
    let esp = get_stack_pointer();
    let ebp = get_base_pointer();
    
    // Estimate stack usage (this is approximate)
    let estimated_stack_start = 0x100000; // 1MB
    let used_stack = if esp < estimated_stack_start {
        estimated_stack_start - esp
    } else {
        0
    };
    
    StackStats {
        current_esp: esp,
        current_ebp: ebp,
        estimated_used: used_stack,
        frame_count: walk_stack(16).len(),
    }
}

#[derive(Debug)]
pub struct StackStats {
    pub current_esp: u32,
    pub current_ebp: u32,
    pub estimated_used: u32,
    pub frame_count: usize,
}

impl StackStats {
    pub fn print(&self) {
        crate::println!("=== Stack Usage Statistics ===");
        crate::println!("Current ESP: 0x{:08x}", self.current_esp);
        crate::println!("Current EBP: 0x{:08x}", self.current_ebp);
        crate::println!("Estimated Used: {} bytes", self.estimated_used);
        crate::println!("Stack Frames: {}", self.frame_count);
        crate::println!("");
    }
}
