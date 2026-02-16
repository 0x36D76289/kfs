use super::{align_up, paging, pmm, PAGE_SIZE};
use super::{USER_SPACE_END, USER_SPACE_START};
use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy)]
pub struct VmRegion {
    pub start: usize,
    pub size: usize,
    pub pages: usize,
    pub used: bool,
}

impl VmRegion {
    pub const fn empty() -> Self {
        VmRegion {
            start: 0,
            size: 0,
            pages: 0,
            used: false,
        }
    }
}

const MAX_VM_REGIONS: usize = 256;
static mut VM_REGIONS: [VmRegion; MAX_VM_REGIONS] = [VmRegion::empty(); MAX_VM_REGIONS];
static NEXT_VADDR: AtomicUsize = AtomicUsize::new(USER_SPACE_START);
static TOTAL_VALLOC: AtomicUsize = AtomicUsize::new(0);

pub fn init() {
    NEXT_VADDR.store(USER_SPACE_START, Ordering::SeqCst);
}

pub fn vmalloc(size: usize) -> Option<*mut u8> {
    if size == 0 {
        return None;
    }

    let pages_needed = align_up(size, PAGE_SIZE) / PAGE_SIZE;
    let aligned_size = pages_needed * PAGE_SIZE;

    let region_idx = unsafe {
        let mut found = None;
        for (i, region) in VM_REGIONS.iter().enumerate() {
            if !region.used {
                found = Some(i);
                break;
            }
        }
        found
    }?;

    let vaddr = NEXT_VADDR.fetch_add(aligned_size, Ordering::SeqCst);

    if vaddr + aligned_size > USER_SPACE_END {
        NEXT_VADDR.fetch_sub(aligned_size, Ordering::SeqCst);
        return None;
    }

    for i in 0..pages_needed {
        let page_vaddr = vaddr + i * PAGE_SIZE;

        if let Some(frame) = pmm::alloc_frame() {
            if !paging::map_page(page_vaddr, frame, paging::USER_PAGE_FLAGS) {
                for j in 0..i {
                    let cleanup_vaddr = vaddr + j * PAGE_SIZE;
                    if let Some(phys) = paging::get_physical_address(cleanup_vaddr) {
                        pmm::free_frame(phys);
                    }
                    paging::unmap_page(cleanup_vaddr);
                }
                return None;
            }

            unsafe {
                core::ptr::write_bytes(page_vaddr as *mut u8, 0, PAGE_SIZE);
            }
        } else {
            for j in 0..i {
                let cleanup_vaddr = vaddr + j * PAGE_SIZE;
                if let Some(phys) = paging::get_physical_address(cleanup_vaddr) {
                    pmm::free_frame(phys);
                }
                paging::unmap_page(cleanup_vaddr);
            }
            return None;
        }
    }

    unsafe {
        VM_REGIONS[region_idx] = VmRegion {
            start: vaddr,
            size: aligned_size,
            pages: pages_needed,
            used: true,
        };
    }

    TOTAL_VALLOC.fetch_add(aligned_size, Ordering::SeqCst);

    Some(vaddr as *mut u8)
}

pub fn vfree(ptr: *mut u8) {
    if ptr.is_null() {
        return;
    }

    let addr = ptr as usize;

    unsafe {
        for region in VM_REGIONS.iter_mut() {
            if region.used && region.start == addr {
                for i in 0..region.pages {
                    let page_vaddr = region.start + i * PAGE_SIZE;
                    if let Some(phys) = paging::get_physical_address(page_vaddr) {
                        pmm::free_frame(phys);
                    }
                    paging::unmap_page(page_vaddr);
                }

                TOTAL_VALLOC.fetch_sub(region.size, Ordering::SeqCst);

                region.used = false;
                region.start = 0;
                region.size = 0;
                region.pages = 0;

                return;
            }
        }
    }
}

pub fn vsize(ptr: *const u8) -> usize {
    if ptr.is_null() {
        return 0;
    }

    let addr = ptr as usize;

    unsafe {
        for region in VM_REGIONS.iter() {
            if region.used && region.start == addr {
                return region.size;
            }
        }
    }

    0
}

pub fn vbrk(increment: isize) -> Option<*mut u8> {
    let current = NEXT_VADDR.load(Ordering::SeqCst);

    if increment == 0 {
        return Some(current as *mut u8);
    }

    let new_break = if increment > 0 {
        current.checked_add(increment as usize)?
    } else {
        current.checked_sub((-increment) as usize)?
    };

    if new_break < USER_SPACE_START || new_break > USER_SPACE_END {
        return None;
    }

    if increment > 0 {
        let pages_needed = align_up(increment as usize, PAGE_SIZE) / PAGE_SIZE;

        for i in 0..pages_needed {
            let vaddr = current + i * PAGE_SIZE;

            if let Some(frame) = pmm::alloc_frame() {
                if !paging::map_page(vaddr, frame, paging::USER_PAGE_FLAGS) {
                    for j in 0..i {
                        let cleanup_vaddr = current + j * PAGE_SIZE;
                        if let Some(phys) = paging::get_physical_address(cleanup_vaddr) {
                            pmm::free_frame(phys);
                        }
                        paging::unmap_page(cleanup_vaddr);
                    }
                    return None;
                }
            } else {
                return None;
            }
        }
    } else {
        let pages_to_free = align_up((-increment) as usize, PAGE_SIZE) / PAGE_SIZE;

        for i in 0..pages_to_free {
            let vaddr = new_break + i * PAGE_SIZE;
            if let Some(phys) = paging::get_physical_address(vaddr) {
                pmm::free_frame(phys);
            }
            paging::unmap_page(vaddr);
        }
    }

    NEXT_VADDR.store(new_break, Ordering::SeqCst);
    Some(current as *mut u8)
}

pub fn get_total_allocated() -> usize {
    TOTAL_VALLOC.load(Ordering::SeqCst)
}

pub fn get_region_count() -> usize {
    unsafe { VM_REGIONS.iter().filter(|r| r.used).count() }
}
