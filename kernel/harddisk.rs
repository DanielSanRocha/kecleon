use crate::memory;
use crate::screen;

const ATA_IO: *mut u8 = 0x1F0 as *mut u8;
const ATA_DATA: *mut u16 = 0x1F0 as *mut u16;

#[derive(Default)]
#[repr(C, packed)]
struct MasterBootRecord {
    magic_number: u32,
}

extern "C" {
    fn ata_wait_bsy();
    fn ata_wait_drq();
    fn ata_read_sectors(lba: u32, sector_count: u8);
}

pub fn initialize() {
    unsafe {
        ata_wait_bsy();
        ata_wait_drq();

        let _ = memory::inb(ATA_IO, 7);

        memory::outb(ATA_IO, 0xA0, 6);

        memory::outb(ATA_IO, 0, 2);
        memory::outb(ATA_IO, 0, 3);
        memory::outb(ATA_IO, 0, 4);
        memory::outb(ATA_IO, 0, 5);

        memory::outb(ATA_IO, 0xEC, 7);

        let status = memory::inb(ATA_IO, 7);

        if status == 0 {
            panic!("Primary ATA Driver not found!");
        }

        ata_wait_bsy();

        let x2 = memory::inb(ATA_IO, 2);
        let x3 = memory::inb(ATA_IO, 3);
        let x4 = memory::inb(ATA_IO, 4);
        let x5 = memory::inb(ATA_IO, 5);

        ata_wait_bsy();

        if x2 != 0 || x3 != 0 || x4 != 0 || x5 != 0 {
            panic!("Device at 0x1F0 is not ATA!");
        }

        ata_wait_bsy();
        ata_wait_drq();

        let name = memory::kmalloc(512);
        for i in 0..=255 {
            let cc = memory::u16_inb(ATA_DATA, 0);
            *(name as *mut u16).offset(i) = cc;
        }

        screen::print_string(name, screen::VgaColor::LightGreen);
    }
}

fn handler() {}

#[no_mangle]
pub extern "C" fn ata_handler() {
    handler();
}
