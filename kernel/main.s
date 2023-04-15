.extern main
.global start

start:
    cps #0x12
    ldr sp, =stack_irq_top

    cps #0x13
    ldr sp, =stack_top

    mcr p15, 0, r0, c8, c7, 0
    mcr p15, 0, r0, c7, c5, 1
    @ dsb

    mcr p15,0,r0,c2,c0,0

    mov r0,#0 // Machine Code
    BL main
    B .

.extern get_application_state
.global goto_user_space
goto_user_space:
    movw r2, #0x150
    movt r2, #0x6000
    msr spsr,r2

    bl get_application_state
    add r0,#4
    ldmia r0!,{r1,r2,r3,r4,r5,r6,r7,r8,r9,r10,r11,r12,lr}
    push {r1}
    mov r1,r0
    ldmia r1!,{r0}
    push {r0}

    ldmia r1!,{r0}
    msr sp_usr,r0
    ldmia r1!,{r0}
    msr lr_usr,r0

    pop {r0}
    pop {r1}
    eret

.global get_cpsr
get_cpsr:
    mrs r0, cpsr
    bx  lr
