#![no_std]
#![feature(core_intrinsics, lang_items)]
#![feature(panic_info_message)]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn main() {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
