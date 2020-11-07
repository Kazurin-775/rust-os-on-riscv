use crate::println;

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
