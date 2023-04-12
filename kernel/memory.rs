use crate::uart;

const HEAP_BEGIN: *mut u32 = 0x100000 as *mut u32;
static mut MMUTABLEBASE: *mut u32 = 0x0 as *mut u32;
static mut OFFSET: u32 = 0;

#[repr(packed)]
#[derive(Clone, Copy)]
struct Page {
    pid: u16,
    addr: u32,
    order: u16,
}

static mut PAGES: *mut Page = 0x0 as *mut Page;

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

pub fn memcopy(source: *mut u8, dest: *mut u8, size: isize) {
    for i in 0..size {
        unsafe {
            *dest.offset(i) = *source.offset(i);
        }
    }
}

pub fn memset(dest: *mut u8, value: u8, size: isize) {
    for i in 0..size {
        unsafe {
            *dest.offset(i) = value;
        }
    }
}

pub fn invalidate() {
    unsafe { invalidate_tlbs() }
}

pub fn initialize() {
    unsafe {
        MMUTABLEBASE = kmalloc(4096 * 4);

        section(0x0, 0x0, 0x0);
        section(0x100000, 0x100000, 0x0);
        section(0x200000, 0x200000, 0x0);
        section(0x300000, 0x300000, 0x0);

        // IO
        section(0x3F000000, 0x3F000000, 0x0);
        section(0x3F200000, 0x3F200000, 0x0);
        section(0x3F300000, 0x3F300000, 0x0);
        section(0x3F900000, 0x3F900000, 0x0);
        section(0x40000000, 0x40000000, 0x0);

        // Framebuffer
        section(0x3C100000, 0x3C100000, 0x0);
        section(0x3C200000, 0x3C200000, 0x0);
        section(0x3C300000, 0x3C300000, 0x0);
        section(0x3C400000, 0x3C400000, 0x0);
        section(0x3C500000, 0x3C500000, 0x0);
        section(0x3C600000, 0x3C600000, 0x0);
        section(0x3C700000, 0x3C700000, 0x0);

        start_mmu(MMUTABLEBASE);
        invalidate();

        PAGES = kmalloc(128 * 8) as *mut Page;

        for i in 0..=127 {
            (*PAGES.offset(i)).pid = 0;
            (*PAGES.offset(i)).addr = 0;
            (*PAGES.offset(i)).order = 0;
        }
    }
}

pub fn alloc_page(pid: u16) -> u32 {
    uart::print("Allocating page for pid ");
    uart::print_int(pid as u32);
    uart::print("\n");

    unsafe {
        let mut count = 0 as u16;

        for i in 0..=127 {
            if (*PAGES.offset(i)).pid == pid {
                count += 1;
            }
        }

        for i in 0..=127 {
            if (*PAGES.offset(i)).pid == 0 {
                (*PAGES.offset(i)).pid = pid;
                (*PAGES.offset(i)).addr = 0x400000 + 0x100000 * i as u32;
                (*PAGES.offset(i)).order = count;

                return 0x400000 + 0x100000 * count as u32;;
            }
        }

        panic!("No memory left on the device!");
    }
}

pub fn free_pages(pid: u16) {
    unsafe {
        for i in 0..=127 {
            if (*PAGES.offset(i)).pid == pid {
                uart::print("Freeing page of process ");
                uart::print_int(pid as u32);
                uart::print("\n");
                (*PAGES.offset(i)).pid = 0;
                (*PAGES.offset(i)).order = 0;
            }
        }
    }
}

pub fn switch(pid: u16) {
    unsafe {
        for i in 0..=127 {
            invalidate_section(0x400000 + 0x100000 * i as u32);
        }

        for i in 0..=127 {
            let page = *PAGES.offset(i);
            if page.pid == pid {
                section(0x400000 + 0x100000 * page.order as u32, page.addr, 0x0);
            }
        }
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

pub fn invalidate_section(vadd: u32) {
    unsafe {
        let ra = vadd >> 20;
        let rb = (MMUTABLEBASE as u32) | (ra << 2);
        *(rb as *mut u32) = 0x0;
    }
}

#[no_mangle]
pub extern "C" fn kmalloc(size: isize) -> *mut u32 {
    uart::print("\t\tAllocated ");
    uart::print_int(size as u32);
    uart::print(" bytes!\n");

    unsafe {
        let tmp = HEAP_BEGIN as u32 + OFFSET;
        OFFSET += size as u32;

        tmp as *mut u32
    }
}

pub extern "C" fn free(_ptr: *mut u32) {}
