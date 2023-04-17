#include <screen.h>
#include <keyboard.h>
#include <process.h>

char prog[128];
char args[128];
char command[256];

void run() {
    prog[0] = '/';
    prog[1] = 'b';
    prog[2] = 'i';
    prog[3] = 'n';
    prog[4] = '/';

    int j = 0;
    for(j=0;command[j] != ' ' && command[j] != 0;j++) {
        prog[j + 5] = command[j];
    }
    prog[j + 5] = 0;
    j++;

    int i = 0;
    for(; command[j] != 0; j++) {
        args[i] = command[j];
        i++;
    }
    args[i] = 0;

    for(i=0;i<256;i++) {
        command[i] = 0;
    }

    print("\n", 0xFF);
    int pid = exec(prog, args);

    if(pid < 0) {
        print("\nError starting process: Code -", 0xFF);
        print_int(-pid, 0xFF);
        print("\n>", 0x4321);
    } else {
        print("\n>", 0x4321);
    }
}

void main(char* args) {
    unsigned int index = 0;

    putc('>', 0x4321);
    while(1) {
        int c = getc();
        if(c != 0) {
            if(index == 255) {
                print("Buffer is full", 0xFF);
            } else {
                if(c == '\n') {
                    run();
                    index = 0;
                } else {
                    putc(c, 0xFFFFFF);
                    command[index] = c;
                    index++;
                }
            }
        }

    }
}
