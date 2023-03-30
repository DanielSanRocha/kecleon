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
