    .section .text.init
    .global _start

_start:
    # initializate sp
    la      sp, _stack_end

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
