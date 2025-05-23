use crate::println;
use core::arch::asm;

const GDT_ADDRESS: u32 = 0x00000800;

#[repr(C, packed)]
#[derive(Copy, Clone)]
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

// GDT access flags
const PRESENT: u8 = 1 << 7;
const DPL_0: u8 = 0 << 5; // Ring 0 (kernel)
const DPL_3: u8 = 3 << 5; // Ring 3 (user)
const DESCRIPTOR_TYPE: u8 = 1 << 4;
const EXECUTABLE: u8 = 1 << 3;
const READABLE_WRITABLE: u8 = 1 << 1;

// GDT granularity flags
const GRANULARITY_4K: u8 = 1 << 7;
const SIZE_32: u8 = 1 << 6;

impl GdtEntry {
    const fn new(base: u32, limit: u32, access: u8, granularity: u8) -> GdtEntry {
        GdtEntry {
            limit_low: (limit & 0xFFFF) as u16,
            base_low: (base & 0xFFFF) as u16,
            base_middle: ((base >> 16) & 0xFF) as u8,
            access,
            granularity: (granularity & 0xF0) | ((limit >> 16) & 0x0F) as u8,
            base_high: ((base >> 24) & 0xFF) as u8,
        }
    }

    const fn null() -> GdtEntry {
        GdtEntry::new(0, 0, 0, 0)
    }
}

static mut GDT: [GdtEntry; 7] = [
    // Null descriptor
    GdtEntry::null(),
    // Kernel code segment (0x08)
    GdtEntry::new(
        0,
        0xFFFFF,
        PRESENT | DPL_0 | DESCRIPTOR_TYPE | EXECUTABLE | READABLE_WRITABLE,
        GRANULARITY_4K | SIZE_32,
    ),
    // Kernel data segment (0x10)
    GdtEntry::new(
        0,
        0xFFFFF,
        PRESENT | DPL_0 | DESCRIPTOR_TYPE | READABLE_WRITABLE,
        GRANULARITY_4K | SIZE_32,
    ),
    // Kernel stack segment (0x18)
    GdtEntry::new(
        0,
        0xFFFFF,
        PRESENT | DPL_0 | DESCRIPTOR_TYPE | READABLE_WRITABLE,
        GRANULARITY_4K | SIZE_32,
    ),
    // User code segment (0x20)
    GdtEntry::new(
        0,
        0xFFFFF,
        PRESENT | DPL_3 | DESCRIPTOR_TYPE | EXECUTABLE | READABLE_WRITABLE,
        GRANULARITY_4K | SIZE_32,
    ),
    // User data segment (0x28)
    GdtEntry::new(
        0,
        0xFFFFF,
        PRESENT | DPL_3 | DESCRIPTOR_TYPE | READABLE_WRITABLE,
        GRANULARITY_4K | SIZE_32,
    ),
    // User stack segment (0x30)
    GdtEntry::new(
        0,
        0xFFFFF,
        PRESENT | DPL_3 | DESCRIPTOR_TYPE | READABLE_WRITABLE,
        GRANULARITY_4K | SIZE_32,
    ),
];

pub fn init_gdt() {
    // Copy GDT to the specified address
    unsafe {
        let gdt_ptr = GDT_ADDRESS as *mut [GdtEntry; 7];
        *gdt_ptr = GDT;
    }

    let gdt_ptr = GdtPtr {
        limit: (core::mem::size_of::<[GdtEntry; 7]>() - 1) as u16,
        base: GDT_ADDRESS,
    };

    unsafe {
        asm!(
            "lgdt [{}]",
            in(reg) &gdt_ptr,
            options(readonly, nostack, preserves_flags)
        );

        asm!(
            "mov ax, 0x10",
            "mov ds, ax",
            "mov es, ax",
            "mov fs, ax",
            "mov gs, ax",
            "mov ss, ax",
            options(nostack, preserves_flags)
        );

        asm!(
            "push 0x08",
            "lea eax, [2f]",
            "push eax",
            "retf",
            "2:",
            options(nostack, preserves_flags)
        );
    }

    println!("GDT initialized at address 0x{:08X}", GDT_ADDRESS);
}

pub fn print_kernel_stack() {
    let mut ebp: u32;
    let mut esp: u32;

    unsafe {
        asm!("mov {}, ebp", out(reg) ebp);
        asm!("mov {}, esp", out(reg) esp);
    }

    println!("=== Kernel Stack Information ===");
    println!("ESP (Stack Pointer): 0x{:08X}", esp);
    println!("EBP (Base Pointer):  0x{:08X}", ebp);
    println!("Stack Frame Chain:");

    let mut frame_ptr = ebp;
    let mut frame_count = 0;

    while frame_ptr != 0 && frame_count < 10 {
        unsafe {
            let next_frame = *(frame_ptr as *const u32);
            let return_addr = *((frame_ptr + 4) as *const u32);

            println!(
                "  Frame {}: EBP=0x{:08X}, Return=0x{:08X}",
                frame_count, frame_ptr, return_addr
            );

            if next_frame <= frame_ptr || next_frame == 0 {
                break;
            }

            frame_ptr = next_frame;
            frame_count += 1;
        }
    }

    println!("Stack Contents (from ESP):");
    for i in 0..8 {
        unsafe {
            let addr = esp + (i * 4);
            let value = *(addr as *const u32);
            println!("  0x{:08X}: 0x{:08X}", addr, value);
        }
    }
}
