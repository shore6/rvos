OUTPUT_ARCH(riscv)
ENTRY(_start)

SECTIONS
{
    . = 0x80200000;

    .text : {
        __text_start = .;
        *(.text.entry)
        *(.text .text.*)
        __text_end = .;
    }

    . = ALIGN(4K);

    .rodata : {
        __rodata_start = .;
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        __rodata_end = .;
    }

    . = ALIGN(4K);

    .data : {
        __data_start = .;
        *(.data .data.*)
        *(.sdata .sdata.*)
        __data_end = .;
    }

    . = ALIGN(4K);

    .bss : {
        __bss_start = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        . = ALIGN(8);       /* .bss の大きさを 8byte の倍数にする */
        __bss_end = .;
    }

    . = ALIGN(4K);
    __boot_stack_bottom = .;
    . += 64K;           /* ブートスタック 64 KiB */
    . = ALIGN(16);
    __boot_stack_top = .;

    . = ALIGN(4K);
    __kernel_end = .;
}
