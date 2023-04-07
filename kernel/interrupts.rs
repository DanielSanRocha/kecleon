const INTERRUPTS_REGISTER: *mut u32 = 0x3F00B200 as *mut u32;

use crate::memory;
use crate::timer;

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
