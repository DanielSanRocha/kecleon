static mut FRAMEBUFFER: *mut u8 = 0x0 as * mut u8;

pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

pub fn initialize(framebuffer: *mut u8) {
    unsafe {
        FRAMEBUFFER = framebuffer;
    }
}

pub fn draw_pixel(x: u32, y: u32, pix: &Pixel) {
    unsafe {
        let location = (FRAMEBUFFER as u32 + y * 24 * 1024 + x * 24) as *mut u8;

        *location.offset(0)  = pix.r;
        *location.offset(1) = pix.g;
        *location.offset(2) = pix.b;
    }
}
