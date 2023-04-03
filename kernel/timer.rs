use crate::screen;
use crate::uart;

pub fn initialize() {}

#[no_mangle]
extern "C" fn timer_handler(n: u32) {
    screen::print("Fired!", screen::RED);
    screen::print_int(n, screen::LIGHTRED);
    uart::print("Fired!");
}

