.section .text.entry
.global _start

_start:
    la sp, __boot_stack_top

    la t0, __bss_start
    la t1, __bss_end
    
.Lbss_loop:
    beq t0, t1, .Lmain
    sd zero, 0(t0)
    addi t0, t0, 8
    bltu t0, t1, .Lbss_loop

.Lmain:
    call kmain

.Lspin:
    wfi
    j .Lspin

