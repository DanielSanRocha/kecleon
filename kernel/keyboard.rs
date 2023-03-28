const KEYBOARD: *mut u8 = 0x60 as *mut u8;

use crate::memory;
use crate::screen;

pub fn initialize() {}

fn handler(c: u8) {
    screen::print_char(c, screen::VgaColor::Cyan);
}

#[no_mangle]
extern "C" fn keyboard_handler(c: u8) {
    handler(c);
}
