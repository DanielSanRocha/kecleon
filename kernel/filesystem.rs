use crate::emmc;
use crate::memory;
use crate::screen;

#[derive(Clone, Copy)]
struct Ext2SuperBlock {
    number_inodes: u32,
    number_blocks: u32,
    reserved_blocks: u32,
    unallocated_blocks: u32,
    unallocated_inodes: u32,
    start_superblock: u32,
    lblock_size: u32,
    lfragment_size: u32,
    block_per_group: u32,
    fragments_per_group: u32,
    inodes_per_group: u32,
    garbage: [u8; 12],
    magic_number: u16
}

#[derive(Clone, Copy)]
struct Ext2BlockGroupDescriptor {
    block_bitmap: u32,
    inode_bitmap: u32,
    inode_table: u32,
    garbage: [u8; 20]
}

#[repr(packed)]
#[derive(Clone, Copy)]
struct Ext2Inode {
    permission: u16,
    user: u16,
    lower_size: u32,
    last_access: u32,
    creation_time: u32,
    modification_time: u32,
    deletion_time: u32,
    group: u16,
    hard_links: u16,
    sectors_number: u32,
    flags: u32,
    garbage1: u32,
    dbp0: u32,
    dbp1: u32,
    dbp2: u32,
    dbp3: u32,
    dbp4: u32,
    dbp5: u32,
    dbp6: u32,
    dbp7: u32,
    dbp8: u32,
    dbp9: u32,
    dbp10: u32,
    dbp11: u32,
    garbage2: [u8; 40]
}

static mut SUPERBLOCK: *const Ext2SuperBlock = 0x0 as *const Ext2SuperBlock;
static mut BGD: *const Ext2BlockGroupDescriptor = 0x0 as *const Ext2BlockGroupDescriptor;
static mut BUFFER: *mut u8 = 0x0 as *mut u8;

pub fn initialize() {
    unsafe {
        SUPERBLOCK = memory::malloc(1024) as *const Ext2SuperBlock;
        emmc::readblock(2, SUPERBLOCK as *mut u8, 2);

        if (*SUPERBLOCK).magic_number != 0xef53 {
            panic!("Wrong magic number for Ext2!!");
        }

        BGD = memory::malloc(32 * 32) as *const Ext2BlockGroupDescriptor;
        emmc::readblock(4, BGD as *mut u8, 2);

        BUFFER = memory::malloc(1024) as *mut u8;
        let root: *mut Ext2Inode = memory::malloc(128) as *mut Ext2Inode;
        read_inode(2, root);

        emmc::readblock(2 * (*root).dbp0, BUFFER, 2);

        for i in 0..=255 {
            let c = *BUFFER.offset(i) as char;
            screen::putc(c, &screen::WHITE);
        }
    }
}

fn read_inode(number: u32, inode: *mut Ext2Inode) {
    unsafe {
        let group = (number - 1) / (*SUPERBLOCK).inodes_per_group;
        let index = (number - 1) % (*SUPERBLOCK).inodes_per_group;
        let block = (index * 128) / 1024;

        let bgd = *BGD.offset(group as isize);

        emmc::readblock(2 * bgd.inode_table + 2 * block, BUFFER, 2);

        let inodes: *mut Ext2Inode = BUFFER as *mut Ext2Inode;
        *inode = *inodes.offset(index as isize);
    }
}
