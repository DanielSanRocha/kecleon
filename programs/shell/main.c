#include <screen.h>
#include <keyboard.h>

void main(char* args) {
    while(1) {
        int c = getc();
        if(c != 0) {
            putc(c, 0x0000FF);
        }
    }
}
