use super::{align_up, KERNEL_HEAP_SIZE, KERNEL_HEAP_START};
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

const MIN_BLOCK_SIZE: usize = 32;

const ALIGNMENT: usize = 8;

#[repr(C)]
struct BlockHeader {
    size: usize,
    free: bool,
    magic: u32,
    next: *mut BlockHeader,
    prev: *mut BlockHeader,
}

const BLOCK_MAGIC: u32 = 0xDEADBEEF;

impl BlockHeader {
    fn is_valid(&self) -> bool {
        self.magic == BLOCK_MAGIC
    }
}

static HEAP_START: AtomicUsize = AtomicUsize::new(0);
static HEAP_END: AtomicUsize = AtomicUsize::new(0);
static HEAP_BREAK: AtomicUsize = AtomicUsize::new(0);
static HEAP_INITIALIZED: AtomicBool = AtomicBool::new(false);

static mut FREE_LIST: *mut BlockHeader = core::ptr::null_mut();

static HEAP_USED: AtomicUsize = AtomicUsize::new(0);

pub fn init() {
    let heap_start = KERNEL_HEAP_START;
    let heap_end = KERNEL_HEAP_START + KERNEL_HEAP_SIZE;

    HEAP_START.store(heap_start, Ordering::SeqCst);
    HEAP_END.store(heap_end, Ordering::SeqCst);
    HEAP_BREAK.store(heap_start, Ordering::SeqCst);

    unsafe {
        let first_block = heap_start as *mut BlockHeader;
        (*first_block).size = KERNEL_HEAP_SIZE;
        (*first_block).free = true;
        (*first_block).magic = BLOCK_MAGIC;
        (*first_block).next = core::ptr::null_mut();
        (*first_block).prev = core::ptr::null_mut();

        FREE_LIST = first_block;
    }

    HEAP_INITIALIZED.store(true, Ordering::SeqCst);
}

pub fn kmalloc(size: usize) -> Option<*mut u8> {
    if size == 0 || !HEAP_INITIALIZED.load(Ordering::SeqCst) {
        return None;
    }

    let total_size = align_up(size + core::mem::size_of::<BlockHeader>(), ALIGNMENT);
    let total_size = core::cmp::max(total_size, MIN_BLOCK_SIZE);

    unsafe {
        let mut current = FREE_LIST;

        while !current.is_null() {
            if (*current).free && (*current).size >= total_size {
                let remaining = (*current).size - total_size;

                if remaining >= MIN_BLOCK_SIZE {
                    let new_block = (current as usize + total_size) as *mut BlockHeader;
                    (*new_block).size = remaining;
                    (*new_block).free = true;
                    (*new_block).magic = BLOCK_MAGIC;
                    (*new_block).next = (*current).next;
                    (*new_block).prev = current;

                    if !(*current).next.is_null() {
                        (*(*current).next).prev = new_block;
                    }

                    (*current).next = new_block;
                    (*current).size = total_size;
                }

                (*current).free = false;
                HEAP_USED.fetch_add((*current).size, Ordering::SeqCst);

                let data_ptr = (current as usize + core::mem::size_of::<BlockHeader>()) as *mut u8;
                return Some(data_ptr);
            }

            current = (*current).next;
        }
    }

    None
}

pub fn kfree(ptr: *mut u8) {
    if ptr.is_null() || !HEAP_INITIALIZED.load(Ordering::SeqCst) {
        return;
    }

    unsafe {
        let header = (ptr as usize - core::mem::size_of::<BlockHeader>()) as *mut BlockHeader;

        if !(*header).is_valid() || (*header).free {
            return;
        }

        let block_size = (*header).size;
        (*header).free = true;
        HEAP_USED.fetch_sub(block_size, Ordering::SeqCst);

        if !(*header).next.is_null() && (*(*header).next).free {
            (*header).size += (*(*header).next).size;
            (*header).next = (*(*header).next).next;

            if !(*header).next.is_null() {
                (*(*header).next).prev = header;
            }
        }

        if !(*header).prev.is_null() && (*(*header).prev).free {
            (*(*header).prev).size += (*header).size;
            (*(*header).prev).next = (*header).next;

            if !(*header).next.is_null() {
                (*(*header).next).prev = (*header).prev;
            }
        }
    }
}

pub fn ksize(ptr: *const u8) -> usize {
    if ptr.is_null() || !HEAP_INITIALIZED.load(Ordering::SeqCst) {
        return 0;
    }

    unsafe {
        let header = (ptr as usize - core::mem::size_of::<BlockHeader>()) as *const BlockHeader;

        if (*header).is_valid() && !(*header).free {
            (*header).size - core::mem::size_of::<BlockHeader>()
        } else {
            0
        }
    }
}

pub fn kbrk(increment: isize) -> Option<*mut u8> {
    if !HEAP_INITIALIZED.load(Ordering::SeqCst) {
        return None;
    }

    let current_break = HEAP_BREAK.load(Ordering::SeqCst);
    let heap_end = HEAP_END.load(Ordering::SeqCst);

    if increment == 0 {
        return Some(current_break as *mut u8);
    }

    let new_break = if increment > 0 {
        current_break.checked_add(increment as usize)?
    } else {
        let dec = (-increment) as usize;
        if dec > current_break - HEAP_START.load(Ordering::SeqCst) {
            return None;
        }
        current_break - dec
    };

    if new_break > heap_end || new_break < HEAP_START.load(Ordering::SeqCst) {
        return None;
    }

    HEAP_BREAK.store(new_break, Ordering::SeqCst);
    Some(current_break as *mut u8)
}

pub fn krealloc(ptr: *mut u8, new_size: usize) -> Option<*mut u8> {
    if ptr.is_null() {
        return kmalloc(new_size);
    }

    if new_size == 0 {
        kfree(ptr);
        return None;
    }

    let old_size = ksize(ptr);
    if old_size == 0 {
        return None;
    }

    let aligned_new_size = align_up(new_size + core::mem::size_of::<BlockHeader>(), ALIGNMENT);
    if aligned_new_size <= old_size + core::mem::size_of::<BlockHeader>() {
        return Some(ptr);
    }

    let new_ptr = kmalloc(new_size)?;
    unsafe {
        core::ptr::copy_nonoverlapping(ptr, new_ptr, old_size);
    }
    kfree(ptr);

    Some(new_ptr)
}

pub fn kcalloc(count: usize, size: usize) -> Option<*mut u8> {
    let total = count.checked_mul(size)?;
    let ptr = kmalloc(total)?;

    unsafe {
        core::ptr::write_bytes(ptr, 0, total);
    }

    Some(ptr)
}

pub fn get_used() -> usize {
    HEAP_USED.load(Ordering::SeqCst)
}

pub fn get_free() -> usize {
    KERNEL_HEAP_SIZE - HEAP_USED.load(Ordering::SeqCst)
}

pub fn get_total() -> usize {
    KERNEL_HEAP_SIZE
}

pub fn count_blocks() -> (usize, usize) {
    let mut free_count = 0;
    let mut used_count = 0;

    unsafe {
        let mut current = FREE_LIST;
        while !current.is_null() {
            if (*current).free {
                free_count += 1;
            } else {
                used_count += 1;
            }
            current = (*current).next;
        }
    }

    (free_count, used_count)
}
