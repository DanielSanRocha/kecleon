.global start

.extern stack_top
.extern main

start:
    ldr sp, =stack_top
    bl main
    b  .
