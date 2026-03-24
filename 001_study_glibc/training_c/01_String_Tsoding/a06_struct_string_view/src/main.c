#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
    const char *data;
    size_t count;
} String_View;

// Hello, World0...
// ^          ^

String_View sv(const char *cstr) {
    return (String_View){
        .data = cstr,
        .count = strlen(cstr),
    };
}

void sv_chop_left(String_View *sv) {
    if (sv->count == 0)
        return;
    sv->count -= 1;
    sv->data += 1;
}

void sv_chop_right(String_View *sv) {
    if (sv->count == 0)
        return;
    sv->count -= 1;
}

int main() {
    String_View s = sv("Hello, World");
    sv_chop_left(&s);
    printf("%.*s\n", (int)s.count, s.data);

    return 0;
}
