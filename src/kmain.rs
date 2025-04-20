#![no_std]
#![no_main]

use core::panic::PanicInfo;

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
// const VGA_WIDTH: usize = 80;
// const VGA_HEIGHT: usize = 25;

#[unsafe(no_mangle)]
pub extern "C" fn k_main() -> ! {
	let message = "42";
	
	// Print the message to the VGA text buffer
	for (i, &byte) in message.as_bytes().iter().enumerate() {
		unsafe {
			*VGA_BUFFER.add(i * 2) = byte;
			*VGA_BUFFER.add(i * 2 + 1) = 0x0F;
		}
	}

	loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}
