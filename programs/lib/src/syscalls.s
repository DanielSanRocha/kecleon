.global putc_syscall
putc_syscall:
    svc 0x1
    bx  lr

