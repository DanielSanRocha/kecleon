use core::panic::PanicInfo;

use crate::uart;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    match (*_info).message() {
        Some(pmsg) => match pmsg.as_str() {
            Some(_message) => {
                uart::print("Kernel Panic: ");
                uart::print(_message);
                loop {}
            }
            None => loop {},
        },
        None => loop {},
    }
}
