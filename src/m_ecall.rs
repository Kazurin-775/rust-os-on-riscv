use crate::println;

pub fn handle_ecall(func_no: usize) {
    match func_no {
        1 => {
            println!("ecall #1");
        }
        _ => panic!("unknown ecall function number"),
    }

    // let mepc += 4
    unsafe {
        #[rustfmt::skip]
        asm!(
            "csrrw t0, mepc, t0",
            "addi t0, t0, 4",
            "csrrw t0, mepc, t0",
        );
    }
}
