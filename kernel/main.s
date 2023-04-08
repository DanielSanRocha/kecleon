#include "mmu.h"

.extern main
.global start

start:
    cps #0x12
    ldr sp, =stack_irq_top

    cps #0x13
    ldr sp, =stack_top

    LDR sp, =stack_top

    mcr p15, 0, r0, c8, c7, 0  @ TLBIALL: invalidate all TLBs
    mcr p15, 0, r0, c7, c5, 1  @ ICIALLU: invalidate instruction cache
    dsb                        @ ensure invalidations have completed

    mcr p15,0,r0,c2,c0,0

    BL main
    B .

.global goto_user_space
goto_user_space:
    b 0x400000
    b .