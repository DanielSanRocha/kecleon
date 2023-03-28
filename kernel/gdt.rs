extern "C" {
    fn gdt_install();
}

pub fn initialize() {
    unsafe { gdt_install() }
}
