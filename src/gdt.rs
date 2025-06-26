use core::arch::asm;

pub const GDT_BASE: usize = 0x800;
pub const GDT_SIZE: usize = 8; // Number of GDT entries

/// GDT Segment Descriptor flags
#[allow(dead_code)]
mod flags {
    // Access byte flags
    pub const PRESENT: u8 = 1 << 7; // Present bit
    pub const DPL_0: u8 = 0 << 5; // Descriptor Privilege Level 0 (kernel)
    pub const DPL_3: u8 = 3 << 5; // Descriptor Privilege Level 3 (user)
    pub const DESCRIPTOR_TYPE: u8 = 1 << 4; // 1 = code/data segment, 0 = system segment
    pub const EXECUTABLE: u8 = 1 << 3; // Executable bit
    pub const CONFORMING: u8 = 1 << 2; // Conforming bit for code segments
    pub const WRITABLE: u8 = 1 << 1; // Writable bit for data segments
    pub const READABLE: u8 = 1 << 1; // Readable bit for code segments
    pub const ACCESSED: u8 = 1 << 0; // Accessed bit

    // Flags (granularity and size)
    pub const GRANULARITY: u8 = 1 << 3; // 1 = 4KB granularity, 0 = 1B granularity
    pub const SIZE_32: u8 = 1 << 2; // 1 = 32-bit segment, 0 = 16-bit segment
    pub const LONG_MODE: u8 = 1 << 1; // 1 = 64-bit segment (long mode)
    pub const AVAILABLE: u8 = 1 << 0; // Available for system use
}

/// GDT Entry structure
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct GdtEntry {
    limit_low: u16,  // Lower 16 bits of limit
    base_low: u16,   // Lower 16 bits of base
    base_middle: u8, // Middle 8 bits of base
    access: u8,      // Access flags
    granularity: u8, // Granularity and upper 4 bits of limit
    base_high: u8,   // Upper 8 bits of base
}

impl GdtEntry {
    /// Create a new GDT entry
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

    /// Create a null descriptor
    pub fn null() -> Self {
        GdtEntry::new(0, 0, 0, 0)
    }

    /// Create a kernel code segment
    pub fn kernel_code() -> Self {
        GdtEntry::new(
            0,       // Base address
            0xFFFFF, // Limit 4GB
            flags::PRESENT
                | flags::DPL_0
                | flags::DESCRIPTOR_TYPE
                | flags::EXECUTABLE
                | flags::READABLE,
            flags::GRANULARITY | flags::SIZE_32,
        )
    }

    /// Create a kernel data segment
    pub fn kernel_data() -> Self {
        GdtEntry::new(
            0,       // Base address
            0xFFFFF, // Limit 4GB
            flags::PRESENT | flags::DPL_0 | flags::DESCRIPTOR_TYPE | flags::WRITABLE,
            flags::GRANULARITY | flags::SIZE_32,
        )
    }

    /// Create a kernel stack segment
    pub fn kernel_stack() -> Self {
        GdtEntry::new(
            0,       // Base address
            0xFFFFF, // Limit 4GB
            flags::PRESENT | flags::DPL_0 | flags::DESCRIPTOR_TYPE | flags::WRITABLE,
            flags::GRANULARITY | flags::SIZE_32,
        )
    }

    /// Create a user code segment
    pub fn user_code() -> Self {
        GdtEntry::new(
            0,       // Base address
            0xFFFFF, // Limit 4GB
            flags::PRESENT
                | flags::DPL_3
                | flags::DESCRIPTOR_TYPE
                | flags::EXECUTABLE
                | flags::READABLE,
            flags::GRANULARITY | flags::SIZE_32,
        )
    }

    /// Create a user data segment
    pub fn user_data() -> Self {
        GdtEntry::new(
            0,       // Base address
            0xFFFFF, // Limit 4GB
            flags::PRESENT | flags::DPL_3 | flags::DESCRIPTOR_TYPE | flags::WRITABLE,
            flags::GRANULARITY | flags::SIZE_32,
        )
    }

    /// Create a user stack segment
    pub fn user_stack() -> Self {
        GdtEntry::new(
            0,       // Base address
            0xFFFFF, // Limit 4GB
            flags::PRESENT | flags::DPL_3 | flags::DESCRIPTOR_TYPE | flags::WRITABLE,
            flags::GRANULARITY | flags::SIZE_32,
        )
    }

    /// Create a Task State Segment descriptor
    pub fn tss(base: u32, limit: u32) -> Self {
        GdtEntry::new(
            base,
            limit,
            flags::PRESENT | flags::DPL_0 | 0x09, // TSS
            0x00,
        )
    }
}

