global gdt_flush
global gdt

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

gdt:
resb 1024
gdt_end:
gp:
    dw gdt_end - gdt - 1
    dd gdt
