use crate::memory;
use crate::screen;

const USB: *mut u32 = 0x3F980000 as *mut u32;

pub fn initialize() {
    let vendor = memory::inq(USB, 16);
    let userid = memory::inq(USB, 15);
    // let vendor = 1234;
    // let userid = 555;

    screen::print("\n    Vendor -> ", screen::WHITE);
    screen::print_int(vendor, screen::ORANGE);
    screen::print("\n    UserID -> ", screen::WHITE);
    screen::print_int(userid, screen::ORANGE);
    screen::print("\n", screen::BLACK);
}
