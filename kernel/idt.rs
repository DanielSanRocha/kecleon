extern "C" {
    fn idt_install();
}

pub fn initialize() {
    unsafe { idt_install() }
}
