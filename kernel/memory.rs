use crate::uart;

const HEAP_BEGIN: *mut u32 = 0xC00000 as *mut u32;
static mut OFFSET: u32 = 0;

pub fn outq(reg: *mut u32, val: u32, offset: isize) {
    unsafe {
        *reg.offset(offset) = val;
    }
}

pub fn inq(reg: *mut u32, offset: isize) -> u32 {
    unsafe { *reg.offset(offset) }
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
