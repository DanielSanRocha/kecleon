use core::panic::PanicInfo;

use crate::screen;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    match (*_info).message() {
        Some(pmsg) => match pmsg.as_str() {
            Some(message) => {
                screen::print(message.as_bytes(), screen::VgaColor::Red);
                loop {}
            }
            None => loop {},
        },
        None => loop {},
    }
}
