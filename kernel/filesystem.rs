use crate::ext2;
use crate::memory;

#[repr(packed)]
#[derive(Clone, Copy)]
struct File {
    id: u16,
    inode: u32,
    process: u16,
    offset: u32,
}

static mut FILES: *mut File = 0x0 as *mut File;
static mut INODE_BUFFER: *mut ext2::Inode = 0x0 as *mut ext2::Inode;
static mut BLOCK_BUFFER: *mut u8 = 0x0 as *mut u8;

pub fn initialize() {
    ext2::initialize();
    unsafe {
        FILES = memory::kmalloc(12 * 256) as *mut File;

        for i in 0..=255 {
            (*FILES.offset(i)).id = 0;
            (*FILES.offset(i)).inode = 0;
            (*FILES.offset(i)).offset = 0;
            (*FILES.offset(i)).process = 0;
        }

        INODE_BUFFER = memory::kmalloc(128) as *mut ext2::Inode;
        BLOCK_BUFFER = memory::kmalloc(64 * 1024) as *mut u8;
    }
}

pub fn size(fd: u16) -> u32 {
    unsafe {
        for i in 0..=255 {
            if (*FILES.offset(i)).id == fd {
                let file = *FILES.offset(i);
                ext2::get_inode(file.inode, INODE_BUFFER);

                return (*INODE_BUFFER).lower_size;
            }
        }

        return 0;
    }
}

pub fn read(fd: u16, buffer: *mut u8, nblocks: u32) -> u32 {
    unsafe {
        for i in 0..=255 {
            if (*FILES.offset(i)).id == fd {
                let file = *FILES.offset(i);
                ext2::get_inode(file.inode, INODE_BUFFER);

                if nblocks == 0 {
                    return 0;
                }

                ext2::read_inode(INODE_BUFFER, buffer, nblocks, 0);
            }
        }

        return 0;
    }
}

pub fn open(path: *const u8, process: u16) -> i32 {
    unsafe {
        if (*path.offset(0)) == 0 {
            return -1;
        }

        if (*path.offset(0)) != '/' as u8 {
            return -2;
        }

        unsafe {
            let fd = open_recursion(2, path.offset(1), process);
            fd as i32
        }
    }
}

fn open_recursion(root: u32, path: *const u8, process: u16) -> u16 {
    unsafe {
        let mut size = 0 as isize;
        loop {
            if (*path.offset(size)) == 0 {
                break;
            }
            size += 1;
        }

        ext2::get_inode(root, INODE_BUFFER);

        if (*INODE_BUFFER).permission & 0x4000 == 0 {
            return 0;
        }
        memory::memset(BLOCK_BUFFER, 0x0, 64 * 1024);
        ext2::read_inode(INODE_BUFFER, BLOCK_BUFFER, 64, 0);

        let mut i = 0;
        loop {
            let inode = *(BLOCK_BUFFER.offset(i) as *mut u32);
            if inode == 0 {
                return 0;
            }

            let namesize = *BLOCK_BUFFER.offset(i + 6);
            let mut flag = 1 as u8;

            if size >= namesize as isize {
                for j in 0..namesize {
                    let c = *BLOCK_BUFFER.offset(8 + j as isize + i as isize) as char;

                    if *path.offset(j as isize) != c as u8 {
                        flag = 0;
                        break;
                    }
                }

                if flag == 1 {
                    if namesize == size as u8 {
                        return create_fd(inode, process);
                    }

                    if (*path.offset(namesize as isize)) == '/' as u8 {
                        let new_path = path.offset(namesize as isize + 1);
                        return open_recursion(inode, new_path, process);
                    }
                }
            }

            let entrysize: u16 = *(BLOCK_BUFFER.offset(i) as *mut u16).offset(2);
            i += entrysize as isize;
        }
    }
}

fn new_id() -> u16 {
    let mut nid = 1 as u16;
    let mut i;

    unsafe {
        loop {
            i = 0;
            loop {
                let id = (*FILES.offset(i)).id;
                if nid == id {
                    nid += 1;
                    break;
                }
                i += 1;
                if i == 256 {
                    return nid;
                }
            }
        }
    }
}

fn create_fd(inode: u32, process: u16) -> u16 {
    for i in 0..=255 {
        unsafe {
            let file = *FILES.offset(i);
            if file.id == 0 {
                let id = new_id();
                (*FILES.offset(i)).id = id;
                (*FILES.offset(i)).inode = inode;
                (*FILES.offset(i)).offset = 0;
                (*FILES.offset(i)).process = process;

                return id;
            }
        }
    }

    panic!("No more room for file descriptors!")
}

pub fn block_size() -> u32 {
    unsafe { 1024 << (*ext2::SUPERBLOCK).lblock_size }
}
