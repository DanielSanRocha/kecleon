static mut CURRENT: u32 = 0;

pub fn initialize(seed: u32) {
    unsafe {
        CURRENT = seed;
    }
}

pub fn u16() -> u16 {
    unsafe {
        CURRENT = (CURRENT * 110345 + 12345) % 1048576;
        CURRENT as u16
    }
}
