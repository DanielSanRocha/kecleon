#![no_std]
#![feature(core_intrinsics, lang_items)]
#![feature(panic_info_message)]
#![no_main]

use std;

#[no_mangle]
pub extern "C" fn main() {
    std::screen::print("Hello World!", &std::screen::GREEN);
}

