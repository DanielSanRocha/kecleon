struct GdtEntry {
    unsigned short limit_low;
    unsigned short base_low;
    unsigned char base_middle;
    unsigned char access;
    unsigned char granularity;
    unsigned char base_high;
} __attribute__((packed)); typedef struct GdtEntry GdtEntry;

struct GdtPtr
{
    unsigned short limit;
    unsigned int base;
} __attribute__((packed)); typedef struct GdtPtr GdtPtr;

extern void gdt_flush();
extern GdtPtr gp;
GdtEntry gdt_table[5];

void gdt_set_gate(unsigned short num, unsigned short base, unsigned long limit, unsigned char access, unsigned char gran) {
    gdt_table[num].base_low = (base & 0xFFFF);
    gdt_table[num].base_middle = (base >> 16) & 0xFF;
    gdt_table[num].base_high = (base >> 24) & 0xFF;

    gdt_table[num].limit_low = (limit & 0xFFFF);
    gdt_table[num].granularity = ((limit >> 16) & 0x0F);

    gdt_table[num].granularity |= (gran & 0xF0);
    gdt_table[num].access = access;
}

void gdt_install() {
    gp.limit = (sizeof(GdtEntry) * 3) - 1;
    gp.base = (unsigned) &gdt_table;

    gdt_set_gate(0, 0, 0, 0, 0);
    gdt_set_gate(1, 0, 0xFFFFFFFF, 0x9A, 0xCF);
    gdt_set_gate(2, 0, 0xFFFFFFFF, 0x92, 0xCF);


    gdt_flush();
}
