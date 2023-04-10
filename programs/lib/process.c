#include "process.h"

extern void exit_syscall();

void exit() {
    exit_syscall();
}