use crate::memory;

extern "C" {
    fn sd_init();
    fn sd_readblock(lba: u32, buffer: *mut u8, num: u32);
}

pub fn initialize() {
    unsafe {
        let buffer = memory::kmalloc(512) as *mut u8;
        sd_init();
        sd_readblock(2, buffer, 1);
    }
}

pub fn readblock(lba: u32, buffer: *mut u8, num: u32) {
    unsafe { sd_readblock(lba, buffer, num) }
}
