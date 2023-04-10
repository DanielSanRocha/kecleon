#include "screen.h"

extern int putc_syscall(unsigned char c, unsigned int color);

void print(char* str, unsigned int color) {
    for(unsigned int index = 0; str[index] != 0; index++) {
        putc_syscall(str[index], color);
    }
}

void print_int_loop(unsigned int n, unsigned char c, unsigned int color) {
    if(c == 0) {
        return;
    }

    unsigned int j = n;
    unsigned int decimal = 1;

    for(unsigned char i=0;i<c -1 ;i++) {
        j = j/10;
        decimal *= 10;
    }

    putc_syscall(48 + (unsigned char) j, color);
    unsigned int new_n = n - decimal * j;
    if(new_n == 0) {
        for(unsigned char k=0;k<c-1;j++) {
            putc_syscall((unsigned char) '0', color);
        }
    } else {
        print_int_loop(new_n, c - 1, color);
    }
}

void print_int(unsigned int number, unsigned int color) {
    unsigned int j = number;
    unsigned char c = 0;

    while(j>=10) {
        j = j/10;
        c++;
    }

    print_int_loop(number, c+1, color);
}

