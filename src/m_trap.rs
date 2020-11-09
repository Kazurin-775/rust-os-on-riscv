use crate::println;

const ECALL_FROM_S: isize = 9;

#[no_mangle]
extern "C" fn handle_m_trap(arg0: usize) {
    let mut mcause: isize;
    unsafe {
        asm!("csrr {0}, mcause", out(reg) mcause);
    }
    if mcause < 0 {
        mcause = -(mcause & 0x7FF);
    }

    match mcause {
        ECALL_FROM_S => crate::m_ecall::handle_ecall(arg0),
        _ => {
            unk_m_trap();
            loop {
                unsafe {
                    asm!("wfi");
                }
            }
        }
    }
}

#[no_mangle]
extern "C" fn unk_m_trap() {
    let mut mcause: isize;
    let mepc: usize;
    unsafe {
        // turn off interrupts to save power
        asm!("csrw mie, zero");

        asm!("csrr {0}, mcause", out(reg) mcause);
        asm!("csrr {0}, mepc", out(reg) mepc);
    }
    if mcause < 0 {
        mcause = -(mcause & 0x7FF);
    }
    println!(
        "Unknown machine trap #{} occurred with EPC = {:#X}.",
        mcause, mepc
    );
    println!("Halting the CPU.");
}

global_asm!(include_str!("asm/m_trap.S"));
