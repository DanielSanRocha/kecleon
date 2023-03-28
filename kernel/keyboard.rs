const KEYBOARD: *mut u8 = 0x60 as *mut u8;

use crate::memory;
use crate::screen;

pub fn initialize() {
    // memory::outb(KEYBOARD, 0xFE, 4);
}

fn handler(c: u8) {
    screen::print(b"\nKey fired!: ", screen::VgaColor::Brown);
    screen::print_int(c as u32, screen::VgaColor::Cyan);
}

#[no_mangle]
extern "C" fn keyboard_handler() {
    let c = memory::inb(KEYBOARD, 0);
    handler(c);
}
