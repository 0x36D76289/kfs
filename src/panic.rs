use crate::printk;
use crate::vga::Color;
use core::arch::asm;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PanicLevel {
    Warning,
    Error,
    Fatal,
}

#[inline(never)]
pub fn panic(message: &str) -> ! {
    panic_impl(message, PanicLevel::Fatal, None)
}

#[inline(never)]
pub fn panic_at(message: &str, file: &str, line: u32) -> ! {
    unsafe {
        asm!("cli", options(nomem, nostack));
    }

    printk::set_color(Color::White, Color::Red);
    printk::print("\n\n");
    printk::print(
        "================================================================================",
    );
    printk::print("\n");
    printk::print(
        "                            !!! KERNEL PANIC !!!                               ",
    );
    printk::print("\n");
    printk::print(
        "================================================================================",
    );
    printk::set_color(Color::Red, Color::Black);
    printk::print("\n\n");

    printk::set_color(Color::Yellow, Color::Black);
    printk::print("Message: ");
    printk::reset_color();
    printk::print(message);
    printk::print("\n\n");

    printk::set_color(Color::Yellow, Color::Black);
    printk::print("Location: ");
    printk::reset_color();
    printk::print(file);
    printk::print(":");
    printk::print_dec(line);
    printk::print("\n\n");

    print_registers();

    printk::set_color(Color::White, Color::Red);
    printk::print("\n");
    printk::print("System halted. Please reboot.");
    printk::print("\n");
    printk::reset_color();

    halt_loop()
}

fn panic_impl(message: &str, level: PanicLevel, location: Option<(&str, u32)>) -> ! {
    unsafe {
        asm!("cli", options(nomem, nostack));
    }

    printk::set_color(Color::White, Color::Red);
    printk::print("\n\n");
    printk::print(
        "================================================================================",
    );
    printk::print("\n");

    match level {
        PanicLevel::Fatal => {
            printk::print(
                "                            !!! KERNEL PANIC !!!                               ",
            );
        }
        PanicLevel::Error => {
            printk::print(
                "                            !!! KERNEL ERROR !!!                               ",
            );
        }
        PanicLevel::Warning => {
            printk::print(
                "                            !!! KERNEL WARNING !!!                             ",
            );
        }
    }

    printk::print("\n");
    printk::print(
        "================================================================================",
    );
    printk::set_color(Color::Red, Color::Black);
    printk::print("\n\n");

    printk::set_color(Color::Yellow, Color::Black);
    printk::print("Message: ");
    printk::reset_color();
    printk::print(message);
    printk::print("\n\n");

    if let Some((file, line)) = location {
        printk::set_color(Color::Yellow, Color::Black);
        printk::print("Location: ");
        printk::reset_color();
        printk::print(file);
        printk::print(":");
        printk::print_dec(line);
        printk::print("\n\n");
    }

    print_registers();

    printk::set_color(Color::White, Color::Red);
    printk::print("\n");
    printk::print("System halted. Please reboot.");
    printk::print("\n");
    printk::reset_color();

    halt_loop()
}

fn print_registers() {
    let eax: u32;
    let ebx: u32;
    let ecx: u32;
    let edx: u32;
    let esi: u32;
    let edi: u32;
    let ebp: u32;
    let esp: u32;

    unsafe {
        asm!(
            "mov {0}, eax",
            "mov {1}, ebx",
            "mov {2}, ecx",
            "mov {3}, edx",
            "mov {4}, esi",
            "mov {5}, edi",
            "mov {6}, ebp",
            "mov {7}, esp",
            out(reg) eax,
            out(reg) ebx,
            out(reg) ecx,
            out(reg) edx,
            out(reg) esi,
            out(reg) edi,
            out(reg) ebp,
            out(reg) esp,
        );
    }

    printk::set_color(Color::LightCyan, Color::Black);
    printk::print("CPU Registers:\n");
    printk::reset_color();

    printk::print("  EAX=");
    printk::print_hex_padded(eax);
    printk::print("  EBX=");
    printk::print_hex_padded(ebx);
    printk::print("  ECX=");
    printk::print_hex_padded(ecx);
    printk::print("  EDX=");
    printk::print_hex_padded(edx);
    printk::print("\n");

    printk::print("  ESI=");
    printk::print_hex_padded(esi);
    printk::print("  EDI=");
    printk::print_hex_padded(edi);
    printk::print("  EBP=");
    printk::print_hex_padded(ebp);
    printk::print("  ESP=");
    printk::print_hex_padded(esp);
    printk::print("\n");

    let cr0 = crate::memory::paging::get_cr0();
    let cr2 = crate::memory::paging::get_cr2();
    let cr3 = crate::memory::paging::get_cr3();

    printk::print("  CR0=");
    printk::print_hex_padded(cr0);
    printk::print("  CR2=");
    printk::print_hex_padded(cr2);
    printk::print("  CR3=");
    printk::print_hex_padded(cr3);
    printk::print("\n");
}

pub fn halt_loop() -> ! {
    loop {
        unsafe {
            asm!("cli", "hlt", options(nomem, nostack));
        }
    }
}

pub fn warn(message: &str) {
    printk::set_color(Color::Yellow, Color::Black);
    printk::print("[WARN] ");
    printk::reset_color();
    printk::print(message);
    printk::print("\n");
}

pub fn error(message: &str) {
    printk::set_color(Color::LightRed, Color::Black);
    printk::print("[ERROR] ");
    printk::reset_color();
    printk::print(message);
    printk::print("\n");
}

#[macro_export]
macro_rules! kassert {
    ($cond:expr) => {
        if !$cond {
            $crate::panic::panic_at(
                concat!("Assertion failed: ", stringify!($cond)),
                file!(),
                line!(),
            );
        }
    };
    ($cond:expr, $msg:expr) => {
        if !$cond {
            $crate::panic::panic_at($msg, file!(), line!());
        }
    };
}

#[macro_export]
macro_rules! kpanic {
    ($msg:expr) => {
        $crate::panic::panic_at($msg, file!(), line!())
    };
}
