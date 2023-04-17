#![no_std]
#![feature(core_intrinsics, lang_items)]
#![feature(panic_info_message)]

pub mod emmc;
pub mod ext2;
pub mod filesystem;
pub mod interrupts;
pub mod keyboard;
pub mod memory;
pub mod panic;
pub mod process;
pub mod random;
pub mod screen;
pub mod timer;
pub mod uart;
pub mod usb;

extern "C" {
    fn get_cpsr() -> u32;
    fn framebuffer_initialize() -> u32;
    fn hang();
    fn goto_user_space();
}

fn log(message: &str, color: screen::Pixel) {
    uart::print(message);
    screen::print(message, color);
}

fn log_int(n: u32, color: screen::Pixel) {
    uart::print_int(n);
    screen::print_int(n, color);
}

// Machine Codes:
// 0 - Raspberry Pi 2B
// 1 - cubieboard2
#[no_mangle]
pub extern "C" fn main(machine: u16) {
    unsafe {
        if machine == 0 {
            uart::initialize(0x3F201000 as *mut u32);
        } else if machine == 1 {
            uart::initialize(0x01C28000 as *mut u32);
        } else {
            panic!("Invalid machine code!");
        }

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

        log("Welcome to ", screen::RED);
        log("Kecleon", screen::GREEN);
        log(" OS!\n", screen::WHITE);

        log("  Framebuffer location -> ", screen::WHITE);
        log_int(framebuffer as u32, screen::GREEN);
        log("\n", screen::BLACK);
        log("  Current CPSR         -> ", screen::WHITE);
        log_int(get_cpsr(), screen::GREEN);

        log("\n\n", screen::BLACK);

        log("  Enabling Interrupts  -> ", screen::LIGHTBLUE);
        interrupts::initialize();
        interrupts::enable();
        log("Enabled!\n", screen::GREEN);

        log("  Initializing Timer   -> ", screen::LIGHTBLUE);
        timer::initialize();
        log("Initialized!\n", screen::GREEN);

        log("  Blinking the cursor  -> ", screen::LIGHTBLUE);
        timer::schedule(screen::blink_cursor, 500 * 1000);
        log("Blinking!\n", screen::GREEN);

        log("  Initialing Random module -> ", screen::LIGHTBLUE);
        random::initialize(1);
        log("Initialized! Seed -> ", screen::GREEN);
        log_int(1, screen::WHITE);
        screen::putc('\n', &screen::BLACK);

        log("  Intializing EMMC     -> ", screen::LIGHTBLUE);
        emmc::initialize();
        log("Intialized!\n", screen::GREEN);

        log("  Intializing File System -> ", screen::LIGHTBLUE);
        filesystem::initialize();
        log("Initialized!\n", screen::GREEN);

        log("  Initializing USB Driver -> ", screen::LIGHTBLUE);
        usb::usb::initialize();
        log("  Initialized (not implemented yet)!\n", screen::GREEN);

        log("  Initializing Keyboard Driver (USB/UART) ->", screen::LIGHTBLUE);
        keyboard::initialize();
        log("  Initialized!\n", screen::GREEN);

        log("  Scheduling the UART check -> ", screen::LIGHTBLUE);
        timer::schedule(uart::schedule, 10 * 1000);
        log("  Scheduled!\n", screen::GREEN);

        log("  Initializing Processes  -> ", screen::LIGHTBLUE);
        interrupts::disable();
        process::initialize();
        let pid = process::start(b"/bin/shell".as_ptr(), b"".as_ptr(), 0);
        if (pid < 0) {
            panic!("Error starting shell!");
        }

        process::set_current(pid as u16);
        process::focus(pid as u16);
        log("Initialized!\n", screen::GREEN);

        log("  Starting process scheduler -> ", screen::LIGHTBLUE);
        timer::schedule(process::schedule, 1000 * 10);
        log("Started!\n", screen::GREEN);

        log("\n", screen::BLACK);
        goto_user_space();

        hang();
    }
}
