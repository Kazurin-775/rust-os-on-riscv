use core::panic;

mod sifive {
    const SIFIVE_TEST: *mut u32 = 0x100000 as _;
    const FINISHER_FAIL: u32 = 0x3333;
    const FINISHER_PASS: u32 = 0x5555;

    pub fn shutdown() {
        unsafe {
            SIFIVE_TEST.write_volatile(FINISHER_PASS);
        }
    }

    pub fn emergency_shutdown(code: u32) {
        unsafe {
            SIFIVE_TEST.write_volatile(FINISHER_FAIL | (code << 16));
        }
    }
}

pub fn shutdown() -> ! {
    super::sbi::system_reset(super::sbi::RST_SHUTDOWN, super::sbi::RST_NO_REASON);
    // if SBI fails to shut down the machine, try the SiFive method
    sifive::shutdown();
    panic!("cannot shut down the machine");
}

pub fn emergency_shutdown() -> ! {
    super::sbi::system_reset(super::sbi::RST_SHUTDOWN, super::sbi::RST_SYSTEM_FAILURE);
    // if SBI fails to shut down the machine, try the SiFive method
    sifive::emergency_shutdown(1);
    crate::println!("Fatal: cannot shut down the machine. Halting the CPU.");
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}
