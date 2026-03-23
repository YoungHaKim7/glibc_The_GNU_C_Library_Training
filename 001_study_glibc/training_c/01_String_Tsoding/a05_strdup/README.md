# Result(leak memory 발생)

```bash
opt/homebrew/opt/llvm/bin/clang -g -fsanitize=leak -fno-omit-frame-pointer -c ./src/main.c
/opt/homebrew/opt/llvm/bin/clang -g -fsanitize=leak *.o
mv a.out *.o ./target
./target/a.out

Hello, Worl
sizeof(*s) = 1

=================================================================
==33531==ERROR: LeakSanitizer: detected memory leaks

Direct leak of 72 byte(s) in 1 object(s) allocated from:
    #0 0x000102692834 in malloc+0x60 (libclang_rt.lsan_osx_dynamic.dylib:arm64+0x32834)
    #1 0x0001930ab7e0 in _malloc_type_malloc_outlined+0x60 (libsystem_malloc.dylib:arm64+0x1e7e0)
    #2 0x000192e4aae4 in _fetchInitializingClassList(bool)+0x34 (libobjc.A.dylib:arm64+0xaae4)
    #3 0x000192e4a9a4 in _setThisThreadIsInitializingClass(objc_class*)+0x1c (libobjc.A.dylib:arm64+0xa9a4)
    #4 0x000192e4a7c4 in initializeNonMetaClass+0x250 (libobjc.A.dylib:arm64+0xa7c4)
    #5 0x000192e4a608 in initializeNonMetaClass+0x94 (libobjc.A.dylib:arm64+0xa608)
    #6 0x000192e4a608 in initializeNonMetaClass+0x94 (libobjc.A.dylib:arm64+0xa608)
    #7 0x000192e4a608 in initializeNonMetaClass+0x94 (libobjc.A.dylib:arm64+0xa608)
    #8 0x000192e660a8 in initializeAndMaybeRelock(objc_class*, objc_object*, locker_mixin<lockdebug::lock_mixin<objc_lock_base_t>>&, bool)+0xac (libobjc.A.dylib:arm64+0x260a8)
```


```bash
Hello, Worl
sizeof(*s) = 1
```

```c
#include <stdio.h>
#include <string.h>

int main() {
    char *s = strdup("Hello, World");
    int n = strlen(s);
    s[n - 1] = 0;

    printf("%s\n", (s + 1));
    printf("sizeof(*s) = %zu\n", sizeof(*s));

    return 0;
}

```

```bash
ello, Worl
sizeof(*s) = 1
```
