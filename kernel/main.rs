#![no_std]
#![feature(core_intrinsics, lang_items)]
#![feature(panic_info_message)]

pub mod memory;
pub mod panic;
pub mod screen;
pub mod timer;
pub mod uart;

extern "C" {
    fn framebuffer_initialize() -> u32;
    // fn interrupts_initialize();
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

        screen::print("Welcome to ", screen::RED);
        screen::print("Kecleon", screen::GREEN);
        screen::print(" OS!\n", screen::WHITE);

        screen::print("Framebuffer location -> ", screen::WHITE);
        screen::print_int(framebuffer as u32, screen::LIGHTBLUE);

        screen::print("\n\n", screen::BLACK);

        // screen::print("  Enabling Intererupts -> ", screen::LIGHTBLUE);
        // interrupts_initialize();
        // screen::print("Enabled!\n", screen::GREEN);

        screen::print("  Initializing Timer   -> ", screen::LIGHTBLUE);
        timer::initialize();
        screen::print("Initialized!\n", screen::GREEN);

        loop {}
    }
}
