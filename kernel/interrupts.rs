const INTERRUPTS_REGISTER: *mut u32 = 0x3F00B200 as *mut u32;

use crate::memory;
use crate::screen;

extern "C" {
    fn enable_interrupts();
    fn move_vector_table();
}

pub fn initialize() {
    unsafe { move_vector_table(); }
    memory::outq(INTERRUPTS_REGISTER, 0xffffffff, 7);
    memory::outq(INTERRUPTS_REGISTER, 0xffffffff, 8);
    memory::outq(INTERRUPTS_REGISTER, 0xffffffff, 9);

    enable_interrupt(0);
}


#[no_mangle]
extern "C" fn irq_handler() {
    screen::print("Fired!", screen::RED);
}

pub fn enable() {
    unsafe { enable_interrupts(); }
}

fn enable_interrupt(number: u8) {
    memory::outq(INTERRUPTS_REGISTER, 1 << number, 7);
}
