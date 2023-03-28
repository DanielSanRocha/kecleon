const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const VGA_CURSOR: *mut u8 = 0x3D4 as *mut u8;

const VGA_CHARACTERS_PER_LINE: u16 = 80;
const VGA_NUMBER_OF_LINES: u16 = 25;

static mut CURRENT_POS: usize = 0;

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
    White = 15,
}

use crate::memory;

fn enable_cursor() {
    memory::outb(VGA_CURSOR, 0x0A, 1);
    let curstart = memory::inb(VGA_BUFFER, 1) & 0x1F;

    memory::outb(VGA_CURSOR, 0x0A, 0);
    memory::outb(VGA_CURSOR, curstart | 0x20, 1);
}

fn update_cursor(x: u16, y: u16) {
    let position = y * VGA_CHARACTERS_PER_LINE + x;

    memory::outb(VGA_CURSOR, 0x0F, 0);
    memory::outb(VGA_CURSOR, (position & 0xFF) as u8, 1);

    memory::outb(VGA_CURSOR, 0x0E, 0);
    memory::outb(VGA_CURSOR, ((position >> 8) & 0xFF) as u8, 1);
}

pub fn initialize() {
    unsafe {
        CURRENT_POS = 0;
        clear();
        enable_cursor();
        update_cursor(0, 0);
    }
}

pub fn clear() {
    for i in 0..=VGA_CHARACTERS_PER_LINE * VGA_NUMBER_OF_LINES - 1 {
        memory::outb(VGA_BUFFER, 0x0, i as isize * 2);
        memory::outb(VGA_BUFFER, 0x0, i as isize * 2 + 1);
    }
}

pub fn print(message: &[u8], color: VgaColor) {
    for (_, &byte) in message.iter().enumerate() {
        unsafe {
            if byte == '\n' as u8 {
                CURRENT_POS = ((CURRENT_POS as u16 / VGA_CHARACTERS_PER_LINE)
                    * VGA_CHARACTERS_PER_LINE
                    + VGA_CHARACTERS_PER_LINE) as usize
            } else {
                memory::outb(VGA_BUFFER, byte, (CURRENT_POS) as isize * 2);
                memory::outb(VGA_BUFFER, color as u8, (CURRENT_POS) as isize * 2 + 1);
                CURRENT_POS += 1;
            }
        }
    }
}
