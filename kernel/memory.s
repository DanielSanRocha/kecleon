.global start_mmu
start_mmu:
    mov r1,#0
    // invalidate caches
    mcr p15,0,r1,c7,c7,0
    // invalidate TLB entries
    mcr p15,0,r1,c8,c7,0
    // data synchronisation barrier
    mcr p15,0,r1,c7,c10,4

    // set all domains to 0b11
    ldr r1, =0xffffffff
    mcr p15,0,r1,c3,c0,0

    // set the translation table base address (remember to align 16 KiB!)
    mcr p15,0,r0,c2,c0,0

    // set the bits mentioned above
    ldr r1, =0x00401805
    mrc p15,0,r2,c1,c0,0
    orr r2,r2,r1
    mcr p15,0,r2,c1,c0,0

    mov pc, lr

.globl invalidate_tlbs
invalidate_tlbs:
    mov r2,#0
    mcr p15,0,r2,c8,c7,0  ;@ invalidate tlb
    mcr p15,0,r2,c7,c10,4 ;@ DSB ??
    bx lr

