use super::{pmm, PAGE_SIZE};
use core::arch::asm;

pub const ENTRIES_PER_TABLE: usize = 1024;

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum PageFlags {
    Present = 1 << 0,
    Writable = 1 << 1,
    User = 1 << 2,
    WriteThrough = 1 << 3,
    CacheDisable = 1 << 4,
    Accessed = 1 << 5,
    Dirty = 1 << 6,
    PageSize = 1 << 7,
    Global = 1 << 8,
}

impl PageFlags {
    pub fn bits(self) -> u32 {
        self as u32
    }
}

pub fn flags(flags: &[PageFlags]) -> u32 {
    flags.iter().fold(0u32, |acc, f| acc | f.bits())
}

pub const KERNEL_PAGE_FLAGS: u32 = 0b11; // Present | Writable
pub const USER_PAGE_FLAGS: u32 = 0b111; // Present | Writable | User

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct PageDirectoryEntry(u32);

impl PageDirectoryEntry {
    pub const fn empty() -> Self {
        PageDirectoryEntry(0)
    }

    pub fn new(table_addr: u32, flags: u32) -> Self {
        PageDirectoryEntry((table_addr & 0xFFFFF000) | (flags & 0xFFF))
    }

    pub fn is_present(&self) -> bool {
        self.0 & PageFlags::Present.bits() != 0
    }

    pub fn is_writable(&self) -> bool {
        self.0 & PageFlags::Writable.bits() != 0
    }

    pub fn is_user(&self) -> bool {
        self.0 & PageFlags::User.bits() != 0
    }

    pub fn table_addr(&self) -> u32 {
        self.0 & 0xFFFFF000
    }

    pub fn flags(&self) -> u32 {
        self.0 & 0xFFF
    }

    pub fn set(&mut self, table_addr: u32, flags: u32) {
        self.0 = (table_addr & 0xFFFFF000) | (flags & 0xFFF);
    }

