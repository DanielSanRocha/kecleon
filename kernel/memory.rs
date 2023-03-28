pub fn memcpy(source: *mut u8, dest: *mut u8, size: u16) {
    for i in 0..size - 1 {
        unsafe {
            *source.offset(i as isize) = *dest.offset(i as isize);
        }
    }
}

pub fn outb(source: *mut u8, c: u8, offset: isize) {
    unsafe {
        *source.offset(offset) = c;
    }
}

pub fn inb(source: *mut u8, offset: isize) -> u8 {
    unsafe { *source.offset(offset) }
}
