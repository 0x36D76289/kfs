use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;
use x86_64::instructions::segmentation::{CS, DS, ES, FS, GS, SS, Segment};
use x86_64::instructions::tables::load_tss;
use lazy_static::lazy_static;
use core::arch::asm;
use core::ptr::addr_of;

pub const STACK_SIZE: usize = 4096 * 4;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

static mut TSS: TaskStateSegment = TaskStateSegment::new();

pub struct Selectors {
    pub kernel_code_selector: SegmentSelector,
    pub kernel_data_selector: SegmentSelector,
    pub user_code_selector: SegmentSelector,
    pub user_data_selector: SegmentSelector,
    pub tss_selector: SegmentSelector,
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        
        // Add kernel code segment (Ring 0)
        let kernel_code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        
        // Add kernel data segment (Ring 0)
        let kernel_data_selector = gdt.add_entry(Descriptor::kernel_data_segment());
        
        // Add user data segment (Ring 3)
        let user_data_selector = gdt.add_entry(Descriptor::user_data_segment());
        
        // Add user code segment (Ring 3)
        let user_code_selector = gdt.add_entry(Descriptor::user_code_segment());
        // Add TSS segment
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(unsafe { &*addr_of!(TSS) }));
        // let tss_selector = gdt.add_entry(Descriptor::tss_segment(unsafe { &TSS }));
        
        (gdt, Selectors {
            kernel_code_selector,
            kernel_data_selector,
            user_code_selector,
            user_data_selector,
            tss_selector,
        })
    };
}

pub fn init_gdt() {
    unsafe {
        static mut DOUBLE_FAULT_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
        
        let stack_start = VirtAddr::from_ptr(addr_of!(DOUBLE_FAULT_STACK).cast::<u8>());
        let stack_end = stack_start + STACK_SIZE;
        TSS.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = stack_end;
    }
    
    GDT.0.load();
    
    unsafe {
        CS::set_reg(GDT.1.kernel_code_selector);
        DS::set_reg(GDT.1.kernel_data_selector);
        ES::set_reg(GDT.1.kernel_data_selector);
        FS::set_reg(GDT.1.kernel_data_selector);
        GS::set_reg(GDT.1.kernel_data_selector);
        SS::set_reg(GDT.1.kernel_data_selector);
        load_tss(GDT.1.tss_selector);
    }
}

pub fn print_gdt_info() {
    crate::println!();
    crate::println!("=== GDT INFORMATION ===");
    crate::println!("Kernel Code Selector: {:?}", GDT.1.kernel_code_selector);
    crate::println!("Kernel Data Selector: {:?}", GDT.1.kernel_data_selector);
    crate::println!("User Code Selector:   {:?}", GDT.1.user_code_selector);
    crate::println!("User Data Selector:   {:?}", GDT.1.user_data_selector);
    crate::println!("TSS Selector:         {:?}", GDT.1.tss_selector);
    crate::println!("Stack Size:           {} bytes", STACK_SIZE);
    crate::println!("=======================");
    crate::println!();
}

pub fn get_selectors() -> &'static Selectors {
    &GDT.1
}

pub fn print_kernel_stack() {
    crate::println!();
    crate::println!("=== KERNEL STACK INFORMATION ===");
    
    let rsp: u64;
    let rbp: u64;
    
    unsafe {
        asm!("mov {}, rsp", out(reg) rsp);
        asm!("mov {}, rbp", out(reg) rbp);
    }
    
    crate::println!("Current RSP (Stack Pointer): 0x{:016X}", rsp);
    crate::println!("Current RBP (Base Pointer):  0x{:016X}", rbp);
    crate::println!("Current CS (Code Segment):   {:?}", CS::get_reg());
    
    crate::println!();
    crate::println!("Stack Content (top 16 entries):");
    crate::println!("Address          | Value");
    crate::println!("-----------------|------------------");
    
    for i in 0..16 {
        let addr = rsp + (i * 8);
        unsafe {
            let value = *(addr as *const u64);
            crate::println!("0x{:016X} | 0x{:016X}", addr, value);
        }
    }
    
    crate::println!("================================");
    crate::println!();
}

pub fn print_call_stack() {
    crate::println!();
    crate::println!("=== CALL STACK TRACE ===");
    
    let mut rbp: u64;
    unsafe {
        asm!("mov {}, rbp", out(reg) rbp);
    }
    
    let mut frame_count = 0;
    
    while rbp != 0 && frame_count < 10 {
        unsafe {
            let return_addr = *((rbp + 8) as *const u64);
            let prev_rbp = *(rbp as *const u64);
            
            crate::println!("Frame {}: RBP=0x{:016X}, Return=0x{:016X}", 
                           frame_count, rbp, return_addr);
            
            rbp = prev_rbp;
            frame_count += 1;
            
            if rbp == prev_rbp || rbp < 0x1000 {
                break;
            }
        }
    }
    
    crate::println!("========================");
    crate::println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test_case]
    fn test_gdt_initialization() {
        let selectors = get_selectors();
        
        assert_ne!(selectors.kernel_code_selector.0, 0);
        assert_ne!(selectors.kernel_data_selector.0, 0);
        assert_ne!(selectors.tss_selector.0, 0);
    }
}
