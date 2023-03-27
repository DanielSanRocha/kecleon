bits 32

section .text
        align 4
        dd 0x1BADB002
        dd 0x00
        dd - (0x1BADB002 + 0x00)

global start
extern main
extern load_gdt

start:
  cli
  mov esp, stack_space
  call load_gdt
  call main
  hlt

section .bss
resb 32768
stack_space:
