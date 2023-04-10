use crate::memory;
use crate::random;
use crate::screen;
use crate::filesystem;

#[repr(packed)]
#[derive(Clone, Copy)]
struct Process {
    pid: u16,
}

extern "C" {
    fn goto_user_space();
}

static mut PROCESSES: *mut Process = 0x0 as *mut Process;
const USER_SPACE: *mut u8 = 0x400000 as *mut u8;

pub fn initialize() {
    unsafe {
        PROCESSES = memory::kmalloc(2 * 256) as *mut Process;
    }
}

pub fn start(binary: &str) {
    let pid = random::u16();

    let fd = filesystem::open(binary, 0);
    if fd == 0 {
        panic!("binary not found!");
    }
    let size = filesystem::size(fd);
    if size == 0 {
        panic!("Error checking file size!");
    }
    let mut nblocks = (size / (1024 * 1024)) as u16;
    if size > (nblocks as u32) * 1024 * 1024 {
        nblocks += 1;
    }

    filesystem::read(fd, USER_SPACE, nblocks);

    unsafe { goto_user_space(); }
}

#[no_mangle]
pub extern "C" fn exit() {
    screen::print("Program exited!", screen::WHITE);
}

pub fn syscall(number: u16, r1: u32, r2: u32) -> i32 {
    if  number == 0x0 {
        exit();
        0
    } else {
        screen::print("Invalid process systemcall called!", screen::RED);
        -1
    }
}
