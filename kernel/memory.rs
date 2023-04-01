pub fn outq(reg: *mut u32, val: u32, offset: isize) {
    unsafe { *reg.offset(offset) = val; }
}

pub fn inq(reg: *mut u32, offset: isize) -> u32 {
    unsafe { *reg.offset(offset) }
}
