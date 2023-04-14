.global vector_table
.extern start
.extern stack_top

.extern undefined_handler
.extern swi_handler
.extern prefetch_handler
.extern data_handler
.extern unused_handler
.extern fiq_handler
.extern irq_handler

vector_table:
    ldr pc,reset_handler_ptr
    ldr pc,undefined_handler_ptr
    ldr pc,swi_handler_ptr
    ldr pc,prefetch_handler_ptr
    ldr pc,data_handler_ptr
    ldr pc,unused_handler_ptr
    ldr pc,irq_handler_ptr
    ldr pc,fiq_handler_ptr

reset_handler_ptr:      .word start
undefined_handler_ptr:  .word undefined
swi_handler_ptr:        .word swi
prefetch_handler_ptr:   .word prefetch_handler
data_handler_ptr:       .word data_handler
unused_handler_ptr:     .word unused_handler
irq_handler_ptr:        .word irq
fiq_handler_ptr:        .word fiq_handler

.global move_vector_table
move_vector_table:
    push {r0-r9}
    cpsid i

    mov r0,#0x10000
    mov r1,#0x0000
    ldmia r0!,{r2,r3,r4,r5,r6,r7,r8,r9}
    stmia r1!,{r2,r3,r4,r5,r6,r7,r8,r9}
    ldmia r0!,{r2,r3,r4,r5,r6,r7,r8,r9}
    stmia r1!,{r2,r3,r4,r5,r6,r7,r8,r9}

    pop {r0-r9}
    bx  lr

.global enable_interrupts
enable_interrupts:
    cpsie i
    bx lr

.global disable_interrupts
disable_interrupts:
    cpsid i
    bx    lr

.global hang
hang:
    wfi
    b hang

.extern get_application_state
irq:
    cpsid i
    push {r0}
    push {r1,r2,r3,r4,r5,r6,r7,r8,r9,r10,r11,r12,lr}

    mrs r0, spsr
    and r0, r0, #3
    cmp r0,#0
    bne kernel_irq
user_irq:
    bl get_application_state
    add r0,#4
    pop {r1,r2,r3,r4,r5,r6,r7,r8,r9,r10,r11,r12,lr}
    stmia r0!,{r1,r2,r3,r4,r5,r6,r7,r8,r9,r10,r11,r12,lr}
    mov r1,r0
    pop {r0}
    stmia r1!,{r0}

    mrs r0,sp_usr
    stmia r1!,{r0}
    mrs r0,lr_usr
    stmia r1, {r0}

    bl irq_handler

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
kernel_irq:
    bl irq_handler
    pop  {r1,r2,r3,r4,r5,r6,r7,r8,r9,r10,r11,r12,lr}
    pop  {r0}
    eret

swi:
    cpsid i
    push {r1,r2,r3,r4,r5,r6,r7,r8,r9,r10,r11,r12,lr}
    mov r3,r2
    mov r2,r1
    mov r1,r0
    LDR r0,[lr,#-4]
    BIC r0,r0,#0xFF000000
    bl swi_handler
    pop  {r1,r2,r3,r4,r5,r6,r7,r8,r9,r10,r11,r12,lr}
    eret

.extern exit
undefined:
    cpsid i
    movw r0, #0x15b
    movt r0, #0x6000
    msr spsr,r0
    ldr r0,=undefined_supervisor
    mov lr,r0
    eret
undefined_supervisor:
    ldr sp, =stack_top
    mov r0,#-1
    bl exit
    b .
