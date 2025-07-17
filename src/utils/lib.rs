#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

use core::panic::PanicInfo;

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[no_mangle]
pub extern "C" fn __stack_chk_fail() -> ! {
    panic!("Stack overflow detected");
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
