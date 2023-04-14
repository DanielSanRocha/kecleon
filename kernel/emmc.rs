use crate::memory;
use crate::timer;

const PL181: *mut u32 = 0x1000B000 as *mut u32;

pub fn initialize() {
    unsafe {
        memory::outq(PL181, 0x17, 0x0);
        // timer::sleep(1000 * 1000);
        loop {}
    }
}

pub fn readblock(lba: u32, buffer: *mut u8, num: u32) {
    // unsafe { sd_readblock(lba, buffer, num) }
}
