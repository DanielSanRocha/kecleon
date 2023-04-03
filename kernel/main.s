.extern main
.global start

start:
    mrc	p15, 0, r4, c1, c0, 0	@ System Control Register
	orr	r4, r4, #0x00000004
	orr	r4, r4, #0x00001000
	mcr	p15, 0, r4, c1, c0, 0

    mrc	p15, 0, r4, c1, c0, 1	@ Read Auxiliary Control Register
	orr	r4, r4, #0x00000040
	mcr	p15, 0, r4, c1, c0, 1

    LDR sp, =stack_top
    BL main
    B .
