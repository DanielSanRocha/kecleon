extern "C" {
    fn putc_syscall(c: u8, color: u32);
}

pub const RED: u32 = 0xFF;
pub const GREEN: u32 = 0xFF00;
pub const BLUE: u32 = 0xFF0000;
pub const WHITE: u32 = 0xFFFFFF;
pub const BLACK: u32 = 0x0;
pub const LIGHTBLUE: u32 = 0xff6464;
pub const LIGHTRED: u32 = 0x6464c8;
pub const ORANGE: u32 = 0x417bdb;

pub fn print(s: &str, color: u32) {
    for c in s.chars() {
        unsafe{
            putc_syscall(c as u8, color);
        }
    }
}
