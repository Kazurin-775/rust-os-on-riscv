    .text
    .global m_trap

m_trap:
    # reload the stack pointer in case it's corrupted
    la      sp, _stack_end

    # print an error message
    jal     unk_m_trap

    # halt the machine
1:
    wfi
    j       1b
