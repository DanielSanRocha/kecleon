#include "keyboard.h"

extern int getc_syscall();

int getc() {
    return getc_syscall();
}
