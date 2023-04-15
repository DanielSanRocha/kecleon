use crate::memory;
use crate::screen;
use crate::storage;

#[repr(packed, C)]
#[derive(Clone, Copy)]
pub struct Process {
    pid: u16,
    parent: u16,
    r1: u32,
    r2: u32,
    r3: u32,
    r4: u32,
    r5: u32,
    r6: u32,
    r7: u32,
    r8: u32,
    r9: u32,
    r10: u32,
    r11: u32,
    r12: u32,
    pc: u32,
    r0: u32,
    sp: u32,
    lr: u32,
}

static mut PROCESSES: *mut Process = 0x0 as *mut Process;
const USER_SPACE: *mut u8 = 0x400000 as *mut u8;
static mut CURRENT_PROCESS_PID: u16 = 0x0 as u16;
static mut CURRENT_PROCESS_INDEX: u16 = 0 as u16;
static mut FOCUS_PROCESS_PID: u16 = 0x0 as u16;
static mut FOCUS_PROCESS_INDEX: u16 = 0 as u16;

pub fn initialize() {
    unsafe {
        PROCESSES = memory::kmalloc(68 * 256) as *mut Process;

        for i in 0..=255 {
            (*PROCESSES.offset(i)).pid = 0;
            (*PROCESSES.offset(i)).parent = 0;
            (*PROCESSES.offset(i)).pc = 0;
        }
    }
}

pub fn new_pid() -> u16 {
    // TODO optimize this algorithm
    let mut npid = 1 as u16;
    let mut i;

    unsafe {
        loop {
            i = 0;
            loop {
                let pid = (*PROCESSES.offset(i)).pid;
                if pid == npid {
                    npid += 1;
                    break;
                }
                i += 1;
                if i == 256 {
                    return npid;
                }
            }
        }
    }
}

pub fn schedule(_deltatime: u32) {
    unsafe {
        for i in (CURRENT_PROCESS_INDEX + 1)..=255 {
            let proc = *PROCESSES.offset(i as isize);
            if proc.pid == 0 {
                continue;
            }
            CURRENT_PROCESS_INDEX = i;
            CURRENT_PROCESS_PID = proc.pid;
            memory::switch(proc.pid);
            return;
        }

        for i in 0..CURRENT_PROCESS_INDEX {
            let proc = *PROCESSES.offset(i as isize);
            if proc.pid == 0 {
                continue;
            }
            CURRENT_PROCESS_INDEX = i;
            CURRENT_PROCESS_PID = proc.pid;
            memory::switch(proc.pid);
            return;
        }
    }
}

pub fn start(binary: &str, arguments: &str, parent: u16) -> u16 {
    unsafe {
        let pid = new_pid();

        let fd = storage::filesystem::open(binary, pid);
        if fd == 0 {
            panic!("binary not found!");
        }

        let size = storage::filesystem::size(fd);
        if size == 0 {
            panic!("Error checking file size!");
        }
        let mut nblocks = size / storage::filesystem::block_size();
        if size > (nblocks as u32) * storage::filesystem::block_size() {
            nblocks += 1;
        }

        let mut npages = size / (1024 * 1024);
        if size > npages * (1024 * 1024) {
            npages += 1;
        }

        for i in 0..=255 {
            let proc = *PROCESSES.offset(i);
            if proc.pid == 0 {
                (*PROCESSES.offset(i)).pid = pid;
                (*PROCESSES.offset(i)).parent = parent;
                (*PROCESSES.offset(i)).pc = 0x400000;
                (*PROCESSES.offset(i)).sp = 0x0;
                (*PROCESSES.offset(i)).lr = 0x0;

                for _ in 1..=npages {
                    memory::alloc_page(pid);
                }

                //For arguments and Heap
                let heap = memory::alloc_page(pid);
                (*PROCESSES.offset(i)).r0 = heap;

                memory::switch(pid);
                storage::filesystem::read(fd, USER_SPACE, nblocks);

                let mut i = 0;
                let ptr = heap as *mut u8;
                for c in arguments.chars() {
                    *ptr.offset(i) = c as u8;
                    i += 1;
                }
                *ptr.offset(i) = 0;

                if CURRENT_PROCESS_PID != 0 {
                    memory::switch(CURRENT_PROCESS_PID);
                }

                return pid;
            }
        }

        panic!("No more room for allocating processes!");
    }
}

pub fn set_current(pid: u16) {
    unsafe {
        for i in 0..=255 {
            let proc = *PROCESSES.offset(i);
            if proc.pid == pid {
                CURRENT_PROCESS_INDEX = i as u16;
                CURRENT_PROCESS_PID = proc.pid;
                memory::switch(pid);
                return;
            }
        }

        panic!("Process not found!");
    }
}

pub fn focus(pid: u16) {
    unsafe {
        for i in 0..=255 {
            let proc = *PROCESSES.offset(i);
            if proc.pid == pid {
                FOCUS_PROCESS_INDEX = i as u16;
                FOCUS_PROCESS_PID = proc.pid;
                return;
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn get_application_state() -> *mut Process {
    unsafe {
        return PROCESSES.offset(CURRENT_PROCESS_INDEX as isize);
    }
}

#[no_mangle]
pub extern "C" fn exit(code: i32) {
    if code < 0 {
        screen::print("\nProgram exited!", screen::RED);
    }
    unsafe {
        (*PROCESSES.offset(CURRENT_PROCESS_INDEX as isize)).pid = 0;
        memory::free_pages(CURRENT_PROCESS_PID);
    }
}

pub fn syscall(number: u16, r1: u32, r2: u32) -> i32 {
    if number == 0x0 {
        exit(r1 as i32);
        0
    } else {
        screen::print("Invalid process systemcall called!", screen::RED);
        -1
    }
}
