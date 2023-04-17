#include "process.h"

extern int exit_syscall();
extern int exec_syscall();

int exit() {
    return exit_syscall();
}

int exec(char* program, char* arguments) {
    return exec_syscall(program,arguments);
}