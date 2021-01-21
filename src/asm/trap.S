    .text
    .global s_trap

s_trap:
    # load an emergency sp
    la      sp, _stack_end

    # treat everything as an unknown S trap
    j       unknown_s_trap
