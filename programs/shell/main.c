#include <screen.h>

extern void putc_syscall(unsigned char c, unsigned int color);

void main() {
    print("Hello from User Space!", 0x1234);
}
