use crate::memory;
use crate::timer;
use crate::uart;
use crate::storage::driver::Driver;

pub struct HardDisk {
    pub addr: *mut u32,
}

impl HardDisk {
}

impl Driver for HardDisk {
    fn initialize(&self) {
    }

    fn readblock(&self, lba: u32, buffer: *mut u8, num: u32) {
    }
}
