use core::panic::PanicInfo;

use crate::screen;
use crate::uart;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    match (*_info).message() {
        Some(pmsg) => match pmsg.as_str() {
            Some(message) => {
                uart::print("Kernel Panic: ");
                uart::print(message);

                screen::print("Kernel Panic: ", screen::RED);
                screen::print(message, screen::LIGHTRED);
                loop {}
            }
            None => loop {},
        },
        None => loop {},
    }
}
