use crate::memory;

const LCD_CTR: *mut u32 = 0x1000001C as *mut u32;
const LCD: *mut u32 = 0x10120000 as *mut u32;

pub fn initialize() -> *mut u8 {
    memory::outq(LCD_CTR, 0x2CFC, 0);
    memory::outq(LCD, 0x1313a4fe, 0);
    memory::outq(LCD, 0x0505f67f, 1);
    memory::outq(LCD, 0x071F1800, 2);
    memory::outq(LCD, 250 * 1024 * 1024, 4);
    memory::outq(LCD, 0x82b, 6);

    return (250 * 1024 * 1024) as *mut u8;
}
