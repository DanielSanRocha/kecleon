static mut FRAMEBUFFER: *mut u8 = 0x0 as *mut u8;

static mut CURRENT_X: isize = 0;
static mut CURRENT_Y: isize = 0;

extern "C" {
    pub fn font(c: u8) -> *const u8;
}

pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub fn initialize(framebuffer: *mut u8) {
    unsafe {
        FRAMEBUFFER = framebuffer;
    }
}

pub const RED: Pixel = Pixel { r: 255, g: 0, b: 0 };
pub const GREEN: Pixel = Pixel { r: 0, g: 255, b: 0 };
pub const BLUE: Pixel = Pixel { r: 0, g: 0, b: 255 };
pub const WHITE: Pixel = Pixel { r: 255, g: 255, b: 255 };
pub const BLACK: Pixel = Pixel { r: 0, g: 0, b: 0 };

pub fn draw_pixel(x: isize, y: isize, pix: &Pixel) {
    unsafe {
        let location = FRAMEBUFFER.offset(y * 3 * 1024 + x * 3);

        *location.offset(0) = pix.r;
        *location.offset(1) = pix.g;
        *location.offset(2) = pix.b;
    }
}

pub fn putc(c: char) {
    unsafe {
        let bmp = font(c as u8);

        for w in 0..8 {
            for h in 0..8 {
                let mask = 1 << w;
                if *bmp.offset(h) & mask != 0 {
                    draw_pixel(CURRENT_X * 16 + 2 * w, CURRENT_Y * 16 + 2 * h, &WHITE);
                    draw_pixel(CURRENT_X * 16 + 2 * w + 1, CURRENT_Y * 16 + 2 * h, &WHITE);
                    draw_pixel(CURRENT_X * 16 + 2 * w, CURRENT_Y * 16 + 2 * h + 1, &WHITE);
                    draw_pixel(CURRENT_X * 16 + 2 * w + 1, CURRENT_Y * 16 + 2 * h + 1, &WHITE);
                } else {
                    draw_pixel(CURRENT_X * 16 + 2 * w, CURRENT_Y * 16 + 2 * h, &BLACK);
                    draw_pixel(CURRENT_X * 16 + 2 * w + 1, CURRENT_Y * 16 + 2 * h, &BLACK);
                    draw_pixel(CURRENT_X * 16 + 2 * w, CURRENT_Y * 16 + 2 * h + 1, &BLACK);
                    draw_pixel(CURRENT_X * 16 + 2 * w + 1, CURRENT_Y * 16 + 2 * h + 1, &BLACK);
                }
            }
        }

        CURRENT_X += 1;
    }
}
