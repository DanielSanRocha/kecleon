.extern main
.global start

start:
    cps #0x12
    ldr sp, =stack_irq_top

    cps #0x13
    ldr sp, =stack_top

    LDR sp, =stack_top
    BL main
    B .

