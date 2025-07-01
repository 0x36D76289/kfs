use core::arch::asm;

// GDT entry structure
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

impl GdtEntry {
    pub fn new(base: u32, limit: u32, access: u8, granularity: u8) -> Self {
        GdtEntry {
            limit_low: (limit & 0xFFFF) as u16,
            base_low: (base & 0xFFFF) as u16,
            base_middle: ((base >> 16) & 0xFF) as u8,
            access,
            granularity: (granularity & 0xF0) | (((limit >> 16) & 0x0F) as u8),
            base_high: ((base >> 24) & 0xFF) as u8,
        }
    }

    pub fn null() -> Self {
        GdtEntry {
            limit_low: 0,
            base_low: 0,
            base_middle: 0,
            access: 0,
            granularity: 0,
            base_high: 0,
        }
    }
}

// GDT pointer structure
#[repr(C, packed)]
#[derive(Debug)]
pub struct GdtPtr {
    limit: u16,
    base: u32,
}

// Global Descriptor Table
#[repr(C, packed)]
pub struct Gdt {
    entries: [GdtEntry; 6], // null, kernel code, kernel data, kernel stack, user code, user data, user stack
}

impl Gdt {
    pub fn new() -> Self {
        let mut gdt = Gdt {
            entries: [GdtEntry::null(); 6],
        };

        // Null descriptor (required)
        gdt.entries[0] = GdtEntry::null();

        // Kernel code segment (0x08)
        // Base: 0x00000000, Limit: 0xFFFFF, Access: 0x9A (Present, Ring 0, Code, Readable)
        // Granularity: 0xCF (4KB granularity, 32-bit)
        gdt.entries[1] = GdtEntry::new(0x00000000, 0xFFFFF, 0x9A, 0xCF);

        // Kernel data segment (0x10)
        // Base: 0x00000000, Limit: 0xFFFFF, Access: 0x92 (Present, Ring 0, Data, Writable)
        // Granularity: 0xCF (4KB granularity, 32-bit)
        gdt.entries[2] = GdtEntry::new(0x00000000, 0xFFFFF, 0x92, 0xCF);

        // Kernel stack segment (0x18)
        // Base: 0x00000000, Limit: 0xFFFFF, Access: 0x92 (Present, Ring 0, Data, Writable)
        // Granularity: 0xCF (4KB granularity, 32-bit)
        gdt.entries[3] = GdtEntry::new(0x00000000, 0xFFFFF, 0x92, 0xCF);

        // User code segment (0x20)
        // Base: 0x00000000, Limit: 0xFFFFF, Access: 0xFA (Present, Ring 3, Code, Readable)
        // Granularity: 0xCF (4KB granularity, 32-bit)
        gdt.entries[4] = GdtEntry::new(0x00000000, 0xFFFFF, 0xFA, 0xCF);

        // User data/stack segment (0x28)
        // Base: 0x00000000, Limit: 0xFFFFF, Access: 0xF2 (Present, Ring 3, Data, Writable)
        // Granularity: 0xCF (4KB granularity, 32-bit)
        gdt.entries[5] = GdtEntry::new(0x00000000, 0xFFFFF, 0xF2, 0xCF);

        gdt
    }

    pub fn load(&self) {
        let gdt_ptr = GdtPtr {
            limit: (core::mem::size_of::<Gdt>() - 1) as u16,
            base: self as *const _ as u32,
        };

        unsafe {
            // Load GDT
            asm!(
                "lgdt [{}]",
                in(reg) &gdt_ptr as *const _ as u32
            );

            // Reload segment registers
            asm!(
                "mov ax, 0x10",      // Kernel data segment
                "mov ds, ax",
                "mov es, ax",
                "mov fs, ax",
                "mov gs, ax",
                "mov ss, ax",
                out("ax") _
            );

            // Far jump to reload CS
            asm!(
                "push 0x08",         // Kernel code segment
                "lea eax, [2f]",
                "push eax",
                "retf",
                "2:",
                out("eax") _
            );
        }
    }

    pub fn get_entry(&self, index: usize) -> Option<&GdtEntry> {
        if index < self.entries.len() {
            Some(&self.entries[index])
        } else {
            None
        }
    }

