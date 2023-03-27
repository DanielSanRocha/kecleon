const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const CHARACTERS_PER_LINE: u16 = 100;
const NUMBER_OF_LINES: u16 = 50;

#[derive(Copy, Clone)]
pub enum VgaColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGrey = 7,
    DarkGrey = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    LightBrown = 14,
    White = 15
}

pub fn clear() {
    for i in 1..=CHARACTERS_PER_LINE * NUMBER_OF_LINES {
        unsafe {
            *VGA_BUFFER.offset(i as isize * 2) = 0x0;
            *VGA_BUFFER.offset(i as isize * 2) = 0x0;
        }
    }
}

pub fn print(message: &[u8], color: VgaColor) {
    for (i, &byte) in message.iter().enumerate() {
        unsafe {
            *VGA_BUFFER.offset(i as isize * 2) = byte;
            *VGA_BUFFER.offset(i as isize * 2 + 1) = color as u8;
        }
    }
}
