#include "process.h"

extern int exit_syscall();

int exit() {
    return exit_syscall();
}