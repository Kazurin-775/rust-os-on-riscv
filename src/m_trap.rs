use crate::println;

const ECALL_FROM_S: isize = 9;

#[no_mangle]
extern "C" fn handle_m_trap(arg0: usize) {
    let mcause: isize;
    unsafe {
        asm!("csrr {0}, mcause", out(reg) mcause);
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
    let mcause: isize;
    let mepc: usize;
    unsafe {
        asm!("csrr {0}, mcause", out(reg) mcause);
        asm!("csrr {0}, mepc", out(reg) mepc);
    }
    println!(
        "Unknown machine trap #{} occurred with EPC = {:#X}.",
        mcause, mepc
    );
    println!("Halting the CPU.");
}

global_asm!(include_str!("asm/m_trap.S"));
