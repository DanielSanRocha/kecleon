#[derive(Copy, Clone)]
struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

extern "C" {
    fn gdt_flush();
    static mut gdt: [GdtEntry; 3];
}

fn gdt_set_gate(num: usize, base: u32, limit: u32, access: u8, gran: u8) {
    unsafe {
        gdt[num].base_low = (base & 0xFFFF) as u16;
        gdt[num].base_middle = ((base >> 16) & 0xFF) as u8;
        gdt[num].base_high = ((base >> 24) & 0xFF) as u8;

        gdt[num].limit_low = (limit & 0xFFFF) as u16;
        gdt[num].granularity = ((limit >> 16) & 0x0F) as u8;

        gdt[num].granularity |= gran & 0xF0;
        gdt[num].access = access;
    }
}

pub fn initialize() {
    unsafe {
        gdt_set_gate(0, 0, 0, 0, 0);
        gdt_set_gate(1, 0, 0xFFFFFFFF, 0x9A, 0xCF);
        gdt_set_gate(2, 0, 0xFFFFFFFF, 0x92, 0xCF);

        gdt_flush();
    }
}
