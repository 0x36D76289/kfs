#![no_std]
#![no_main]

mod vga_buffer;
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn k_main() -> ! {
	println!("42");

	loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{info}");
	loop {}
}
