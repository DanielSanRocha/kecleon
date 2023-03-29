void c_outb (unsigned int _port, unsigned char _data) {
    __asm__ __volatile__ ("outb %1, %0" : : "dN" (_port), "a" (_data));
}

unsigned char c_inb(unsigned int port) {
    unsigned char ret;
    asm volatile ( "inb %1, %0"
                   : "=a"(ret)
                   : "Nd"(port) );
    return ret;
}