/// GDT Pointer structure for LGDT instruction
#[repr(C, packed)]
struct GdtPointer {
    limit: u16, // Size of GDT - 1
    base: u32,  // Base address of GDT
}

pub struct Gdt {
    entries: [GdtEntry; GDT_SIZE],
}

impl Gdt {
    pub fn new() -> Self {
        let mut gdt = Gdt {
            entries: [GdtEntry::null(); GDT_SIZE],
        };

        gdt.entries[0] = GdtEntry::null(); // 0x00: Null descriptor
        gdt.entries[1] = GdtEntry::kernel_code(); // 0x08: Kernel code
        gdt.entries[2] = GdtEntry::kernel_data(); // 0x10: Kernel data
        gdt.entries[3] = GdtEntry::kernel_stack(); // 0x18: Kernel stack
        gdt.entries[4] = GdtEntry::user_code(); // 0x20: User code
        gdt.entries[5] = GdtEntry::user_data(); // 0x28: User data
        gdt.entries[6] = GdtEntry::user_stack(); // 0x30: User stack
        gdt.entries[7] = GdtEntry::null(); // 0x38: Reserved for TSS

        gdt
    }

    /// Load the GDT and reload segment registers
    pub fn load(&self) {
        use crate::println;

        // Copy GDT to the required address (0x800)
        let gdt_ptr = GDT_BASE as *mut GdtEntry;

        unsafe {
            // Copy our GDT entries to the fixed address
            for (i, entry) in self.entries.iter().enumerate() {
                core::ptr::write_volatile(gdt_ptr.add(i), *entry);
            }
        }

        // Create GDT pointer
        let gdt_pointer = GdtPointer {
            limit: (GDT_SIZE * core::mem::size_of::<GdtEntry>() - 1) as u16,
            base: GDT_BASE as u32,
        };

        // Load GDT using LGDT instruction
        unsafe {
            asm!(
                "lgdt [{}]",
                in(reg) &gdt_pointer,
                options(readonly, nostack, preserves_flags)
            );
        }

        println!("GDT loaded at 0x{:08X}", GDT_BASE);
    }

    /// Reload segment registers after loading GDT
    #[allow(dead_code)]
    fn reload_segments(&self) {
        unsafe {
            // Reload CS by performing a far jump to kernel code segment (0x08)
            asm!(
                "ljmp $0x08, $1f", // Long jump to reload CS
                "1:",              // Label for jump target
                options(att_syntax)
            );

            // Reload data segment registers
            asm!(
                "movw $0x10, %ax", // Load kernel data segment selector
                "movw %ax, %ds",   // Reload DS
                "movw %ax, %es",   // Reload ES
                "movw %ax, %fs",   // Reload FS
                "movw %ax, %gs",   // Reload GS
                "movw %ax, %ss",   // Reload SS
                options(att_syntax)
            );
        }
    }

    /// Manually reload segment registers (advanced use only)
    pub fn reload_segments_manual(&self) {
        self.reload_segments();
    }

    /// Print GDT information for debugging
    pub fn print_info(&self) {
        use crate::println;

        println!("GDT Information:");
        println!("Base Address: 0x{:08X}", GDT_BASE);
        println!("Size: {} entries", GDT_SIZE);
        println!();

        let descriptors = [
            "Null Descriptor",
            "Kernel Code",
            "Kernel Data",
            "Kernel Stack",
            "User Code",
            "User Data",
            "User Stack",
            "Reserved/TSS",
        ];

        for (i, entry) in self.entries.iter().enumerate() {
            let base = (entry.base_high as u32) << 24
                | (entry.base_middle as u32) << 16
                | (entry.base_low as u32);
            let limit = (entry.granularity as u32 & 0x0F) << 16 | (entry.limit_low as u32);

            println!(
                "Entry {}: {} (Selector: 0x{:02X})",
                i,
                descriptors[i],
                i * 8
            );
            println!("  Base: 0x{:08X}, Limit: 0x{:05X}", base, limit);
            println!(
                "  Access: 0x{:02X}, Granularity: 0x{:02X}",
                entry.access, entry.granularity
            );
        }
    }
}

/// Initialize the Global Descriptor Table
pub fn init() {
    use crate::println;

    println!("Initializing GDT...");

    let gdt = Gdt::new();
    gdt.load();

    println!("GDT loaded successfully at 0x{:08X}", GDT_BASE);
}

/// Get current GDT information for stack tracing and debugging
pub fn get_gdt_info() -> (usize, usize) {
    (GDT_BASE, GDT_SIZE)
}
