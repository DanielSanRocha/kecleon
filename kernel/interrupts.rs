const INTERRUPTS_REGISTER: *mut u32 = 0x3F00B200 as *mut u32;

use crate::memory;
use crate::timer;
use crate::screen;

extern "C" {
    fn enable_interrupts();
    fn move_vector_table();
}

pub fn initialize() {
    unsafe {
        move_vector_table();
    }
    memory::outq(INTERRUPTS_REGISTER, 2, 4);
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

#[no_mangle]
extern "C" fn undefined_handler() {
    panic!("Undefined Handler Called!");
}

#[no_mangle]
extern "C" fn swi_handler(r0: u32, r1: u32, r2: u32) {
    if r0 == 0x1 {
        screen::syscall(r1 as u8, r2);
    } else {
        screen::print("Invalid system call called!", screen::RED);
    }
}

#[no_mangle]
extern "C" fn prefetch_handler() {
    panic!("Prefetch Handler Called!");
}

#[no_mangle]
extern "C" fn data_handler() {
    panic!("Data Handler Handler Called!");
}

#[no_mangle]
extern "C" fn unused_handler() {
    panic!("Unused Handler Called!");
}

#[no_mangle]
extern "C" fn fiq_handler() {
    panic!("FIQ Handler Called!");
}
