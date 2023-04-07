use crate::screen;
use crate::memory;

const SYSTEM_TIMER_REGISTER: *mut u32 = 0x3F003000 as *mut u32;

pub fn initialize() {
    let x0 = memory::inq(SYSTEM_TIMER_REGISTER, 1);
    screen::print_int(x0, screen::LIGHTRED);

    memory::outq(SYSTEM_TIMER_REGISTER,x0 + 10000,4);
}

pub fn handler() {
    memory::outq(SYSTEM_TIMER_REGISTER, 2, 0);

    let x0 = memory::inq(SYSTEM_TIMER_REGISTER, 1);
    memory::outq(SYSTEM_TIMER_REGISTER,x0 + 10000,4);

    screen::print("Fire!", screen::RED);
}
