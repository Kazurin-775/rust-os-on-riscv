use core::panic;

mod sifive {
    const SIFIVE_TEST: *mut u32 = 0x100000 as _;
    const FINISHER_PASS: u32 = 0x5555;

    pub fn shutdown() {
        unsafe {
            SIFIVE_TEST.write_volatile(FINISHER_PASS);
        }
    }
}

pub fn shutdown() -> ! {
    super::sbi::system_reset(super::sbi::RST_SHUTDOWN, super::sbi::RST_NO_REASON);
    // if SBI fails to shut down the machine, try the SiFive method
    sifive::shutdown();
    panic!("cannot shut down the machine");
}
