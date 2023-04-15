use crate::memory;
use crate::timer;
use crate::uart;
use crate::storage::driver::Driver;

pub struct EMMC {
    pub addr: *mut u32,
}

impl EMMC {
    fn send_command(&self, cmd :u32, argument: u32, resp: u32) -> i32 {
        let mut wcmd = cmd & 0x3F;
        if wcmd != cmd {
            panic!("Wrong command to self.addr!");
        }

        memory::outq(self.addr, argument, 2);

        wcmd = wcmd | (resp << 6);
        wcmd = wcmd | 0x400;

        memory::outq(self.addr, wcmd, 3);

        timer::sleep(100);
        return 0;
    }
}

impl Driver for EMMC {
    fn initialize(&self) {
        unsafe {
            uart::print("\tInitializing SD Card...\n");
            memory::outq(self.addr, 0xBF, 0);
            timer::sleep(1000);
            memory::outq(self.addr, 0xC6, 1);
            timer::sleep(1000);

            uart::print("\tEntering IDLE state...\n");
            self.send_command(0, 0, 0);
            uart::print("\tEntering Identification state...\n");
            self.send_command(1, 0, 0);
            timer::sleep(500);
            self.send_command(2, 0, 2);

            self.send_command(3, 0, 0);
            uart::print("\tEntering Register state...\n");
            self.send_command(7, 0x4567, 0);
            uart::print("\tCard is Ready!\n");

            uart::print_int(memory::inq(self.addr, 5));
            uart::print("\n");
            uart::print_int(memory::inq(self.addr, 6));
            uart::print("\n");
            uart::print_int(memory::inq(self.addr, 7));
            uart::print("\n");
            uart::print_int(memory::inq(self.addr, 8));
            uart::print("\n");
        }
    }

    fn readblock(&self, lba: u32, buffer: *mut u8, num: u32) {
        let mut resp: [u32; 4] = [0,0,0,0];
        let mut ubuffer = buffer as *mut u32;

        self.send_command(16, 512, 2);
        timer::sleep(100);
        self.send_command(17, lba, 18);
        timer::sleep(500);

        let x = memory::inq(self.addr,18);
        uart::print_int(x);

        timer::sleep(200);

        unsafe {
            for i in 0..128 {
                let c = memory::inq(self.addr, 0x20);
                *ubuffer.offset(i) = c;
                if c != 0 {
                    panic!("Hummm...");
                }
            }
        }

        unsafe {
            uart::print_int(*ubuffer.offset(100));
        }
    }
}