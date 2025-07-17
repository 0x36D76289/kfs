// https://wiki.osdev.org/Global_Descriptor_Table
use core::mem::size_of;

const GDT_BASE_ADDRESS: u32 = 0x00000800;
const GDT_ENTRIES: usize = 7;

#[repr(C, packed)]
struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

#[repr(C, packed)]
struct GdtPtr {
    limit: u16,
    base: u32,
}

impl GdtEntry {
    fn new(base: u32, limit: u32, access: u8, granularity: u8) -> Self {
        Self {
            limit_low: (limit & 0xFFFF) as u16,
            base_low: (base & 0xFFFF) as u16,
            base_middle: ((base >> 16) & 0xFF) as u8,
            access,
            granularity: (granularity & 0xF0) | (((limit >> 16) & 0x0F) as u8),
            base_high: ((base >> 24) & 0xFF) as u8,
        }
    }
}

// GDT segment selectors
#[allow(dead_code)]
pub const KERNEL_CODE_SELECTOR: u16 = 0x08;
#[allow(dead_code)]
pub const KERNEL_DATA_SELECTOR: u16 = 0x10;
#[allow(dead_code)]
pub const KERNEL_STACK_SELECTOR: u16 = 0x18;
#[allow(dead_code)]
pub const USER_CODE_SELECTOR: u16 = 0x20;
#[allow(dead_code)]
pub const USER_DATA_SELECTOR: u16 = 0x28;
#[allow(dead_code)]
pub const USER_STACK_SELECTOR: u16 = 0x30;

pub fn init() {
    let gdt = unsafe { &mut *(GDT_BASE_ADDRESS as *mut [GdtEntry; GDT_ENTRIES]) };
    
    // Null descriptor
    gdt[0] = GdtEntry::new(0, 0, 0, 0);
    
    // Kernel code segment (0x08)
    gdt[1] = GdtEntry::new(0, 0xFFFFF, 0x9A, 0xCF);
    
    // Kernel data segment (0x10)
    gdt[2] = GdtEntry::new(0, 0xFFFFF, 0x92, 0xCF);
    
    // Kernel stack segment (0x18)
    gdt[3] = GdtEntry::new(0, 0xFFFFF, 0x96, 0xCF);
    
    // User code segment (0x20)
    gdt[4] = GdtEntry::new(0, 0xFFFFF, 0xFA, 0xCF);
    
    // User data segment (0x28)
    gdt[5] = GdtEntry::new(0, 0xFFFFF, 0xF2, 0xCF);
    
    // User stack segment (0x30)
    gdt[6] = GdtEntry::new(0, 0xFFFFF, 0xF6, 0xCF);
    
    let gdt_ptr = GdtPtr {
        limit: (GDT_ENTRIES * size_of::<GdtEntry>() - 1) as u16,
        base: GDT_BASE_ADDRESS,
    };
    
    unsafe {
        load_gdt(&gdt_ptr);
    }
}

extern "C" {
    fn load_gdt(gdt_ptr: *const GdtPtr);
}
