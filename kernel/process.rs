use crate::memory;

struct ProcessEntry {
    pid: u16,
    flags: u8,
    always_zero: u8,
}

static mut PROCESS_VEC: *mut ProcessEntry = 0x0 as *mut ProcessEntry;

pub fn initialize() {
    unsafe {
        PROCESS_VEC = memory::kmalloc(32 * 256) as *mut ProcessEntry;

        for i in 0..256 {
            let proc = PROCESS_VEC.offset(i);
            (*proc).always_zero = 0;
            (*proc).flags = 0;
            (*proc).pid = 0;
        }
    }
}
