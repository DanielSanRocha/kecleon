ENTRY(start)
SECTIONS
{
  _kernel = 0x0000;
 . = 0x10000;
 .text : { *(.text) }
 .data : { *(.data) }
 .bss : { *(.bss COMMON) }
 . = ALIGN(8);
 . = . + 0x4000; /* 16kB of stack memory */
 stack_top = .;
}