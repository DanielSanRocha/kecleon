use crate::memory;
use crate::screen;

const ATA_IO: *mut u8 = 0x1F0 as *mut u8;

static mut ATA_PRIMARY_DETECTED: u8 = 0;
static mut ATA_DRIVE: u8 = 0;

#[derive(Default)]
#[repr(C, packed)]
struct MasterBootRecord {
    magic_number: u32,
}

#[derive(Default)]
#[repr(C, packed)]
struct Ext2SuperBlock {
    inodes_count: u32,
    block_count: u32,
    block_count_reserved: u32,
    block_count_unallocated: u32,
    inodes_count_unallocated: u32,
    lblock_size: u32,
    lfragment_size: u32,
    block_per_group: u32,
    inodes_per_group: u32,
    garbage: [u8; 19],
    magic_number: u16,
}

static mut SUPERBLOCK: Ext2SuperBlock = Ext2SuperBlock {
    inodes_count: 0,
    block_count: 0,
    block_count_reserved: 0,
    block_count_unallocated: 0,
    inodes_count_unallocated: 0,
    lblock_size: 0,
    lfragment_size: 0,
    block_per_group: 0,
    inodes_per_group: 0,
    garbage: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    magic_number: 0,
};

pub fn initialize() {
    detect_ata();
    let mbr = memory::kmalloc(1024) as *mut MasterBootRecord;

    wait_bsy();
    screen::print(b"Hello World!", screen::VgaColor::Blue);

    unsafe {
        screen::print_int((*mbr).magic_number, screen::VgaColor::White);
        read_sectors(mbr as *mut u8, 0, 4);
        screen::print_int((*mbr).magic_number, screen::VgaColor::White);

        if SUPERBLOCK.magic_number != 0xef53 {
            panic!(" Invalid EXT2 filesystem. Kernel Panic!!!");
        }
    }
}

fn wait_bsy() {
    loop {
        if (memory::inb(ATA_IO, 7) & 0x80) != 0x80 {
            break;
        }
    }
}

fn wait_drq() {
    loop {
        if memory::inb(ATA_IO, 7) & 0x40 == 0x40 {
            break;
        }
    }
}

fn detect_ata() {
    memory::outb(ATA_IO, 0x88, 3);
    unsafe {
        ATA_DRIVE = memory::inb(ATA_IO, 3);

        if ATA_DRIVE == 0x88 {
            memory::outb(ATA_IO, 0xA0, 6);
            memory::outb(ATA_IO, 0, 2);
            memory::outb(ATA_IO, 0, 3);
            memory::outb(ATA_IO, 0, 4);
            memory::outb(ATA_IO, 0, 5);

            memory::outb(ATA_IO, 0xEC, 7);
            ATA_DRIVE = memory::inb(ATA_IO, 7);
            if ATA_DRIVE > 0 {
                ATA_PRIMARY_DETECTED = 1;
                screen::print(b"Detected primary ATA - Drive: ", screen::VgaColor::White);
                screen::print_int(ATA_DRIVE as u32, screen::VgaColor::Cyan);
                screen::print_char(' ' as u8, screen::VgaColor::Black);
            }
        }
    }
}

fn read_sectors(ptr: *mut u8, lba: u32, sector_count: u8) {
    unsafe {
        if ATA_PRIMARY_DETECTED == 0 {
            panic!("ATA drive not detected!");
        }
    }

    wait_bsy();

    memory::outb(ATA_IO, (0xE0 | ((lba >> 24) & 0xF)) as u8, 6);
    memory::outb(ATA_IO, sector_count, 2);
    memory::outb(ATA_IO, lba as u8, 3);
    memory::outb(ATA_IO, (lba >> 8) as u8, 4);
    memory::outb(ATA_IO, (lba >> 16) as u8, 5);
    memory::outb(ATA_IO, 0x40, 6);
    memory::outb(ATA_IO, 0x20, 7);

    screen::print(b" Reading sectors...", screen::VgaColor::White);
    for j in 0..sector_count {
        wait_bsy();
        wait_drq();

        for i in 0..=255 {
            let c = memory::inb(ATA_IO, 0);
            memory::outb(ptr, c, (i as u32 + j as u32 * 256) as isize);
        }
    }
}
