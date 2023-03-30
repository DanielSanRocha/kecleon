use crate::memory;
use crate::screen;

const PCI_CONFIG_ADDRESS: *mut u32 = 0xCF8 as *mut u32;
const PCI_CONFIG_DATA: *mut u32 = 0xCFC as *mut u32;

pub fn initialize() {
    let fake_vendor_id = get_vendor_id(0, 0, 0);

    if memory::u32_inb(PCI_CONFIG_ADDRESS, 0) == 0 {
        panic!(" PCI is disabled!")
    }

    if (fake_vendor_id != 0xffffffff) && (fake_vendor_id != 0x0) {
        screen::print_char(' ' as u8, screen::VgaColor::Black);
        screen::print_int(fake_vendor_id, screen::VgaColor::White);
        panic!(" Something wrong with PCI!");
    }

    for bus in 0..=255 {
        for device_number in 0..=255 {
            for func in 0..=7 {
                let vendor = get_vendor_id(bus, device_number, func);

                if (vendor != 0xffffffff) & (vendor != 0) {
                    screen::print(b"\n     ", screen::VgaColor::Black);
                    screen::print(b"Detected Device -> ", screen::VgaColor::DarkGrey);
                    screen::print_int(vendor, screen::VgaColor::White);
                    screen::print_char('-' as u8, screen::VgaColor::White);
                    screen::print_int(vendor, screen::VgaColor::White);
                }
            }
        }
    }
}

fn get_vendor_id(bus: u8, device: u8, func: u8) -> u32 {
    let lbus = bus as u32;
    let ldevice = device as u32;
    let lfunc = func as u32;

    let address = (lbus << 16) | (ldevice << 11) | (lfunc << 8) | 0x80000000;

    memory::u32_outb(PCI_CONFIG_ADDRESS, address, 0);
    let read = memory::u32_inb(PCI_CONFIG_DATA, 0);

    read
}
