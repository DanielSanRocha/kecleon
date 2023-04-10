use crate::memory;

#[derive(Copy, Clone)]
struct Schedule {
    flags: u8,
    last_executed: u32,
    handler: fn(timedelta: u32) -> (),
    interval: u32,
}

static mut SCHEDULES: *mut Schedule = 0 as *mut Schedule;

const SYSTEM_TIMER_REGISTER: *mut u32 = 0x3F003000 as *mut u32;

pub fn initialize() {
    unsafe {
        SCHEDULES = memory::kmalloc(72 * 64) as *mut Schedule;

        for i in 0..=63 {
            (*SCHEDULES.offset(i)).flags = 0;
            (*SCHEDULES.offset(i)).last_executed = 0;
            (*SCHEDULES.offset(i)).interval = 0;
        }

        let t0 = memory::inq(SYSTEM_TIMER_REGISTER, 1);
        memory::outq(SYSTEM_TIMER_REGISTER, t0 + 5000, 4);
    }
}

pub fn schedule(handler: fn(deltatime: u32) -> (), interval: u32) {
    unsafe {
        let mut flag = false;

        for i in 0..=63 {
            if (*SCHEDULES.offset(i)).flags != 0 {
                continue;
            } else {
                flag = true;
                (*SCHEDULES.offset(i)).flags = 1;
                (*SCHEDULES.offset(i)).handler = handler;
                (*SCHEDULES.offset(i)).interval = interval;
                (*SCHEDULES.offset(i)).last_executed = current();
                break;
            }
        }

        if !flag {
            panic!("Scheduler is full!")
        }
    }
}

pub fn sleep(time: u32) {
    let t0 = memory::inq(SYSTEM_TIMER_REGISTER, 1);

    loop {
        if memory::inq(SYSTEM_TIMER_REGISTER, 1) - t0 > time {
            break;
        }
    }
}

pub fn current() -> u32 {
    memory::inq(SYSTEM_TIMER_REGISTER, 1)
}

pub fn handler() {
    memory::outq(SYSTEM_TIMER_REGISTER, 2, 0);

    let t0 = memory::inq(SYSTEM_TIMER_REGISTER, 1);
    memory::outq(SYSTEM_TIMER_REGISTER, t0 + 5000, 4);

    unsafe {
        for i in 0..=63 {
            if (*SCHEDULES.offset(i)).flags == 0 {
                continue;
            } else {
                let schedule = *SCHEDULES.offset(i);

                if schedule.last_executed + schedule.interval < t0 {
                    let deltatime = t0 - schedule.last_executed;
                    (*SCHEDULES.offset(i)).last_executed = t0;
                    (schedule.handler)(deltatime);
                }
            }
        }
    }
}
