#![no_std]
#![feature(core_intrinsics, lang_items)]
#![feature(panic_info_message)]

pub mod emmc;
pub mod ext2;
pub mod filesystem;
pub mod interrupts;
pub mod memory;
pub mod panic;
pub mod screen;
pub mod timer;
pub mod uart;

extern "C" {
    fn framebuffer_initialize() -> u32;
    fn hang();
}

#[no_mangle]
pub extern "C" fn main() {
    unsafe {
        uart::print("Starting Kernel...\n");

        uart::print("  Enabling MMU -> ");
        memory::initialize();
        uart::print("\nEnabled!\n");

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

        screen::print("  Framebuffer location -> ", screen::WHITE);
        screen::print_int(framebuffer as u32, screen::GREEN);

        screen::print("\n\n", screen::BLACK);

        screen::print("  Enabling Interrupts  -> ", screen::LIGHTBLUE);
        interrupts::initialize();
        interrupts::enable();
        screen::print("Enabled!\n", screen::GREEN);

        screen::print("  Initializing Timer   -> ", screen::LIGHTBLUE);
        timer::initialize();
        screen::print("Initialized!\n", screen::GREEN);

        screen::print("  Blinking the cursor  -> ", screen::LIGHTBLUE);
        timer::schedule(screen::blink_cursor, 500 * 1000);
        screen::print("Blinking!\n", screen::GREEN);

        screen::print("  Intializing EMMC     -> ", screen::LIGHTBLUE);
        emmc::initialize();
        screen::print("Intialized!\n", screen::GREEN);

        screen::print("  Intializing File System -> ", screen::LIGHTBLUE);
        filesystem::initialize();
        screen::print("Initialized!\n", screen::GREEN);

        let fd = filesystem::open("/boot/kernel.bin", 0);

        screen::print_int(fd as u32, screen::GREEN);

        hang();
    }
}
