use crate::emmc;
use crate::memory;

#[derive(Clone, Copy)]
struct SuperBlock {
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
    magic_number: u16,
}

#[derive(Clone, Copy)]
struct BlockGroupDescriptor {
    block_bitmap: u32,
    inode_bitmap: u32,
    inode_table: u32,
    garbage: [u8; 20],
}

#[repr(packed)]
#[derive(Clone, Copy)]
pub struct Inode {
    pub permission: u16,
    pub user: u16,
    pub lower_size: u32,
    pub last_access: u32,
    pub creation_time: u32,
    pub modification_time: u32,
    pub deletion_time: u32,
    pub group: u16,
    pub hard_links: u16,
    pub sectors_number: u32,
    pub flags: u32,
    pub garbage1: u32,
    pub dbp0: u32,
    pub dbp1: u32,
    pub dbp2: u32,
    pub dbp3: u32,
    pub dbp4: u32,
    pub dbp5: u32,
    pub dbp6: u32,
    pub dbp7: u32,
    pub dbp8: u32,
    pub dbp9: u32,
    pub dbp10: u32,
    pub dbp11: u32,
    pub garbage2: [u8; 40],
}

static mut SUPERBLOCK: *const SuperBlock = 0x0 as *const SuperBlock;
static mut BGD: *const BlockGroupDescriptor = 0x0 as *const BlockGroupDescriptor;
static mut BUFFER: *mut u8 = 0x0 as *mut u8;
static mut ROOT: *mut Inode = 0x0 as *mut Inode;

pub fn initialize() {
    unsafe {
        SUPERBLOCK = memory::kmalloc(1024) as *const SuperBlock;
        emmc::readblock(2, SUPERBLOCK as *mut u8, 2);

        if (*SUPERBLOCK).magic_number != 0xef53 {
            panic!("Wrong magic number for Ext2!!");
        }

        BGD = memory::kmalloc(32 * 32) as *const BlockGroupDescriptor;
        emmc::readblock(4, BGD as *mut u8, 2);

        BUFFER = memory::kmalloc(1024) as *mut u8;

        ROOT = memory::kmalloc(128) as *mut Inode;
        get_inode(2, ROOT);
    }
}

pub fn get_inode(number: u32, inode: *mut Inode) {
    unsafe {
        let group = (number - 1) / (*SUPERBLOCK).inodes_per_group;
        let index = (number - 1) % (*SUPERBLOCK).inodes_per_group;
        let block = (index * 128) / 1024;

        let bgd = *BGD.offset(group as isize);

        emmc::readblock(2 * bgd.inode_table + 2 * block, BUFFER, 2);

        let inodes: *mut Inode = BUFFER as *mut Inode;
        *inode = *inodes.offset(index as isize);
    }
}

pub fn read_inode(inode: *const Inode, buffer: *mut u8, blocks: u8) {
    unsafe {
        if blocks == 0 {
            return;
        }
        if blocks > 0 {
            emmc::readblock(2 * (*inode).dbp0, buffer, 2);
        }
        if blocks > 1 {
            let dbp1 = (*inode).dbp1;
            if dbp1 == 0 {
                return;
            }
            emmc::readblock(2 * dbp1, buffer.offset(1024), 2);
        }
    }
}
