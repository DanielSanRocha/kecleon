use crate::memory;
use crate::screen;
use crate::timer;

const USB: *mut u32 = 0x3F980000 as *mut u32;

#[repr(packed, C)]
#[derive(Clone, Copy)]
struct Device {
    number: u32,
    port: u8,
}

static mut DEVICES: *mut Device = 0x0 as *mut Device;

pub fn initialize() {
    unsafe {
        DEVICES = memory::kmalloc(5 * 32) as *mut Device;

        let vendor = memory::inq(USB, 16);
        let userid = memory::inq(USB, 15);

        screen::print("\n    Vendor -> ", screen::WHITE);
        screen::print_int(vendor, screen::ORANGE);
        screen::print("\n    UserID -> ", screen::WHITE);
        screen::print_int(userid, screen::ORANGE);
        screen::print("\n", screen::BLACK);

        memory::outq(USB, 0x4, 0);
        timer::sleep(1000 * 30);
        memory::outq(USB, 0x0, 0);
    }
}
