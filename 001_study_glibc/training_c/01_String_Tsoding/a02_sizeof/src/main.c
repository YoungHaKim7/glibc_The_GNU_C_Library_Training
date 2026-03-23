#include <stdio.h>

int main() {
    char *s = "Hello, World";
    printf("%s\n", s);
    printf("sizeof(*s) = %zu\n", sizeof(*s));

    return 0;
}
