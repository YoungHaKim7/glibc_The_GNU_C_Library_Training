#include <stdio.h>

int main() {
    char *s = "Hello, World";
    printf("%c\n", *s);
    printf("sizeof(*s) = %zu\n", sizeof(*s));

    return 0;
}
