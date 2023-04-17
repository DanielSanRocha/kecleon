const INTERRUPTS_REGISTER: *mut u32 = 0x3F00B200 as *mut u32;

use crate::keyboard;
use crate::memory;
use crate::process;
use crate::screen;
use crate::timer;
use crate::uart;

extern "C" {
    fn disable_interrupts();
    fn enable_interrupts();
    fn move_vector_table();
}

pub fn initialize() {
    unsafe {
        move_vector_table();
    }
    memory::outq(INTERRUPTS_REGISTER, 3, 4);
}

#[no_mangle]
extern "C" fn irq_handler() {
    let pending = memory::inq(INTERRUPTS_REGISTER, 1);

    if (pending & 2) != 0 {
        timer::handler();
    }
}

pub fn enable() {
    unsafe {
        enable_interrupts();
    }
}

pub fn disable() {
    unsafe {
        disable_interrupts();
    }
}

#[no_mangle]
extern "C" fn undefined_handler() {
    panic!("Undefined Handler Called!");
}

#[no_mangle]
extern "C" fn swi_handler(r0: u32, r1: u32, r2: u32, r3: u32) -> i32 {
    let driver = (r0 & 0xFFFF) as u16;
    let number = r1 as u16;

    if driver == 0x0 {
        return process::syscall(number, r2, r3);
    } else if driver == 0x1 {
        return screen::syscall(number, r2, r3);
    } else if driver == 0x2 {
        return keyboard::syscall(number, r2, r3);
    } else {
        uart::print("Invalid system call called!");
        screen::print("Invalid system call called!", screen::RED);
        return -1;
    }
}

#[no_mangle]
extern "C" fn prefetch_handler() {
    panic!("Prefetch Handler Called!");
}

#[no_mangle]
extern "C" fn data_handler() {
    panic!("Data Handler Called!");
}

#[no_mangle]
extern "C" fn unused_handler() {
    panic!("Unused Handler Called!");
}

#[no_mangle]
extern "C" fn fiq_handler() {
    panic!("FIQ Handler Called!");
}
