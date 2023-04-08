#![no_std]
#![feature(core_intrinsics, lang_items)]
#![feature(panic_info_message)]
#![no_main]

use stdkecleon;

#[no_mangle]
pub extern "C" fn main() {
    stdkecleon::screen::print("Hello World!", &stdkecleon::screen::GREEN);
}

