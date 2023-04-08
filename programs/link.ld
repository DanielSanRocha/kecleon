MEMORY {
    ram : ORIGIN = 0x400000, LENGTH = 0x100000
}

SECTIONS {
    .text : { *(.text*) } > ram
    .bss : { *(.bss*) } > ram
     . = . + 0x4000;
    stack_top = .;
}
