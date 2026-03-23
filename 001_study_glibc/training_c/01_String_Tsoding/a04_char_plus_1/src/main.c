#include <stdio.h>

int main() {
    char *s = "Hello, World";
    printf("%s\n", s + 1);
    printf("sizeof(*s) = %zu\n", sizeof(*s));

    return 0;
}