    pub fn clear(&mut self) {
        self.0 = 0;
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct PageTableEntry(u32);

impl PageTableEntry {
    pub const fn empty() -> Self {
        PageTableEntry(0)
    }

    pub fn new(frame_addr: u32, flags: u32) -> Self {
        PageTableEntry((frame_addr & 0xFFFFF000) | (flags & 0xFFF))
    }

    pub fn is_present(&self) -> bool {
        self.0 & PageFlags::Present.bits() != 0
    }

    pub fn is_writable(&self) -> bool {
        self.0 & PageFlags::Writable.bits() != 0
    }

    pub fn is_user(&self) -> bool {
        self.0 & PageFlags::User.bits() != 0
    }

    pub fn frame_addr(&self) -> u32 {
        self.0 & 0xFFFFF000
    }

    pub fn flags(&self) -> u32 {
        self.0 & 0xFFF
    }

    pub fn set(&mut self, frame_addr: u32, flags: u32) {
        self.0 = (frame_addr & 0xFFFFF000) | (flags & 0xFFF);
    }

    pub fn clear(&mut self) {
        self.0 = 0;
    }
}

#[repr(C, align(4096))]
pub struct PageTable {
    pub entries: [PageTableEntry; ENTRIES_PER_TABLE],
}

impl PageTable {
    pub const fn new() -> Self {
        PageTable {
            entries: [PageTableEntry::empty(); ENTRIES_PER_TABLE],
        }
    }
}

#[repr(C, align(4096))]
pub struct PageDirectory {
    pub entries: [PageDirectoryEntry; ENTRIES_PER_TABLE],
}

impl PageDirectory {
    pub const fn new() -> Self {
        PageDirectory {
            entries: [PageDirectoryEntry::empty(); ENTRIES_PER_TABLE],
        }
    }
}

static mut KERNEL_PAGE_DIRECTORY: PageDirectory = PageDirectory::new();

static mut FIRST_PAGE_TABLE: PageTable = PageTable::new();

static mut KERNEL_PAGE_TABLES: [PageTable; 4] = [
    PageTable::new(),
    PageTable::new(),
    PageTable::new(),
    PageTable::new(),
];

pub fn init() {
    unsafe {
        for i in 0..ENTRIES_PER_TABLE {
            let addr = (i * PAGE_SIZE) as u32;
            FIRST_PAGE_TABLE.entries[i] = PageTableEntry::new(addr, KERNEL_PAGE_FLAGS);
        }

        let first_pt_addr = &FIRST_PAGE_TABLE as *const _ as u32;
        KERNEL_PAGE_DIRECTORY.entries[0] =
            PageDirectoryEntry::new(first_pt_addr, KERNEL_PAGE_FLAGS);

        for (idx, table) in KERNEL_PAGE_TABLES.iter_mut().enumerate() {
            let base = ((idx + 1) * ENTRIES_PER_TABLE * PAGE_SIZE) as u32;

            for i in 0..ENTRIES_PER_TABLE {
                let addr = base + (i * PAGE_SIZE) as u32;
                table.entries[i] = PageTableEntry::new(addr, KERNEL_PAGE_FLAGS);
            }

            let table_addr = table as *const _ as u32;
            KERNEL_PAGE_DIRECTORY.entries[idx + 1] =
                PageDirectoryEntry::new(table_addr, KERNEL_PAGE_FLAGS);
        }

        let pd_addr = &KERNEL_PAGE_DIRECTORY as *const _ as u32;
        load_page_directory(pd_addr);
        enable_paging();
    }
}

pub unsafe fn load_page_directory(pd_addr: u32) {
    asm!(
        "mov cr3, {}",
        in(reg) pd_addr,
        options(nostack, preserves_flags)
    );
}

pub unsafe fn enable_paging() {
    asm!(
        "mov eax, cr0",
        "or eax, 0x80000000",
        "mov cr0, eax",
        out("eax") _,
        options(nostack)
    );
}

pub unsafe fn disable_paging() {
    asm!(
        "mov eax, cr0",
        "and eax, 0x7FFFFFFF",
        "mov cr0, eax",
        out("eax") _,
        options(nostack)
    );
}

pub fn flush_tlb_entry(addr: usize) {
    unsafe {
        asm!(
            "invlpg [{}]",
            in(reg) addr,
            options(nostack, preserves_flags)
        );
    }
}

pub fn flush_tlb() {
    unsafe {
        asm!(
            "mov eax, cr3",
            "mov cr3, eax",
            out("eax") _,
            options(nostack)
        );
    }
}

pub fn get_cr3() -> u32 {
    let cr3: u32;
    unsafe {
        asm!(
            "mov {}, cr3",
            out(reg) cr3,
            options(nostack, preserves_flags)
        );
    }
    cr3
}

pub fn get_cr0() -> u32 {
    let cr0: u32;
    unsafe {
        asm!(
            "mov {}, cr0",
            out(reg) cr0,
            options(nostack, preserves_flags)
        );
    }
    cr0
}

pub fn get_cr2() -> u32 {
    let cr2: u32;
    unsafe {
        asm!(
            "mov {}, cr2",
            out(reg) cr2,
            options(nostack, preserves_flags)
        );
    }
    cr2
}

pub fn is_paging_enabled() -> bool {
    get_cr0() & 0x80000000 != 0
}

pub fn map_page(virt_addr: usize, phys_addr: usize, flags: u32) -> bool {
    let pd_index = (virt_addr >> 22) & 0x3FF;
    let pt_index = (virt_addr >> 12) & 0x3FF;

    unsafe {
        if !KERNEL_PAGE_DIRECTORY.entries[pd_index].is_present() {
            if let Some(pt_frame) = pmm::alloc_frame() {
                let pt_ptr = pt_frame as *mut PageTable;
                core::ptr::write_bytes(pt_ptr, 0, 1);

                KERNEL_PAGE_DIRECTORY.entries[pd_index] =
                    PageDirectoryEntry::new(pt_frame as u32, flags | KERNEL_PAGE_FLAGS);
            } else {
                return false;
            }
        }
        let pt_addr = KERNEL_PAGE_DIRECTORY.entries[pd_index].table_addr() as *mut PageTable;
        (*pt_addr).entries[pt_index] = PageTableEntry::new(phys_addr as u32, flags);
        flush_tlb_entry(virt_addr);
    }

    true
}

pub fn unmap_page(virt_addr: usize) {
    let pd_index = (virt_addr >> 22) & 0x3FF;
    let pt_index = (virt_addr >> 12) & 0x3FF;

    unsafe {
        if KERNEL_PAGE_DIRECTORY.entries[pd_index].is_present() {
            let pt_addr = KERNEL_PAGE_DIRECTORY.entries[pd_index].table_addr() as *mut PageTable;
            (*pt_addr).entries[pt_index].clear();
            flush_tlb_entry(virt_addr);
        }
    }
}

pub fn get_physical_address(virt_addr: usize) -> Option<usize> {
    let pd_index = (virt_addr >> 22) & 0x3FF;
    let pt_index = (virt_addr >> 12) & 0x3FF;
    let offset = virt_addr & 0xFFF;

    unsafe {
        if !KERNEL_PAGE_DIRECTORY.entries[pd_index].is_present() {
            return None;
        }

        let pt_addr = KERNEL_PAGE_DIRECTORY.entries[pd_index].table_addr() as *const PageTable;
        let pte = (*pt_addr).entries[pt_index];

        if !pte.is_present() {
            return None;
        }

        Some((pte.frame_addr() as usize) + offset)
    }
}

pub fn get_kernel_page_directory() -> &'static PageDirectory {
    unsafe { &KERNEL_PAGE_DIRECTORY }
}
