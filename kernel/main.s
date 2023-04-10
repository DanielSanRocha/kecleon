#include "mmu.h"

.extern main
.global start

start:
    cps #0x12
    ldr sp, =stack_irq_top

    cps #0x13
    ldr sp, =stack_top

    mcr p15, 0, r0, c8, c7, 0
    mcr p15, 0, r0, c7, c5, 1
    dsb

    mcr p15,0,r0,c2,c0,0

    BL main
    B .

.global goto_user_space
goto_user_space:
    movw r0, #0x150
    movt r0, #0x6000
    msr spsr,r0
    mov r0,#0x400000
    mov lr,r0
    @ b 0x400000
    eret

.global get_cpsr
get_cpsr:
    mrs r0, cpsr
    bx  lr
