extern "C" {
    fn putc_syscall(c: u8, color: u32);
}

pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Pixel {
    pub fn to_u32(&self) -> u32 {
        self.r as u32 + ((self.g as u32) << 8) + ((self.b as u32) << 16)
    }
}

pub const RED: Pixel = Pixel {r: 0, g: 255, b: 0};
pub const GREEN: Pixel = Pixel {r: 255, g: 0, b: 0};
pub const BLUE: Pixel = Pixel {r: 0, g: 0, b: 255};

pub fn print(s: &str, color: &Pixel) {
    let color_u32 = color.to_u32();
    for c in s.chars() {
        unsafe{ putc_syscall(c as u8, color_u32); }
    }
}
