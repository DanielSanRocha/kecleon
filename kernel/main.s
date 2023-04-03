.extern main
.global start
start:
    LDR sp, =stack_top
    BL main
    B .

.global get_el
get_el:
    BX lr
