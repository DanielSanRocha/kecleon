#![no_std]
#![feature(core_intrinsics, lang_items)]
#![feature(panic_info_message)]
#![no_main]

use core::panic::PanicInfo;

pub mod screen;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}