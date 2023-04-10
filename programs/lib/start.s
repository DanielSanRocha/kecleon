.global start
.extern main
.extern stack_top
start:
    ldr sp,=stack_top
    bl main
    mov r0,#0
    svc #0
    b   .
