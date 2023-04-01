#![no_std]
#![feature(core_intrinsics, lang_items)]
#![feature(panic_info_message)]

pub mod panic;
pub mod memory;
pub mod uart;

#[no_mangle]
pub extern "C" fn main() {
    uart::print("Starting Kernel...");

    loop {}
}

