#include <process.h>
#include <screen.h>

extern unsigned int mode();

void main() {
    print("Hello from User Space!", 0x4321);
}
