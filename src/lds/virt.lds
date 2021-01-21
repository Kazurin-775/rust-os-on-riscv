OUTPUT_ARCH("riscv")
ENTRY(_start)

SECTIONS {
    . = 0x80200000;
    .text : {
        *(.text.init) *(.text .text.*)
    }
    .rodata : {
        *(.rodata .rodata.*)
    }
    .data : {
        *(.sdata .sdata.*) *(.data .data.*)
    }
    .bss : {
        _bss_start = .;
        *(.sbss .sbss.*) *(.bss .bss.*)
        . = ALIGN(8);
        _bss_end = .;
    }
    _stack_start = .;
    . += 0x80000;
    _stack_end = .;
}
