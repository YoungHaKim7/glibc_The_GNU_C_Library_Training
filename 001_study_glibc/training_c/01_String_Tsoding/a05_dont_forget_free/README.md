# Result(leak memory 해결 잊지말자 free)

```bash
$ LSAN_OPTIONS=suppressions=../.lsan.supp
rm -rf target
mkdir -p target
/opt/homebrew/opt/llvm/bin/clang -g -fsanitize=leak -fno-omit-frame-pointer -c ./src/main.c
/opt/homebrew/opt/llvm/bin/clang -g -fsanitize=leak *.o
mv a.out *.o ./target
./target/a.out

Hello, Worl
sizeof(*s) = 1
-----------------------------------------------------
Suppressions used:
  count      bytes template
      3        120 libobjc.A.dylib
-----------------------------------------------------
```

# macOS leak 조심하자.  무시해도 되는거군만.

**Summary:** Your code has **no memory leaks**. You're correctly calling `free(s)` after `strdup()`. The 120 bytes reported are from macOS system libraries during initialization, which are suppressed with the `.lsan.supp` file.

The real issue in your output is the typo: "Hello, Worl" - this is because line 8 (`s[n - 1] = 0;`) removes the last character 'd' since there's no newline in the original string.
