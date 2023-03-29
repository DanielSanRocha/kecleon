use crate::memory;
use crate::screen;

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

// static mut BUFFER: *mut u8 = 0x0 as *mut u8;
const ATA: *mut u8 = 0x1F0 as *mut u8;

pub fn initialize() {
    unsafe {
        memory::outb(ATA, 0x40, 6);
        read_sectors(&mut SUPERBLOCK as *mut Ext2SuperBlock as *mut u8, 4, 4);

        if SUPERBLOCK.magic_number != 0xEF53 {
            screen::print(b"  Invalid EXT2 filesystem, kernel panic", screen::VgaColor::Red);
            loop {}
        }

        screen::print_int(SUPERBLOCK.magic_number as u32, screen::VgaColor::White);
    }
}

fn wait_bsy() {
    loop {
        let s = memory::inb(ATA, 7);
        screen::print_int(s as u32, screen::VgaColor::White);
        screen::print_char(' ' as u8, screen::VgaColor::White);

        if s & 0x80 == 0 {
            break;
        }
    }
}

fn wait_drq() {
    loop {
        if memory::inb(ATA, 7) & 0x40 != 0 {
            break;
        }
    }
}

fn read_sectors(ptr: *mut u8, lba: u32, sector_count: u8) {
    wait_bsy();

    memory::outb(ATA, (0xE0 | ((lba >> 24) & 0xF)) as u8, 6);
    memory::outb(ATA, sector_count, 2);
    memory::outb(ATA, lba as u8, 3);
    memory::outb(ATA, (lba >> 8) as u8, 4);
    memory::outb(ATA, (lba >> 16) as u8, 5);
    memory::outb(ATA, 0x20, 7);

    for j in 0..sector_count {
        wait_bsy();
        wait_drq();

        for i in 0..=255 {
            let c = memory::inb(ATA, 0);
            memory::outb(ptr, c, (i as u16 + j as u16 * 256) as isize);
        }
    }
}
