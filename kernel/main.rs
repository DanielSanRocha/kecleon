#![no_std]
#![no_main]
#![feature(panic_info_message)]

pub mod gdt;
pub mod harddisk;
pub mod idt;
pub mod keyboard;
pub mod memory;
pub mod panic;
pub mod process;
pub mod screen;

extern "C" {
    fn hlt();
}

#[no_mangle]
pub extern "C" fn main() {
    screen::initialize();

    screen::print(b"Welcome to ", screen::VgaColor::LightMagenta);
    screen::print(b"Kecleon", screen::VgaColor::LightGreen);
    screen::print(b" OS!\n", screen::VgaColor::LightCyan);
    screen::print(b"Booting...\n\n", screen::VgaColor::LightGrey);

    screen::print(b"GDT - Loading GDT table... ", screen::VgaColor::LightMagenta);
    gdt::initialize();
    screen::print(b"Loaded!", screen::VgaColor::Green);

    screen::print(b"\nIDT - Loading IDT table...", screen::VgaColor::LightMagenta);
    idt::initialize();
    screen::print(b" Loaded!", screen::VgaColor::Green);

    screen::print(b"\nKeyboard - Loading keyboard...", screen::VgaColor::LightMagenta);
    keyboard::initialize();
    screen::print(b" Loaded!", screen::VgaColor::Green);

    screen::print(b"\nMemory - Loading memory manager...", screen::VgaColor::LightMagenta);
    memory::initialize();
    screen::print(b" Loaded!", screen::VgaColor::Green);

    screen::print(b"\nProcess - Loading process manager...", screen::VgaColor::LightMagenta);
    process::initialize();
    screen::print(b" Loaded!", screen::VgaColor::Green);

    screen::print(b"\nHardDisk - Loading harddisk driver...", screen::VgaColor::LightMagenta);
    harddisk::initialize();
    screen::print(b" Loaded!", screen::VgaColor::Green);

    screen::print(b"\n\n>", screen::VgaColor::White);

    loop {
        unsafe {
            hlt();
        }
    }
}
