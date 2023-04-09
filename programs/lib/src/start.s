.global start

.extern stack_top
.extern main

start:
    ldr sp, =stack_top
    bl main
    mov r0, #0x00
    svc 0x0
    b   .
