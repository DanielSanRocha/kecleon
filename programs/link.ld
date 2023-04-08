ENTRY(start)

SECTIONS {
    . = 0x40000;
    .text : { *(.text*) }
    .data : { *(.data) }
    .bss : { *(.bss*) }
     . = . + 0x4000;
    stack_top = .;
}
