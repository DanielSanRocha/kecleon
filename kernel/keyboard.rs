use crate::screen;

pub fn initialize() {}

fn handler(c: u8) {
    screen::print_char(c, screen::VgaColor::Cyan);
}

#[no_mangle]
extern "C" fn keyboard_handler(c: u8) {
    handler(c);
}
