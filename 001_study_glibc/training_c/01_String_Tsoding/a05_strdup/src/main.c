#include <stdio.h>
#include <string.h>

int main() {
    char *s = strdup("Hello, World");
    int n = strlen(s);
    s[n - 1] = 0;

    printf("%s\n", s);
    printf("sizeof(*s) = %zu\n", sizeof(*s));

    return 0;
}
