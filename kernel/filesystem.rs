use crate::ext2;
use crate::memory;
use crate::random;

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
        FILES = memory::malloc(12 * 256) as *mut File;

        for i in 0..=255 {
            (*FILES.offset(i)).id = 0;
            (*FILES.offset(i)).inode = 0;
            (*FILES.offset(i)).offset = 0;
            (*FILES.offset(i)).process = 0;
        }

        INODE_BUFFER = memory::malloc(128) as *mut ext2::Inode;
        BLOCK_BUFFER = memory::malloc(1024) as *mut u8;
    }
}

pub fn size(fd: u16) -> u32 {
    unsafe {
        for i in 0..=255 {
            if(*FILES.offset(i)).id == fd {
                let file = *FILES.offset(i);
                ext2::get_inode(file.inode, INODE_BUFFER);

                return (*INODE_BUFFER).lower_size
            }
        }

        return 0;
    }
}

pub fn read(fd: u16, buffer: *mut u8, nblocks: u16) -> u16 {
    unsafe {
        for i in 0..=255 {
            if (*FILES.offset(i)).id == fd {
                let file = *FILES.offset(i);
                ext2::get_inode(file.inode, INODE_BUFFER);

                if nblocks == 0 {
                    return 0;
                }

                if nblocks > 2 {
                    panic!("Files with size bigger than 2MB are not supported!");
                }

                ext2::read_inode(INODE_BUFFER, buffer, nblocks as u8);
            }
        }

        return 0;
    }
}

pub fn open(path: &str, process: u16) -> u16 {
    if path.len() == 0 {
        panic!("Trying to open file with empty name!");
    }

    if path.as_bytes()[0] != '/' as u8 {
        panic!("Path must start  with a backslash '/'!");
    }

    let inode_number = open_recursion(2, &path[1..path.len()], process);

    inode_number as u16
}

fn open_recursion(root: u32, path: &str, process: u16) -> u16 {
    unsafe {
        ext2::get_inode(root, INODE_BUFFER);
        ext2::read_inode(INODE_BUFFER, BLOCK_BUFFER, 1);

        if (*INODE_BUFFER).permission & 0x4000 == 0 {
            return 0;
        }

        let mut i = 0;
        loop {
            let inode = *(BLOCK_BUFFER.offset(i) as *mut u32);
            if inode == 0 {
                return 0;
            }

            let namesize = *BLOCK_BUFFER.offset(i + 6);
            let mut flag = 1 as u8;

            if path.len() >= namesize as usize {
                for j in 0..namesize {
                    let c = *BLOCK_BUFFER.offset(8 + j as isize + i as isize) as char;
                    if path.as_bytes()[j as usize] != c as u8 {
                        flag = 0;
                        break;
                    }
                }

                if flag == 1 {
                    if namesize == path.len() as u8 {
                        return create_fd(inode, process);
                    }

                    if path.as_bytes()[namesize as usize] == '/' as u8 {
                        let new_path = &path[(namesize as usize + 1)..path.len()];
                        return open_recursion(inode, new_path, process);
                    }
                }
            }

            let entrysize: u16 = *(BLOCK_BUFFER.offset(i) as *mut u16).offset(2);
            i += entrysize as isize;
        }
    }
}

fn create_fd(inode: u32, process: u16) -> u16 {
    for i in 0..=255 {
        unsafe {
            let file = *FILES.offset(i);
            if file.id == 0 {
                let id = random::u16();
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
