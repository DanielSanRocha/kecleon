.global putc_syscall
putc_syscall:
    push {lr}
    mov r2,r1
    mov r1,r0
    mov r0,#0x11
    svc 0x1
    pop {lr}
    bx  lr
