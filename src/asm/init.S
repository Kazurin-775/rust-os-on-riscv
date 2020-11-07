    .section .text.init
    .global _start

    .set MSTATUS_MPP, 11

_start:
    # load M-mode trap vector
    la      t0, m_trap
    csrw    mtvec, t0

    # set MPP to S-mode
    li      t0, (0b01 << MSTATUS_MPP)
    csrw    mstatus, t0

    # jump to S-mode code
    la      t0, s_mode_start
    csrw    mepc, t0
    mret

s_mode_start:
    # initializate registers
    la      sp, _stack_end
    la      gp, _global_pointer

    # clear .bss segment
    la      t0, _bss_start
    la      t1, _bss_end
    bge     t0, t1, 2f
1:
    sd      zero, (t0)
    addi    t0, t0, 8
    blt     t0, t1, 1b
2:

    # call kmain
    jal     kmain

    # halt the machine if returned from kmain
1:
    wfi
    j       1b
