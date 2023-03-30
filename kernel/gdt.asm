global gdt_flush
global gp

gdt_flush:
    lgdt [gp]
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    jmp 0x08:gdt_flush_ret
gdt_flush_ret:
    ret

gp:
    dw 0
    dd 0

global hlt
hlt:
    hlt
    ret

global cli
cli:
    cli
    ret

global sti
sti:
    sti
    ret
