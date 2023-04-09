#include "screen.h"

extern void putc_syscall(unsigned char c, unsigned int color);

void print(char* str, unsigned int color) {
    for(unsigned int index = 0; str[index] != 0; index++) {
        putc_syscall(str[index], color);
    }
}
