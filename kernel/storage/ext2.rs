use crate::storage::driver::Driver;
use crate::storage::driver::EmptyDriver;
use crate::memory;

#[derive(Clone, Copy)]
pub struct SuperBlock {
    number_inodes: u32,
    number_blocks: u32,
    reserved_blocks: u32,
    unallocated_blocks: u32,
    unallocated_inodes: u32,
    start_superblock: u32,
    pub lblock_size: u32,
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
    pub dbp: [u32; 12],
    pub ibp: u32,
    pub dibp: u32,
    pub tibp: u32,
    pub garbage2: [u8; 28],
}

pub static mut SUPERBLOCK: *const SuperBlock = 0x0 as *const SuperBlock;
static mut BGD: *const BlockGroupDescriptor = 0x0 as *const BlockGroupDescriptor;
static mut BUFFER: *mut u8 = 0x0 as *mut u8;
static mut LBLOCK_SIZE: u32 = 0;
static mut SPB: u32 = 0;

static mut STORAGE: *const dyn Driver = &EmptyDriver {};

pub fn initialize(storage: *const dyn Driver) {
    unsafe {
        STORAGE = storage;
        SUPERBLOCK = memory::kmalloc(1024) as *const SuperBlock;
        (*STORAGE).readblock(2, SUPERBLOCK as *mut u8, 2);

        if (*SUPERBLOCK).magic_number != 0xef53 {
            panic!("Wrong magic number for Ext2!!");
        }

        LBLOCK_SIZE = 1 << (*SUPERBLOCK).lblock_size;
        SPB = 2 * LBLOCK_SIZE;

        BGD = memory::kmalloc(512 * SPB as isize) as *const BlockGroupDescriptor;
        (*STORAGE).readblock(2 * (*SUPERBLOCK).lblock_size + 4, BGD as *mut u8, SPB);

        BUFFER = memory::kmalloc(512 * SPB as isize) as *mut u8;
    }
}

pub fn get_inode(number: u32, inode: *mut Inode) {
    unsafe {
        let group = (number - 1) / (*SUPERBLOCK).inodes_per_group;
        let index = (number - 1) % (*SUPERBLOCK).inodes_per_group;
        let block = (index * 128) / (512 * SPB);

        let bgd = *BGD.offset(group as isize);

        (*STORAGE).readblock(SPB * bgd.inode_table + SPB * block, BUFFER, SPB);

        let inodes: *mut Inode = BUFFER as *mut Inode;
        *inode = *inodes.offset(index as isize);
    }
}

pub fn read_inode(inode: *const Inode, buffer: *mut u8, blocks: u32, offset: u32) {
    unsafe {
        if blocks == 0 {
            return;
        }

        if offset <= 11 {
            let bp = (*inode).dbp[offset as usize];
            if bp == 0 {
                return;
            }
            (*STORAGE).readblock(SPB * bp, buffer, SPB);
            read_inode(inode, buffer.offset(512 * SPB as isize), blocks - 1, offset + 1);
        } else {
            //TODO implement this function
        }
    }
}