    pub fn print_gdt_info(&self) {
        crate::println!("=== Global Descriptor Table ===");
        crate::println!("GDT Base Address: 0x{:08x}", self as *const _ as u32);
        crate::println!("GDT Size: {} bytes", core::mem::size_of::<Gdt>());
        crate::println!("");

        // Segment names for better readability
        let segment_names = [
            "Null Descriptor",
            "Kernel Code",
            "Kernel Data", 
            "Kernel Stack",
            "User Code",
            "User Data/Stack",
        ];

        // Print each GDT entry
        for (i, entry) in self.entries.iter().enumerate() {
            let base = (entry.base_high as u32) << 24 
                     | (entry.base_middle as u32) << 16 
                     | entry.base_low as u32;
            
            let limit = ((entry.granularity as u32 & 0x0F) << 16) | entry.limit_low as u32;
            
            crate::println!("Entry {}: {} (Selector 0x{:02x})", i, segment_names[i], i * 8);
            crate::println!("  Base: 0x{:08x}", base);
            crate::println!("  Limit: 0x{:05x}", limit);
            crate::println!("  Access: 0x{:02x}", entry.access);
            crate::println!("  Granularity: 0x{:02x}", entry.granularity);
            crate::println!("");
        }
    }
}

// Global state to track GDT initialization
static mut GDT_INITIALIZED: bool = false;

pub fn init() {
    unsafe {
        // Place GDT at address 0x00000800 as required
        let gdt_ptr = 0x800 as *mut Gdt;
        let gdt = &mut *(gdt_ptr);
        *gdt = Gdt::new();
        
        crate::println!("Initializing GDT at address 0x{:08x}", gdt_ptr as u32);
        gdt.load();
        crate::println!("GDT loaded successfully!");
        
        // Print GDT information directly
        gdt.print_gdt_info();
        
        GDT_INITIALIZED = true;
    }
}

pub fn get_gdt() -> Option<&'static Gdt> {
    unsafe {
        if GDT_INITIALIZED {
            Some(&*(0x800 as *const Gdt))
        } else {
            None
        }
    }
}

// Access bit flags
pub const PRESENT: u8 = 1 << 7;
pub const PRIVILEGE_RING0: u8 = 0 << 5;
pub const PRIVILEGE_RING3: u8 = 3 << 5;
pub const DESCRIPTOR_TYPE: u8 = 1 << 4;
pub const EXECUTABLE: u8 = 1 << 3;
pub const DIRECTION_CONFORMING: u8 = 1 << 2;
pub const READABLE_WRITABLE: u8 = 1 << 1;

// Granularity bit flags
pub const GRANULARITY_4K: u8 = 1 << 7;
pub const SIZE_32: u8 = 1 << 6;

// Test function to verify GDT is working correctly
pub fn test_gdt_functionality() {
    crate::println!("=== GDT Functionality Test ===");
    
    // Test segment register values
    let cs: u16;
    let ds: u16;
    let es: u16;
    let fs: u16;
    let gs: u16;
    let ss: u16;
    
    unsafe {
        asm!("mov {:x}, cs", out(reg) cs);
        asm!("mov {:x}, ds", out(reg) ds);
        asm!("mov {:x}, es", out(reg) es);
        asm!("mov {:x}, fs", out(reg) fs);
        asm!("mov {:x}, gs", out(reg) gs);
        asm!("mov {:x}, ss", out(reg) ss);
    }
    
    crate::println!("Current Segment Registers:");
    crate::println!("  CS (Code Segment):    0x{:04x}", cs);
    crate::println!("  DS (Data Segment):    0x{:04x}", ds);
    crate::println!("  ES (Extra Segment):   0x{:04x}", es);
    crate::println!("  FS (F Segment):       0x{:04x}", fs);
    crate::println!("  GS (G Segment):       0x{:04x}", gs);
    crate::println!("  SS (Stack Segment):   0x{:04x}", ss);
    
    // Verify expected values
    if cs == 0x08 && ds == 0x10 && ss == 0x10 {
        crate::println!("✓ GDT segments loaded correctly!");
    } else {
        crate::println!("✗ GDT segments may not be loaded correctly!");
    }
    
    crate::println!("");
}
