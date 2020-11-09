    .text
    .global m_trap

m_trap:
    # detect double faults
    csrrw   t0, mscratch, t0
    bnez    t0, double_fault
    csrrwi  t0, mscratch, 1

    # include files from `src/asm/`. I wonder why this works, but including
    # from `./` doesn't...
    .include "src/asm/save_context.S"

    # jump to Rust code
    jal     handle_m_trap

    .include "src/asm/restore_context.S"

    csrw    mscratch, zero
    mret

double_fault:
    # reload the stack pointer in case it's corrupted
    la      sp, _stack_end

    # print an error message
    jal     unk_m_trap

    # halt the machine
1:
    wfi
    j       1b
