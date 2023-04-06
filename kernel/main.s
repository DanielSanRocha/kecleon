.extern main
.global start

start:
    @ mov r0,#0x10000
    @ MCR p15, 4, r0, c12, c0, 0

    LDR sp, =stack_top
    BL main
    B .

