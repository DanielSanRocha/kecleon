pub trait Driver {
    fn initialize(&self);
    fn readblock(&self, lba: u32, buffer: *mut u8, num: u32);
}

pub struct EmptyDriver {}
impl Driver for EmptyDriver {
    fn initialize(&self) {}
    fn readblock(&self, lba: u32, buffer: *mut u8, num: u32) {}
}

pub const EMPTYDRIVER: *const EmptyDriver = &EmptyDriver {};
