use crate::screen;
use crate::uart;

const HEAP_BEGIN: *mut u32 = 0x100000 as *mut u32;
static mut MMUTABLEBASE: *mut u32 = 0x0 as *mut u32;
static mut OFFSET: u32 = 0;

extern "C" {
    fn start_mmu(MMUTABLEBASE: *mut u32);
    fn invalidate_tlbs();
}

pub fn outq(reg: *mut u32, val: u32, offset: isize) {
    unsafe {
        *reg.offset(offset) = val;
    }
}

pub fn inq(reg: *mut u32, offset: isize) -> u32 {
    unsafe { *reg.offset(offset) }
}

pub fn invalidate() {
    unsafe { invalidate_tlbs() }
}

pub fn initialize() {
    unsafe {
        MMUTABLEBASE = malloc(4096 * 4);

        for i in 0..=4095 {
            section(i * 1024 * 1024, i * 1024 * 1024, 0x0000);
        }

        start_mmu(MMUTABLEBASE);
        invalidate();
    }
}

pub fn section(vadd: u32, padd: u32, flags: u32) {
    unsafe {
        let mut ra = vadd >> 20;
        let rb = (MMUTABLEBASE as u32) | (ra << 2);
        ra = padd >> 20;
        let rc = (ra << 20) | flags | 2;

        *(rb as *mut u32) = rc;
    }
}

#[no_mangle]
pub extern "C" fn malloc(size: isize) -> *mut u32 {
    uart::print("\n\t\tAllocated ");
    uart::print_int(size as u32);
    uart::print(" bytes!");

    unsafe {
        let tmp = HEAP_BEGIN as u32 + OFFSET;
        OFFSET += size as u32;

        tmp as *mut u32
    }
}
