#include "memory.h"
#include "keyboard.h"

extern void isr0();
extern void isr1();
extern void isr2();
extern void isr3();
extern void isr4();
extern void isr5();
extern void isr6();
extern void isr7();
extern void isr8();
extern void isr9();
extern void isr10();
extern void isr11();
extern void isr12();
extern void isr13();
extern void isr14();
extern void isr15();
extern void isr16();
extern void isr17();
extern void isr18();
extern void isr19();
extern void isr20();
extern void isr21();
extern void isr22();
extern void isr23();
extern void isr24();
extern void isr25();
extern void isr26();
extern void isr27();
extern void isr28();
extern void isr29();
extern void isr30();
extern void isr31();

extern void irq0();
extern void irq1();
extern void irq2();
extern void irq3();
extern void irq4();
extern void irq5();
extern void irq6();
extern void irq7();
extern void irq8();
extern void irq9();
extern void irq10();
extern void irq11();
extern void irq12();
extern void irq13();
extern void irq14();
extern void irq15();

struct idt_entry {
    unsigned short base_lo;
    unsigned short sel;
    unsigned char always0;
    unsigned char flags;
    unsigned short base_hi;
} __attribute__((packed));

struct idt_ptr {
    unsigned short limit;
    unsigned int base;
} __attribute__((packed));

struct regs {
    unsigned int gs, fs, es, ds;
    unsigned int edi, esi, ebp, esp, ebx, edx, ecx, eax;
    unsigned int int_no, err_code;
    unsigned int eip, cs, eflags, useresp, ss;
};

struct idt_entry idt[256];
struct idt_ptr idtp;

extern void idt_flush();

void idt_set_gate(unsigned char num, unsigned long base, unsigned short sel, unsigned char flags) {
  idt[num].base_lo = base & 0xFFFF;
  idt[num].base_hi = (base >> 16) & 0xFFFF;
  idt[num].sel = sel;
  idt[num].always0 = 0;
  idt[num].flags = flags;
}

#define ICW1_INIT	0x10
#define ICW1_ICW4 0x01

#define PIC1_COMM 0x20
#define PIC1_DATA (PIC1_COMM+1)

#define PIC2_COMM 0xa0
#define PIC2_DATA (PIC2_COMM+1)

void idt_install() {
    idtp.limit = (sizeof (struct idt_entry) * 256) - 1;
    idtp.base = (unsigned) &idt;

    idt_set_gate(0, (unsigned)isr0, 0x08, 0x8E);
    idt_set_gate(1, (unsigned)isr1, 0x08, 0x8E);
    idt_set_gate(2, (unsigned)isr2, 0x08, 0x8E);
    idt_set_gate(3, (unsigned)isr3, 0x08, 0x8E);
    idt_set_gate(4, (unsigned)isr4, 0x08, 0x8E);
    idt_set_gate(5, (unsigned)isr5, 0x08, 0x8E);
    idt_set_gate(6, (unsigned)isr6, 0x08, 0x8E);
    idt_set_gate(7, (unsigned)isr7, 0x08, 0x8E);
    idt_set_gate(8, (unsigned)isr8, 0x08, 0x8E);
    idt_set_gate(9, (unsigned)isr9, 0x08, 0x8E);
    idt_set_gate(10, (unsigned)isr10, 0x08, 0x8E);
    idt_set_gate(11, (unsigned)isr11, 0x08, 0x8E);
    idt_set_gate(12, (unsigned)isr12, 0x08, 0x8E);
    idt_set_gate(13, (unsigned)isr13, 0x08, 0x8E);
    idt_set_gate(14, (unsigned)isr14, 0x08, 0x8E);
    idt_set_gate(15, (unsigned)isr15, 0x08, 0x8E);
    idt_set_gate(16, (unsigned)isr16, 0x08, 0x8E);
    idt_set_gate(17, (unsigned)isr17, 0x08, 0x8E);
    idt_set_gate(18, (unsigned)isr18, 0x08, 0x8E);
    idt_set_gate(19, (unsigned)isr19, 0x08, 0x8E);
    idt_set_gate(20, (unsigned)isr20, 0x08, 0x8E);
    idt_set_gate(21, (unsigned)isr21, 0x08, 0x8E);
    idt_set_gate(22, (unsigned)isr22, 0x08, 0x8E);
    idt_set_gate(23, (unsigned)isr23, 0x08, 0x8E);
    idt_set_gate(24, (unsigned)isr24, 0x08, 0x8E);
    idt_set_gate(25, (unsigned)isr25, 0x08, 0x8E);
    idt_set_gate(26, (unsigned)isr26, 0x08, 0x8E);
    idt_set_gate(27, (unsigned)isr27, 0x08, 0x8E);
    idt_set_gate(28, (unsigned)isr28, 0x08, 0x8E);
    idt_set_gate(29, (unsigned)isr29, 0x08, 0x8E);
    idt_set_gate(30, (unsigned)isr30, 0x08, 0x8E);
    idt_set_gate(31, (unsigned)isr31, 0x08, 0x8E);

    c_outb(PIC1_COMM, ICW1_INIT + ICW1_ICW4);
    c_outb(PIC2_COMM, ICW1_INIT + ICW1_ICW4);

    c_outb(PIC1_DATA, 0x20);
    c_outb(PIC2_DATA, 0x28);

    c_outb(PIC1_DATA, 0x04);
    c_outb(PIC2_DATA, 0x02);

    c_outb(PIC1_DATA, 0x01);
    c_outb(PIC2_DATA, 0x01);

    c_outb(PIC1_DATA, 0x0);
    c_outb(PIC2_DATA, 0x0);

    idt_set_gate(32, (unsigned) irq0, 0x08, 0x8E);
    idt_set_gate(33, (unsigned) irq1, 0x08, 0x8E);
    idt_set_gate(34, (unsigned) irq2, 0x08, 0x8E);
    idt_set_gate(35, (unsigned) irq3, 0x08, 0x8E);
    idt_set_gate(36, (unsigned) irq4, 0x08, 0x8E);
    idt_set_gate(37, (unsigned) irq5, 0x08, 0x8E);
    idt_set_gate(38, (unsigned) irq6, 0x08, 0x8E);
    idt_set_gate(39, (unsigned) irq7, 0x08, 0x8E);
    idt_set_gate(40, (unsigned) irq8, 0x08, 0x8E);
    idt_set_gate(41, (unsigned) irq9, 0x08, 0x8E);
    idt_set_gate(42, (unsigned) irq10, 0x08, 0x8E);
    idt_set_gate(43, (unsigned) irq11, 0x08, 0x8E);
    idt_set_gate(44, (unsigned) irq12, 0x08, 0x8E);
    idt_set_gate(45, (unsigned) irq13, 0x08, 0x8E);
    idt_set_gate(46, (unsigned) irq14, 0x08, 0x8E);
    idt_set_gate(47, (unsigned) irq15, 0x08, 0x8E);

    idt_flush();
}

extern void keyboard_handler(unsigned char c);
extern void ata_handler();

void fault_handler(struct regs *r) {
  c_outb(0x20, 0x20);
}

void irq_handler(struct regs *r) {
  if(r->int_no == 33) {
    unsigned char c = c_inb(0x60);
    if(!(c & 0X80)) {
      keyboard_handler(kbdus[c]);
    }
  }

  // if(r->int_no == 46) {
  //   ata_handler();
  // }

  if (r->int_no >= 40) {
    c_outb(PIC2_COMM, 0x20);
  }

  c_outb(PIC1_COMM, 0x20);
}
