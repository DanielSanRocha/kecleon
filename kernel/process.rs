use crate::memory;

struct Process {}

static mut PROCESSES: *mut Process = 0x0 as *mut Process;

pub fn initialize() {
    unsafe {
        PROCESSES = memory::malloc(128 * 128) as *mut Process;
    }
}
