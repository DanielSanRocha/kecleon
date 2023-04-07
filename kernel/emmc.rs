use crate::memory;
use crate::timer;

struct Regs {
    arg2: u32,
    block_size_count: u32,
    arg1: u32,
    cmd_xfer_mode: u32,
    response: [u32; 4],
    data: u32,
    status: u32,
    control: [u32; 2],
    int_flags: u32,
    int_mask: u32,
    int_enable: u32,
    control2: u32,
    cap1: u32,
    cap2: u32,
    res0: [u32; 2],
    force_int: u32,
    res1: [u32; 7],
}

const EMMC: *mut Regs = 0x3F300000 as *mut Regs;

pub fn initialize() {
    reset();
}

fn reset() {
    unsafe {
        (*EMMC).control[1] = 1 << 24;
        (*EMMC).int_enable = 0;
        (*EMMC).int_flags = 0xFFFFFFFF;
        (*EMMC).int_mask = 0xFFFFFFFF;
    }
}

fn wait_reg_mask(reg: *const u32, mask: u32, set: bool) {
    loop {
        unsafe {
            if if (*reg & mask) != 0 { set } else { !set } {
                break;
            }

            timer::sleep(1000);
        }
    }
}
