ENTRY(start)
MEMORY {
    ram : ORIGIN = 0x400000, LENGTH = 0x400000
}
SECTIONS {
    .text : { *(.text*) } > ram
    .data : { *(.data) } > ram
    .bss : { *(.bss*) } > ram
     . = . + 0x4000;
    stack_top = .;
}