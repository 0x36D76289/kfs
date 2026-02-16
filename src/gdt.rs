use core::arch::asm;

pub const GDT_ADDRESS: usize = 0x00000800;
pub const GDT_ENTRIES: usize = 7;

pub mod selectors {
    pub const NULL: u16 = 0x00;
    pub const KERNEL_CODE: u16 = 0x08;
    pub const KERNEL_DATA: u16 = 0x10;
    pub const KERNEL_STACK: u16 = 0x18;
    pub const USER_CODE: u16 = 0x20;
    pub const USER_DATA: u16 = 0x28;
    pub const USER_STACK: u16 = 0x30;
}

mod access {
    pub const PRESENT: u8 = 1 << 7;
    pub const RING_0: u8 = 0 << 5;
    pub const RING_3: u8 = 3 << 5;
    pub const DESCRIPTOR: u8 = 1 << 4;
    pub const EXECUTABLE: u8 = 1 << 3;
    pub const DIRECTION: u8 = 0 << 2;
    pub const READWRITE: u8 = 1 << 1;
    pub const ACCESSED: u8 = 0 << 0;
}

mod granularity {
    pub const PAGE_GRAN: u8 = 1 << 7;
    pub const SIZE_32: u8 = 1 << 6;
    pub const LONG_MODE: u8 = 0 << 5;
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

impl GdtEntry {
    pub const fn null() -> GdtEntry {
        GdtEntry {
            limit_low: 0,
            base_low: 0,
            base_middle: 0,
            access: 0,
            granularity: 0,
            base_high: 0,
        }
    }

    pub const fn new(base: u32, limit: u32, access: u8, flags: u8) -> GdtEntry {
        GdtEntry {
            limit_low: (limit & 0xFFFF) as u16,
            base_low: (base & 0xFFFF) as u16,
            base_middle: ((base >> 16) & 0xFF) as u8,
            access,
            granularity: ((limit >> 16) & 0x0F) as u8 | (flags & 0xF0),
            base_high: ((base >> 24) & 0xFF) as u8,
        }
    }
}

#[repr(C, packed)]
pub struct GdtPointer {
    pub limit: u16,
    pub base: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Gdt {
    pub entries: [GdtEntry; GDT_ENTRIES],
}

impl Gdt {
    pub const fn new() -> Gdt {
        let kernel_code_access = access::PRESENT
            | access::RING_0
            | access::DESCRIPTOR
            | access::EXECUTABLE
            | access::READWRITE;
        let kernel_data_access =
            access::PRESENT | access::RING_0 | access::DESCRIPTOR | access::READWRITE;
        let kernel_stack_access =
            access::PRESENT | access::RING_0 | access::DESCRIPTOR | access::READWRITE;

        let user_code_access = access::PRESENT
            | access::RING_3
            | access::DESCRIPTOR
            | access::EXECUTABLE
            | access::READWRITE;
        let user_data_access =
            access::PRESENT | access::RING_3 | access::DESCRIPTOR | access::READWRITE;
        let user_stack_access =
            access::PRESENT | access::RING_3 | access::DESCRIPTOR | access::READWRITE;

        let flags = granularity::PAGE_GRAN | granularity::SIZE_32;

        Gdt {
            entries: [
                GdtEntry::null(),
                GdtEntry::new(0, 0xFFFFF, kernel_code_access, flags),
                GdtEntry::new(0, 0xFFFFF, kernel_data_access, flags),
                GdtEntry::new(0, 0xFFFFF, kernel_stack_access, flags),
                GdtEntry::new(0, 0xFFFFF, user_code_access, flags),
                GdtEntry::new(0, 0xFFFFF, user_data_access, flags),
                GdtEntry::new(0, 0xFFFFF, user_stack_access, flags),
            ],
        }
    }
}

static GDT: Gdt = Gdt::new();

static mut GDT_PTR: GdtPointer = GdtPointer {
    limit: (core::mem::size_of::<Gdt>() - 1) as u16,
    base: GDT_ADDRESS as u32,
};

pub fn init() {
    unsafe {
        let gdt_dest = GDT_ADDRESS as *mut Gdt;
        core::ptr::write_volatile(gdt_dest, GDT);

        GDT_PTR.base = GDT_ADDRESS as u32;
        GDT_PTR.limit = (core::mem::size_of::<Gdt>() - 1) as u16;

        load_gdt(&GDT_PTR);
        reload_segments();
    }
}

unsafe fn load_gdt(gdt_ptr: &GdtPointer) {
    asm!(
        "lgdt [{}]",
        in(reg) gdt_ptr,
        options(nostack, preserves_flags)
    );
}

unsafe fn reload_segments() {
    asm!(
        "push {kernel_code}",
        "lea eax, [2f]",
        "push eax",
        "retf",
        "2:",
        "mov ax, {kernel_data}",
        "mov ds, ax",
        "mov es, ax",
        "mov fs, ax",
        "mov gs, ax",
        "mov ax, {kernel_stack}",
        "mov ss, ax",
        kernel_code = const selectors::KERNEL_CODE,
        kernel_data = const selectors::KERNEL_DATA,
        kernel_stack = const selectors::KERNEL_STACK,
        out("eax") _,
        options(preserves_flags)
    );
}

pub fn get_gdt() -> &'static Gdt {
    unsafe { &*(GDT_ADDRESS as *const Gdt) }
}

pub fn get_gdt_info() -> (u32, u16) {
    unsafe { (GDT_PTR.base, GDT_PTR.limit) }
}

pub fn describe_entry(index: usize) -> (&'static str, u8, u8) {
    let names = [
        "Null",
        "Kernel Code",
        "Kernel Data",
        "Kernel Stack",
        "User Code",
        "User Data",
        "User Stack",
    ];

    let gdt = get_gdt();
    if index < GDT_ENTRIES {
        (
            names[index],
            gdt.entries[index].access,
            gdt.entries[index].granularity,
        )
    } else {
        ("Invalid", 0, 0)
    }
}
