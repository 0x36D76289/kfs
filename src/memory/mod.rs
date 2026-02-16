pub mod heap;
pub mod paging;
pub mod pmm;
pub mod vmm;

pub const PAGE_SIZE: usize = 4096;

pub const KERNEL_SPACE_START: usize = 0x00100000; // 1MB
pub const KERNEL_SPACE_END: usize = 0x00400000; // 4MB
pub const USER_SPACE_START: usize = 0x00400000; // 4MB
pub const USER_SPACE_END: usize = 0xC0000000; // 3GB
pub const KERNEL_HEAP_START: usize = 0x00200000; // 2MB
pub const KERNEL_HEAP_SIZE: usize = 0x00100000; // 1MB

#[inline]
pub const fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

#[inline]
pub const fn align_down(addr: usize, align: usize) -> usize {
    addr & !(align - 1)
}

#[inline]
pub const fn is_aligned(addr: usize, align: usize) -> bool {
    addr & (align - 1) == 0
}

#[inline]
pub const fn addr_to_frame(addr: usize) -> usize {
    addr / PAGE_SIZE
}

#[inline]
pub const fn frame_to_addr(frame: usize) -> usize {
    frame * PAGE_SIZE
}

pub fn init(multiboot_info: u32) {
    pmm::init(multiboot_info);
    paging::init();
    heap::init();
    vmm::init();
}

pub fn get_stats() -> MemoryStats {
    MemoryStats {
        total_memory: pmm::get_total_memory(),
        free_memory: pmm::get_free_memory(),
        used_memory: pmm::get_used_memory(),
        heap_used: heap::get_used(),
        heap_free: heap::get_free(),
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MemoryStats {
    pub total_memory: usize,
    pub free_memory: usize,
    pub used_memory: usize,
    pub heap_used: usize,
    pub heap_free: usize,
}
