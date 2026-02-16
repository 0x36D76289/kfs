#![no_std]
#![no_main]
#![allow(dead_code)]

mod gdt;
mod memory;
mod panic;
mod printk;
mod stack;
mod vga;

use core::panic::PanicInfo;
use vga::Color;

#[no_mangle]
pub extern "C" fn kernel_main(multiboot_magic: u32, multiboot_info: u32) -> ! {
    printk::init();
    printk::clear();

    printk::set_color(Color::LightGreen, Color::Black);
    printkln!("42");
    printkln!();
    printk::set_color(Color::LightCyan, Color::Black);
    printkln!("KFS - Kernel From Scratch v3");
    printkln!("============================");
    printk::reset_color();
    printkln!();

    if multiboot_magic != 0x2BADB002 {
        printk::set_color(Color::Yellow, Color::Black);
        printkln!("Warning: Invalid multiboot magic number");
        printk::reset_color();
    }

    printk::set_color(Color::Yellow, Color::Black);
    printkln!("Initializing GDT...");
    printk::reset_color();
    gdt::init();
    printk::set_color(Color::LightGreen, Color::Black);
    printkln!("GDT initialized successfully!");
    printk::reset_color();
    printkln!();

    printk::set_color(Color::Yellow, Color::Black);
    printkln!("Initializing memory management...");
    printk::reset_color();

    memory::init(multiboot_info);

    printk::set_color(Color::LightGreen, Color::Black);
    printkln!("Memory management initialized!");
    printk::reset_color();
    printkln!();

    print_memory_info();
    printkln!();

    print_paging_info();
    printkln!();

    test_memory_allocation();
    printkln!();

    stack::print_stack_summary();

    printkln!();
    printk::set_color(Color::DarkGray, Color::Black);
    printkln!("Kernel initialization complete. Halting...");
    printk::reset_color();

    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

fn print_memory_info() {
    use printk::{print, print_dec, print_hex_padded, println, reset_color, set_color};

    set_color(Color::LightCyan, Color::Black);
    println("========================================");
    println("          MEMORY INFORMATION           ");
    println("========================================");
    reset_color();

    let stats = memory::get_stats();

    set_color(Color::Yellow, Color::Black);
    print("Total Physical Memory: ");
    reset_color();
    print_dec((stats.total_memory / 1024) as u32);
    println(" KB");

    set_color(Color::Yellow, Color::Black);
    print("Used Physical Memory:  ");
    reset_color();
    print_dec((stats.used_memory / 1024) as u32);
    println(" KB");

    set_color(Color::Yellow, Color::Black);
    print("Free Physical Memory:  ");
    reset_color();
    print_dec((stats.free_memory / 1024) as u32);
    println(" KB");

    println("");

    set_color(Color::Yellow, Color::Black);
    print("Kernel Heap Size:      ");
    reset_color();
    print_dec((memory::KERNEL_HEAP_SIZE / 1024) as u32);
    println(" KB");

    set_color(Color::Yellow, Color::Black);
    print("Heap Used:             ");
    reset_color();
    print_dec(stats.heap_used as u32);
    println(" bytes");

    set_color(Color::Yellow, Color::Black);
    print("Heap Free:             ");
    reset_color();
    print_dec(stats.heap_free as u32);
    println(" bytes");

    set_color(Color::LightCyan, Color::Black);
    println("========================================");
    reset_color();
}

fn print_paging_info() {
    use printk::{print, print_dec, print_hex_padded, println, reset_color, set_color};

    set_color(Color::LightCyan, Color::Black);
    println("========================================");
    println("         PAGING INFORMATION            ");
    println("========================================");
    reset_color();

    set_color(Color::Yellow, Color::Black);
    print("Paging Enabled: ");
    reset_color();
    if memory::paging::is_paging_enabled() {
        set_color(Color::LightGreen, Color::Black);
        println("Yes");
    } else {
        set_color(Color::LightRed, Color::Black);
        println("No");
    }
    reset_color();

    set_color(Color::Yellow, Color::Black);
    print("Page Directory (CR3): ");
    reset_color();
    print_hex_padded(memory::paging::get_cr3());
    println("");

    set_color(Color::Yellow, Color::Black);
    print("CR0: ");
    reset_color();
    print_hex_padded(memory::paging::get_cr0());
    println("");

    set_color(Color::Yellow, Color::Black);
    print("Page Size: ");
    reset_color();
    print_dec(memory::PAGE_SIZE as u32);
    println(" bytes");

    set_color(Color::LightCyan, Color::Black);
    println("========================================");
    reset_color();
}

fn test_memory_allocation() {
    use printk::{print, print_dec, print_hex_padded, println, reset_color, set_color};

    set_color(Color::LightCyan, Color::Black);
    println("========================================");
    println("       MEMORY ALLOCATION TEST          ");
    println("========================================");
    reset_color();

    set_color(Color::Yellow, Color::Black);
    print("Testing kmalloc(64)... ");
    reset_color();

    if let Some(ptr) = memory::heap::kmalloc(64) {
        set_color(Color::LightGreen, Color::Black);
        print("OK ");
        reset_color();
        print("at ");
        print_hex_padded(ptr as u32);
        println("");

        let size = memory::heap::ksize(ptr);
        set_color(Color::Yellow, Color::Black);
        print("  ksize: ");
        reset_color();
        print_dec(size as u32);
        println(" bytes");

        unsafe {
            *ptr = 0x42;
            *(ptr.add(1)) = 0x43;
        }

        set_color(Color::Yellow, Color::Black);
        print("Testing kfree... ");
        reset_color();
        memory::heap::kfree(ptr);
        set_color(Color::LightGreen, Color::Black);
        println("OK");
        reset_color();
    } else {
        set_color(Color::LightRed, Color::Black);
        println("FAILED");
        reset_color();
    }

    set_color(Color::Yellow, Color::Black);
    print("Testing multiple allocations... ");
    reset_color();

    let mut allocs: [Option<*mut u8>; 4] = [None; 4];
    let sizes = [32, 64, 128, 256];
    let mut success = true;

    for i in 0..4 {
        allocs[i] = memory::heap::kmalloc(sizes[i]);
        if allocs[i].is_none() {
            success = false;
            break;
        }
    }

    if success {
        set_color(Color::LightGreen, Color::Black);
        println("OK");
        reset_color();

        for alloc in allocs.iter() {
            if let Some(ptr) = alloc {
                memory::heap::kfree(*ptr);
            }
        }
    } else {
        set_color(Color::LightRed, Color::Black);
        println("FAILED");
        reset_color();
    }

    let (free_blocks, used_blocks) = memory::heap::count_blocks();
    set_color(Color::Yellow, Color::Black);
    print("Heap blocks - Free: ");
    reset_color();
    print_dec(free_blocks as u32);
    set_color(Color::Yellow, Color::Black);
    print(" Used: ");
    reset_color();
    print_dec(used_blocks as u32);
    println("");

    set_color(Color::LightCyan, Color::Black);
    println("========================================");
    reset_color();
}

#[panic_handler]
fn rust_panic(info: &PanicInfo) -> ! {
    printk::set_color(Color::White, Color::Red);
    printkln!();
    printkln!("================================================================================");
    printkln!("                            !!! KERNEL PANIC !!!                               ");
    printkln!("================================================================================");
    printk::set_color(Color::Red, Color::Black);
    printkln!();

    if let Some(location) = info.location() {
        printk::set_color(Color::Yellow, Color::Black);
        printk!("Location: ");
        printk::reset_color();
        printk!(
            "{}:{}:{}",
            location.file(),
            location.line(),
            location.column()
        );
        printkln!();
    }

    if let Some(message) = info.message().as_str() {
        printk::set_color(Color::Yellow, Color::Black);
        printk!("Message: ");
        printk::reset_color();
        printk!("{}", message);
        printkln!();
    }

    printk::set_color(Color::White, Color::Red);
    printkln!();
    printkln!("System halted. Please reboot.");
    printk::reset_color();

    panic::halt_loop()
}

fn print_gdt_info() {
    use printk::{print, print_dec, print_hex_padded, println, reset_color, set_color};

    set_color(Color::LightCyan, Color::Black);
    println("========================================");
    println("     GLOBAL DESCRIPTOR TABLE (GDT)     ");
    println("========================================");
    reset_color();

    let (base, limit) = gdt::get_gdt_info();
    set_color(Color::Yellow, Color::Black);
    print("GDT Base Address: ");
    reset_color();
    print_hex_padded(base);
    println("");

    set_color(Color::Yellow, Color::Black);
    print("GDT Limit:        ");
    reset_color();
    print_dec(limit as u32);
    print(" bytes (");
    print_dec((limit as u32 + 1) / 8);
    println(" entries)");
    println("");

    set_color(Color::DarkGray, Color::Black);
    println("Index | Selector | Name         | Access | Flags");
    println("------|----------|--------------|--------|------");
    reset_color();

    let selectors = [
        gdt::selectors::NULL,
        gdt::selectors::KERNEL_CODE,
        gdt::selectors::KERNEL_DATA,
        gdt::selectors::KERNEL_STACK,
        gdt::selectors::USER_CODE,
        gdt::selectors::USER_DATA,
        gdt::selectors::USER_STACK,
    ];

    for i in 0..gdt::GDT_ENTRIES {
        let (name, access, flags) = gdt::describe_entry(i);

        set_color(Color::DarkGray, Color::Black);
        print("  ");
        print_dec(i as u32);
        print("   |   ");

        reset_color();
        print_hex_padded(selectors[i] as u32);
        print(" | ");

        if i == 0 {
            set_color(Color::DarkGray, Color::Black);
        } else if i <= 3 {
            set_color(Color::LightGreen, Color::Black);
        } else {
            set_color(Color::LightBlue, Color::Black);
        }

        print(name);
        for _ in name.len()..12 {
            print(" ");
        }

        reset_color();
        print(" | ");
        printk::print_byte_hex(access);
        print("   | ");
        printk::print_byte_hex(flags);
        println("");
    }

    set_color(Color::LightCyan, Color::Black);
    println("========================================");
    reset_color();
}
