.global start

.extern stack_top
.extern main

start:
    ldr sp, =stack_top
    bl main
    svc 0x0
    b   .
