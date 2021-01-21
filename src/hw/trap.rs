use core::panic;

global_asm!(include_str!("../asm/trap.S"));

#[no_mangle]
extern "C" fn unknown_s_trap() {
    let mut scause: isize;
    unsafe {
        asm!("csrr {0}, scause", out(reg) scause);
    }
    if scause < 0 {
        // this is an interrupt
        scause &= 0x7FF;
        panic!("unknown interrupt: {}", scause);
    } else {
        // this is an exception
        panic!("unknown exception: {}", scause);
    }
}
