use crate::memory;
use crate::uart;

extern "C" {
    fn sd_init();
    fn sd_readblock(lba: u32, buffer: *mut u8, num: u32);
}

pub fn initialize() {
    unsafe {
        let buffer = memory::malloc(512) as *mut u8;
        sd_init();
        sd_readblock(2, buffer, 1);

        for i in 0..=511 {
            uart::print_int(*buffer.offset(i) as u32);
        }
    }
}
