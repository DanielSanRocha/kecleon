struct MemoryEntry {
    start: u32,
    end: u32,
    process: u16,
    active: u8,
    always_zero: u32,
    flags: u8,
}

const PAGE_SIZE: u32 = 4 * 1024 * 1024;
const HEAP_START: *mut u8 = 0x6000000 as *mut u8;
static mut CURRENT_OFFSET: u32 = 0;
static mut MEMORY_VEC: *mut MemoryEntry = HEAP_START as *mut MemoryEntry;

pub fn outb(source: *mut u8, c: u8, offset: isize) {
    unsafe {
        *source.offset(offset) = c;
        // c_outb(source as u32 + offset as u32, c)
    }
}

pub fn inb(source: *mut u8, offset: isize) -> u8 {
    unsafe {
        *source.offset(offset)
        // c_inb(source as u32 + offset as u32)
    }
}

pub fn memcpy(source: *mut u8, dest: *mut u8, bytes: isize) {
    unsafe {
        for i in 0..bytes {
            *dest.offset(i) = *source.offset(i);
        }
    }
}

pub fn initialize() {
    for i in 0..512 {
        unsafe {
            let mut mem = MEMORY_VEC.offset(i);
            (*mem).always_zero = 0;
            (*mem).active = 0;
            (*mem).start = 0;
            (*mem).end = 0;
            (*mem).process = 0;
            (*mem).flags = 0;
        }
    }

    unsafe {
        let first_page = MEMORY_VEC.offset(0);
        (*first_page).active = 1;
        (*first_page).start = HEAP_START as u32;
        (*first_page).end = HEAP_START as u32 + PAGE_SIZE;
        (*first_page).process = 0;
    }

    unsafe {
        CURRENT_OFFSET += 512 * 128;
    }
}

// fn malloc_page() -> *mut u8 {
//     for i in 0..512 {
//         unsafe {
//             let mem = MEMORY_VEC.offset(i);
//             if (*mem).active == 0 {
//                 (*mem).active = 1;
//                 (*mem).start = HEAP_START as u32 + PAGE_SIZE * i as u32;
//                 (*mem).end = HEAP_START as u32 + PAGE_SIZE * (i + 1) as u32;
//                 (*mem).flags = 0;
//                 (*mem).process = 0;

//                 return (*mem).start as *mut u8;
//             }
//         }
//     }

//     panic!("No more memory available, kernel panic!");
// }

pub fn kmalloc(size: u32) -> *mut u8 {
    unsafe {
        let tmp = HEAP_START.offset(CURRENT_OFFSET as isize);
        CURRENT_OFFSET += size;
        tmp
    }
}
