use crate::memory;

static mut FRAMEBUFFER: *mut u8 = 0x0 as *mut u8;

static mut CURRENT_X: isize = 0;
static mut CURRENT_Y: isize = 0;

static SCREEN_WIDTH: isize = 1600;
static SCREEN_HEIGHT: isize = 900;

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

static mut CURSOR_VISIBLE: u8 = 1;
pub fn blink_cursor(_deltatime: u32) {
    unsafe {
        if CURSOR_VISIBLE == 1 {
            draw_cursor(&WHITE);
        } else {
            draw_cursor(&BLACK);
        }

        CURSOR_VISIBLE = if CURSOR_VISIBLE == 0 { 1 } else { 0 };
    }
}

pub const RED: Pixel = Pixel { r: 255, g: 0, b: 0 };
pub const GREEN: Pixel = Pixel { r: 0, g: 255, b: 0 };
pub const BLUE: Pixel = Pixel { r: 0, g: 0, b: 255 };
pub const WHITE: Pixel = Pixel { r: 255, g: 255, b: 255 };
pub const BLACK: Pixel = Pixel { r: 0, g: 0, b: 0 };
pub const LIGHTBLUE: Pixel = Pixel { r: 100, g: 100, b: 255 };
pub const LIGHTRED: Pixel = Pixel { r: 200, g: 100, b: 100 };
pub const ORANGE: Pixel = Pixel { r: 219, g: 123, b: 65 };

pub fn draw_pixel(x: isize, y: isize, pix: &Pixel) {
    unsafe {
        let location = FRAMEBUFFER.offset(y * 3 * SCREEN_WIDTH + x * 3);

        *location.offset(0) = pix.r;
        *location.offset(1) = pix.g;
        *location.offset(2) = pix.b;
    }
}

pub fn print(s: &str, color: Pixel) {
    for c in s.chars() {
        putc(c, &color);
    }
}

pub fn print_string(s: *const u8, color: Pixel) {
    let mut i = 0 as isize;
    loop {
        unsafe {
            let c = *s.offset(i) as char;
            if c == '\0' {
                break;
            }

            putc(c, &color);
            i += 1;
        }
    }
}

pub fn print_int(n: u32, color: Pixel) {
    let mut j = n;
    let mut c = 0 as u8;
    while j >= 10 {
        j = j / 10;
        c += 1;
    }
    print_int_loop(n, c + 1, &color);
}

fn print_int_loop(i: u32, count: u8, color: &Pixel) {
    if count == 0 {
        return;
    }
    let mut j = i;
    let mut decimal = 1;
    for _ in 0..count - 1 {
        j = j / 10;
        decimal *= 10;
    }
    putc((48 + j as u8) as char, color);
    let new_i = i - decimal * j;
    if new_i == 0 {
        for _ in 0..count - 1 {
            putc('0', color);
        }
    } else {
        print_int_loop(new_i, count - 1, color);
    }
}

pub fn putc(c: char, color: &Pixel) {
    unsafe {
        draw_cursor(&BLACK);

        if c == '\n' {
            CURRENT_X = 0;
            CURRENT_Y += 1;

            if CURRENT_Y > SCREEN_HEIGHT / 16 - 2 {
                CURRENT_Y -= 1;
                scroll();
            }
        }

        let bmp = font(c as u8);

        for w in 0..8 {
            for h in 0..8 {
                let mask = 1 << w;
                if (*bmp.offset(h) & mask) != 0 {
                    draw_pixel(CURRENT_X * 16 + 2 * w, CURRENT_Y * 16 + 2 * h, color);
                    draw_pixel(CURRENT_X * 16 + 2 * w + 1, CURRENT_Y * 16 + 2 * h, color);
                    draw_pixel(CURRENT_X * 16 + 2 * w, CURRENT_Y * 16 + 2 * h + 1, color);
                    draw_pixel(CURRENT_X * 16 + 2 * w + 1, CURRENT_Y * 16 + 2 * h + 1, color);
                } else {
                    draw_pixel(CURRENT_X * 16 + 2 * w, CURRENT_Y * 16 + 2 * h, &BLACK);
                    draw_pixel(CURRENT_X * 16 + 2 * w + 1, CURRENT_Y * 16 + 2 * h, &BLACK);
                    draw_pixel(CURRENT_X * 16 + 2 * w, CURRENT_Y * 16 + 2 * h + 1, &BLACK);
                    draw_pixel(CURRENT_X * 16 + 2 * w + 1, CURRENT_Y * 16 + 2 * h + 1, &BLACK);
                }
            }
        }

        CURRENT_X += 1;
        if CURRENT_X >= SCREEN_WIDTH / 16 {
            CURRENT_X = 0;
            CURRENT_Y += 1;
        }

        if CURRENT_Y > SCREEN_HEIGHT / 16 - 2 {
            CURRENT_Y -= 1;
            scroll();
        }
    }
}

fn scroll() {
    unsafe {
        memory::memcopy(
            FRAMEBUFFER.offset(3 * SCREEN_WIDTH * 16),
            FRAMEBUFFER,
            3 * (SCREEN_HEIGHT - 1) * SCREEN_WIDTH,
        )
    }
}

fn draw_cursor(color: &Pixel) {
    unsafe {
        for i in 0..=16 {
            for j in 16..=19 {
                draw_pixel(CURRENT_X * 16 + i, CURRENT_Y * 16 + j, &color);
            }
        }
    }
}

pub fn syscall(c: u8, color: u32) {
    let r = color as u8;
    let g = (color >> 8) as u8;
    let b = (color >> 16) as u8;

    putc(c as char, &Pixel {r: r, g: g, b: b});
}
