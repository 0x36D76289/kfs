use super::{addr_to_frame, align_up, frame_to_addr, PAGE_SIZE};
use core::sync::atomic::{AtomicUsize, Ordering};

const MAX_MEMORY: usize = 128 * 1024 * 1024;

const MAX_FRAMES: usize = MAX_MEMORY / PAGE_SIZE;

const BITMAP_SIZE: usize = MAX_FRAMES / 8;

static mut FRAME_BITMAP: [u8; BITMAP_SIZE] = [0; BITMAP_SIZE];

static TOTAL_FRAMES: AtomicUsize = AtomicUsize::new(0);

static USED_FRAMES: AtomicUsize = AtomicUsize::new(0);

static MEMORY_START: AtomicUsize = AtomicUsize::new(0);
static MEMORY_END: AtomicUsize = AtomicUsize::new(0);

#[repr(C, packed)]
struct MultibootMmapEntry {
    size: u32,
    base_addr: u64,
    length: u64,
    entry_type: u32,
}

#[repr(C, packed)]
struct MultibootInfo {
    flags: u32,
    mem_lower: u32,
    mem_upper: u32,
    boot_device: u32,
    cmdline: u32,
    mods_count: u32,
    mods_addr: u32,
    syms: [u32; 4],
    mmap_length: u32,
    mmap_addr: u32,
}

pub fn init(multiboot_info_addr: u32) {
    unsafe {
        for byte in FRAME_BITMAP.iter_mut() {
            *byte = 0xFF;
        }
    }

    if multiboot_info_addr != 0 {
        parse_multiboot_mmap(multiboot_info_addr);
    } else {
        let start = 0x100000; // 1MB
        let end = 0x1000000; // 16MB
        mark_region_free(start, end - start);
        MEMORY_START.store(start, Ordering::SeqCst);
        MEMORY_END.store(end, Ordering::SeqCst);
    }

    mark_region_used(0x100000, 0x100000); // 1MB
    mark_region_used(0, 0x100000);
    mark_region_used(0x800, PAGE_SIZE);
}

fn parse_multiboot_mmap(multiboot_info_addr: u32) {
    unsafe {
        let info = &*(multiboot_info_addr as *const MultibootInfo);

        if info.flags & (1 << 6) != 0 {
            let mmap_addr = info.mmap_addr as usize;
            let mmap_end = mmap_addr + info.mmap_length as usize;
            let mut addr = mmap_addr;

            let mut mem_start = usize::MAX;
            let mut mem_end = 0usize;

            while addr < mmap_end {
                let entry = &*(addr as *const MultibootMmapEntry);

                if entry.entry_type == 1 {
                    let base = entry.base_addr as usize;
                    let length = entry.length as usize;

                    if base < MAX_MEMORY {
                        let end = core::cmp::min(base + length, MAX_MEMORY);
                        mark_region_free(base, end - base);

                        if base < mem_start {
                            mem_start = base;
                        }
                        if end > mem_end {
                            mem_end = end;
                        }
                    }
                }
                addr += entry.size as usize + 4;
            }

            MEMORY_START.store(mem_start, Ordering::SeqCst);
            MEMORY_END.store(mem_end, Ordering::SeqCst);
        } else {
            let mem_upper_kb = info.mem_upper as usize;
            let total_mem = (mem_upper_kb + 1024) * 1024;
            let end = core::cmp::min(total_mem, MAX_MEMORY);

            mark_region_free(0x100000, end - 0x100000);
            MEMORY_START.store(0x100000, Ordering::SeqCst);
            MEMORY_END.store(end, Ordering::SeqCst);
        }
    }
}

fn mark_region_free(start: usize, length: usize) {
    let start_frame = addr_to_frame(align_up(start, PAGE_SIZE));
    let end_frame = addr_to_frame(start + length);

    for frame in start_frame..end_frame {
        if frame < MAX_FRAMES {
            clear_frame_bit(frame);
            TOTAL_FRAMES.fetch_add(1, Ordering::SeqCst);
        }
    }
}

