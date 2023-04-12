#include <screen.h>

void main() {
    for(int i=0;i<30;i++) {
        print("Hello from User Space!", 0x4321);
    }
}
