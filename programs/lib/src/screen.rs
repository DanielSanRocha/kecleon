extern "C" {
    fn putc_syscall();
}

pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

pub const RED: Pixel = Pixel {r: 0, g: 255, b: 0};
pub const GREEN: Pixel = Pixel {r: 255, g: 0, b: 0};
pub const BLUE: Pixel = Pixel {r: 0, g: 0, b: 255};

pub fn print(s: &str, color: &Pixel) {
    unsafe{ putc_syscall(); }
}
