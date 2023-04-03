#![no_std]
#![feature(core_intrinsics, lang_items)]
#![feature(panic_info_message)]

pub mod memory;
pub mod panic;
pub mod screen;
pub mod uart;

extern "C" {
    pub fn framebuffer_initialize() -> u32;
}

#[no_mangle]
pub extern "C" fn main() {
    unsafe {
        uart::print("Starting Kernel...\n");

        uart::print("Initializing framebuffer...");
        let framebuffer = framebuffer_initialize() as *mut u8;

        if framebuffer as u32 == 0 {
            panic!("Erro initializing the framebuffer!");
        }
        uart::print("\nInitialized!\n");

        uart::print("Initialing screen driver...");
        screen::initialize(framebuffer);
        uart::print("\nInitialized!\n");

        screen::draw_pixel(10, 10, &screen::Pixel {r: 255, g: 0, b: 0});

    }
}