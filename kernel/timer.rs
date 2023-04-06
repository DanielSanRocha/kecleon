use crate::screen;
use crate::memory;

const SYSTEM_TIMER_REGISTER: *mut u32 = 0x3F00B408 as *mut u32;

pub fn initialize() {
    memory::outq(SYSTEM_TIMER_REGISTER,0x00F90000,0);
    memory::outq(SYSTEM_TIMER_REGISTER,0x00F90200,0);
}

pub fn handler() {
    screen::print("Fired!", screen::RED);
}
