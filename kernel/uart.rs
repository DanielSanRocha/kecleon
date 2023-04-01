const UART_DR: *mut u32 = 0x101f1000 as *mut u32;

use crate::memory;

pub fn print_char(c: u8) {
    memory::outq(UART_DR, c as u32, 0);
}

pub fn get_char() -> u8 {
    memory::inq(UART_DR, 0) as u8
}

pub fn print(msg: &str) {
    for c in msg.chars() {
        print_char(c as u8);
    }
}