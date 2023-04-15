#![no_std]
#![feature(core_intrinsics, lang_items)]
#![feature(panic_info_message)]

pub mod storage;
pub mod framebuffer;
pub mod interrupts;
pub mod keyboard;
pub mod memory;
pub mod panic;
pub mod process;
pub mod screen;
pub mod timer;
pub mod uart;
pub mod usb;

extern "C" {
    fn get_cpsr() -> u32;
    fn hang();
    fn goto_user_space();
}

// Machine Code:
//  1 - versatilepb (qemu)
#[no_mangle]
pub extern "C" fn main(machine: u32) {
    unsafe {
        uart::print("Starting Kernel...\n");

        uart::print("  Enabling MMU -> ");
        memory::initialize();
        uart::print("\nEnabled!\n");

        uart::print("Initializing framebuffer...");
        let framebuffer = framebuffer::initialize();

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
        screen::print("\n", screen::BLACK);
        screen::print("  Current CPSR         -> ", screen::WHITE);
        screen::print_int(get_cpsr(), screen::GREEN);

        screen::print("\n\n", screen::BLACK);

        screen::print("  Enabling Interrupts  -> ", screen::LIGHTBLUE);
        interrupts::initialize();
        interrupts::enable();
        screen::print("Enabled!\n", screen::GREEN);

        screen::print("  Initializing Timer   -> ", screen::LIGHTBLUE);
        timer::initialize();
        screen::print("Initialized!\n", screen::GREEN);

        screen::print("  Blinking the cursor  -> ", screen::LIGHTBLUE);
        timer::schedule(screen::blink_cursor, 500);
        screen::print("Blinking!\n", screen::GREEN);

        screen::print("  Intializing Storage Driver -> ", screen::LIGHTBLUE);

        let storage = if machine == 0 {
            screen::print("\n     Initializing HardDisk...\n", screen::WHITE);
            &storage::harddisk::HardDisk { addr: 0x0 as *mut u32 } as *const dyn storage::driver::Driver
        } else {
            screen::print("\n     Initializing EMMC...\n", screen::WHITE);
            &storage::emmc::EMMC { addr: 0x1000b000 as *mut u32 } as *const dyn storage::driver::Driver
        };

        (*storage).initialize();
        screen::print("Intialized!\n", screen::GREEN);

        screen::print("  Intializing File System -> ", screen::LIGHTBLUE);
        storage::filesystem::initialize(storage);
        screen::print("Initialized!\n", screen::GREEN);

        screen::print("  Initializing USB Driver -> ", screen::LIGHTBLUE);
        usb::usb::initialize();
        screen::print("  Initialized (not implemented yet)!\n", screen::GREEN);

        screen::print("  Initializing Keyboard Driver (USB/UART) ->", screen::LIGHTBLUE);
        keyboard::initialize();
        screen::print("  Initialized!\n", screen::GREEN);

        screen::print("  Scheduling the UART check -> ", screen::LIGHTBLUE);
        timer::schedule(uart::schedule, 500);
        screen::print("  Scheduled!\n", screen::GREEN);

        screen::print("  Initializing Processes  -> ", screen::LIGHTBLUE);
        interrupts::disable();
        process::initialize();
        let pid = process::start("/bin/shell", "I am a process!", 0);
        process::start("/bin/echo", "I am another process!", 0);
        process::set_current(pid);
        process::focus(pid);
        screen::print("Initialized!\n", screen::GREEN);

        screen::print("  Starting process scheduler -> ", screen::LIGHTBLUE);
        timer::schedule(process::schedule, 1000);
        screen::print("Started!\n", screen::GREEN);

        screen::print("\n", screen::BLACK);
        goto_user_space();

        hang();
    }
}
