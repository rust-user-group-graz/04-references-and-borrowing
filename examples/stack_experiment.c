#include <stdio.h>

void func() {
    int a;
    printf("was %d, setting to %d\n", a, 42);
    a = 42;
}

int main() {
    func();
    func();
    return 0;
}
