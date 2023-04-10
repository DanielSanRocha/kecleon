#include <process.h>
#include <screen.h>

void main() {
    print("Hello from User Space!", 0x4321);

    exit();
    while(1);
}
