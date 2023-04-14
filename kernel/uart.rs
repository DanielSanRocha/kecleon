use crate::memory;
use crate::screen;

const UART_DR: *mut u32 = 0x3F201000 as *mut u32;

pub fn print_char(c: u8) {
    memory::outq(UART_DR, c as u32, 0);
}

pub fn get_char() -> char {
    let c = memory::inq(UART_DR, 0) as u8 as char;

    if c == '\r' {
        '\n'
    } else {
        c
    }
}

pub fn print_int(n: u32) {
    let mut j = n;
    let mut c = 0 as u8;
    while j >= 10 {
        j = j / 10;
        c += 1;
    }
    print_int_loop(n, c + 1);
}

pub fn print(msg: &str) {
    for c in msg.chars() {
        print_char(c as u8);
    }
}

fn print_int_loop(i: u32, count: u8) {
    if count == 0 {
        return;
    }
    let mut j = i;
    let mut decimal = 1;
    for _ in 0..count - 1 {
        j = j / 10;
        decimal *= 10;
    }
    print_char(48 + j as u8);
    let new_i = i - decimal * j;
    if new_i == 0 {
        for _ in 0..count - 1 {
            print_char('0' as u8);
        }
    } else {
        print_int_loop(new_i, count - 1);
    }
}

#[no_mangle]
extern "C" fn uart_print(s: *const u8, length: u32) {
    unsafe {
        let mut index = 0;
        loop {
            print_char(*s.offset(index));
            index += 1;
            if index >= length as isize {
                break;
            }
        }
    }
}

#[no_mangle]
extern "C" fn uart_print_int(n: u32) {
    print_int(n);
}

pub fn schedule(_deltatime: u32) {
    if memory::inq(UART_DR, 6) & 0x10 != 0 {
        return;
    }

    let c = get_char();
    if c != '\0' {
        screen::putc(c, &screen::WHITE);
    }
}