fn mark_region_used(start: usize, length: usize) {
    let start_frame = addr_to_frame(start);
    let end_frame = addr_to_frame(align_up(start + length, PAGE_SIZE));

    for frame in start_frame..end_frame {
        if frame < MAX_FRAMES {
            if !test_frame_bit(frame) {
                set_frame_bit(frame);
                USED_FRAMES.fetch_add(1, Ordering::SeqCst);
            }
        }
    }
}

#[inline]
fn test_frame_bit(frame: usize) -> bool {
    unsafe {
        let byte = frame / 8;
        let bit = frame % 8;
        (FRAME_BITMAP[byte] & (1 << bit)) != 0
    }
}

#[inline]
fn set_frame_bit(frame: usize) {
    unsafe {
        let byte = frame / 8;
        let bit = frame % 8;
        FRAME_BITMAP[byte] |= 1 << bit;
    }
}

#[inline]
fn clear_frame_bit(frame: usize) {
    unsafe {
        let byte = frame / 8;
        let bit = frame % 8;
        FRAME_BITMAP[byte] &= !(1 << bit);
    }
}

fn find_free_frame() -> Option<usize> {
    unsafe {
        for (byte_idx, byte) in FRAME_BITMAP.iter().enumerate() {
            if *byte != 0xFF {
                for bit in 0..8 {
                    if (*byte & (1 << bit)) == 0 {
                        let frame = byte_idx * 8 + bit;
                        if frame < MAX_FRAMES {
                            return Some(frame);
                        }
                    }
                }
            }
        }
    }
    None
}

pub fn alloc_frame() -> Option<usize> {
    if let Some(frame) = find_free_frame() {
        set_frame_bit(frame);
        USED_FRAMES.fetch_add(1, Ordering::SeqCst);
        Some(frame_to_addr(frame))
    } else {
        None
    }
}

pub fn free_frame(addr: usize) {
    let frame = addr_to_frame(addr);
    if frame < MAX_FRAMES && test_frame_bit(frame) {
        clear_frame_bit(frame);
        USED_FRAMES.fetch_sub(1, Ordering::SeqCst);
    }
}

pub fn alloc_frames(count: usize) -> Option<usize> {
    if count == 0 {
        return None;
    }

    unsafe {
        let mut start_frame = 0;
        let mut found_count = 0;

        for frame in 0..MAX_FRAMES {
            if !test_frame_bit(frame) {
                if found_count == 0 {
                    start_frame = frame;
                }
                found_count += 1;

                if found_count == count {
                    for f in start_frame..(start_frame + count) {
                        set_frame_bit(f);
                    }
                    USED_FRAMES.fetch_add(count, Ordering::SeqCst);
                    return Some(frame_to_addr(start_frame));
                }
            } else {
                found_count = 0;
            }
        }
    }

    None
}

pub fn free_frames(addr: usize, count: usize) {
    let start_frame = addr_to_frame(addr);

    for frame in start_frame..(start_frame + count) {
        if frame < MAX_FRAMES && test_frame_bit(frame) {
            clear_frame_bit(frame);
            USED_FRAMES.fetch_sub(1, Ordering::SeqCst);
        }
    }
}

pub fn get_total_memory() -> usize {
    TOTAL_FRAMES.load(Ordering::SeqCst) * PAGE_SIZE
}

pub fn get_free_memory() -> usize {
    let total = TOTAL_FRAMES.load(Ordering::SeqCst);
    let used = USED_FRAMES.load(Ordering::SeqCst);
    (total.saturating_sub(used)) * PAGE_SIZE
}

pub fn get_used_memory() -> usize {
    USED_FRAMES.load(Ordering::SeqCst) * PAGE_SIZE
}

pub fn get_total_frames() -> usize {
    TOTAL_FRAMES.load(Ordering::SeqCst)
}

pub fn get_free_frames() -> usize {
    let total = TOTAL_FRAMES.load(Ordering::SeqCst);
    let used = USED_FRAMES.load(Ordering::SeqCst);
    total.saturating_sub(used)
}

pub fn get_used_frames() -> usize {
    USED_FRAMES.load(Ordering::SeqCst)
}
