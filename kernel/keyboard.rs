use crate::process;
use crate::screen;
use crate::uart;

pub fn initialize() {}

pub fn listener(key: u8) {
    process::putc(key);
}

pub fn syscall(number: u16, r1: u32, r2: u32) -> i32 {
    if number == 0x0 {
        process::getc() as i32
    } else {
        uart::print("Invalid keyboard systemcall called!");
        screen::print("Invalid keyboard systemcall called!", screen::RED);
        -1
    }
}
