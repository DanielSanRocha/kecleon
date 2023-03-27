#![no_std]
#![no_main]

pub mod panic;
pub mod video;

#[no_mangle]
pub extern "C" fn main() -> ! {
    video::clear();
    video::print(b"Hello Missigno!", video::VgaColor::White);

    loop {}
}
