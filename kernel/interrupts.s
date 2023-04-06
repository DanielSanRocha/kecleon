.global vector_table
.extern start
.extern stack_top

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
undefined_handler_ptr:  .word hang
swi_handler_ptr:        .word hang
prefetch_handler_ptr:   .word hang
data_handler_ptr:       .word hang
unused_handler_ptr:     .word hang
irq_handler_ptr:        .word irq
fiq_handler_ptr:        .word hang

.global move_vector_table
move_vector_table:
    push {r0-r9}

    mov r0,#0x10000
    mov r1,#0x0000
    ldmia r0!,{r2,r3,r4,r5,r6,r7,r8,r9}
    stmia r1!,{r2,r3,r4,r5,r6,r7,r8,r9}
    ldmia r0!,{r2,r3,r4,r5,r6,r7,r8,r9}
    stmia r1!,{r2,r3,r4,r5,r6,r7,r8,r9}

    @ cps #0x12 //change to IRQ mode
    @ ldr sp, =stack_irq_top

    @ cps #0x13
    @ ldr sp, =stack_top

    pop {r0-r9}
    bx  lr

.global enable_interrupts
enable_interrupts:
    cpsie i
    bx lr

hang:
    b .

.extern irq_handler
irq:
    push {r0,r1,r2,r3,r4,r5,r6,r7,r8,r9,r10,r11,r12,lr}
    bl irq_handler
    pop  {r0,r1,r2,r3,r4,r5,r6,r7,r8,r9,r10,r11,r12,lr}
    subs pc,lr,#4
    @ eret