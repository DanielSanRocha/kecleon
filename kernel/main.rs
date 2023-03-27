#![no_std]
#![no_main]

pub mod panic;
pub mod memory;
pub mod screen;

#[no_mangle]
pub extern "C" fn main() -> ! {
    screen::clear();
    screen::print(b"Kecleon!!", screen::VgaColor::LightGreen);

    loop {}
}
