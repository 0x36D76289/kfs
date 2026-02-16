use crate::printk::{print, print_dec, print_hex_padded, println, reset_color, set_color};
use crate::vga::Color;
use core::arch::asm;

extern "C" {
    static stack_bottom: u8;
    static stack_top: u8;
}

#[inline(always)]
pub fn get_esp() -> u32 {
    let esp: u32;
    unsafe {
        asm!("mov {}, esp", out(reg) esp, options(nomem, nostack));
    }
    esp
}

#[inline(always)]
pub fn get_ebp() -> u32 {
    let ebp: u32;
    unsafe {
        asm!("mov {}, ebp", out(reg) ebp, options(nomem, nostack));
    }
    ebp
}

pub fn get_stack_bottom() -> u32 {
    unsafe { &stack_bottom as *const u8 as u32 }
}

pub fn get_stack_top() -> u32 {
    unsafe { &stack_top as *const u8 as u32 }
}

pub fn get_stack_size() -> u32 {
    get_stack_top() - get_stack_bottom()
}

pub fn get_stack_used() -> u32 {
    get_stack_top() - get_esp()
}

fn print_stack_header() {
    set_color(Color::LightCyan, Color::Black);
    println("========================================");
    println("         KERNEL STACK DUMP             ");
    println("========================================");
    reset_color();
}

fn print_stack_boundaries() {
    set_color(Color::Yellow, Color::Black);
    print("Stack Top:    ");
    reset_color();
    print_hex_padded(get_stack_top());
    println("");

    set_color(Color::Yellow, Color::Black);
    print("Stack Bottom: ");
    reset_color();
    print_hex_padded(get_stack_bottom());
    println("");

    set_color(Color::Yellow, Color::Black);
    print("Stack Size:   ");
    reset_color();
    print_dec(get_stack_size());
    println(" bytes");

    println("");
}

fn print_stack_pointers() {
    let esp = get_esp();
    let ebp = get_ebp();

    set_color(Color::LightGreen, Color::Black);
    print("ESP (Stack Pointer): ");
    reset_color();
    print_hex_padded(esp);
    println("");

    set_color(Color::LightGreen, Color::Black);
    print("EBP (Base Pointer):  ");
    reset_color();
    print_hex_padded(ebp);
    println("");

    set_color(Color::LightGreen, Color::Black);
    print("Stack Used:          ");
    reset_color();
    print_dec(get_stack_used());
    println(" bytes");

    println("");
}

pub fn print_stack_contents(num_entries: usize) {
    let esp = get_esp();
    let top = get_stack_top();

    set_color(Color::LightCyan, Color::Black);
    println("Stack Contents (from ESP upward):");
    println("----------------------------------");
    set_color(Color::DarkGray, Color::Black);
    println("  Address    | Value      | Offset");
    reset_color();

    let mut addr = esp;
    let mut count = 0;

    while addr < top && count < num_entries {
        let value = unsafe { *(addr as *const u32) };

        set_color(Color::DarkGray, Color::Black);
        print("  ");
        print_hex_padded(addr);
        print(" | ");

        reset_color();
        print_hex_padded(value);
        print(" | ");

        set_color(Color::DarkGray, Color::Black);
        print("+");
        print_dec((addr - esp) as u32);

        if addr == esp {
            set_color(Color::LightGreen, Color::Black);
            print(" <- ESP");
        }

        reset_color();
        println("");

        addr += 4;
        count += 1;
    }

    if addr >= top {
        set_color(Color::Yellow, Color::Black);
        println("  [End of stack reached]");
        reset_color();
    }
}

pub fn print_stack() {
    print_stack_header();
    print_stack_boundaries();
    print_stack_pointers();
    print_stack_contents(16);

    set_color(Color::LightCyan, Color::Black);
    println("========================================");
    reset_color();
}

pub fn print_stack_summary() {
    set_color(Color::LightCyan, Color::Black);
    print("[Stack] ");
    reset_color();

    print("ESP=");
    print_hex_padded(get_esp());
    print(" EBP=");
    print_hex_padded(get_ebp());
    print(" Used=");
    print_dec(get_stack_used());
    print("/");
    print_dec(get_stack_size());
    println(" bytes");
}

pub fn print_stack_trace() {
    set_color(Color::LightCyan, Color::Black);
    println("Stack Trace (EBP chain):");
    println("------------------------");
    reset_color();

    let bottom = get_stack_bottom();
    let top = get_stack_top();
    let mut ebp = get_ebp();
    let mut frame = 0;

    while ebp >= bottom && ebp < top && frame < 20 {
        let saved_ebp = unsafe { *(ebp as *const u32) };
        let return_addr = unsafe { *((ebp + 4) as *const u32) };

        set_color(Color::Yellow, Color::Black);
        print("  Frame ");
        print_dec(frame);
        reset_color();
        print(": EBP=");
        print_hex_padded(ebp);
        print(" Return=");
        print_hex_padded(return_addr);
        println("");

        if saved_ebp == 0 || saved_ebp <= ebp {
            break;
        }

        ebp = saved_ebp;
        frame += 1;
    }

    if frame == 0 {
        set_color(Color::DarkGray, Color::Black);
        println("  [No valid stack frames found]");
        reset_color();
    }
}
