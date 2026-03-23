# Result(leak memory 해결 잊지말자 free)

```bash
Hello, Worl
sizeof(*s) = 1
```

# macOS leak 조심하자.  무시해도 되는거군만.

**Summary:** Your code has **no memory leaks**. You're correctly calling `free(s)` after `strdup()`. The 120 bytes reported are from macOS system libraries during initialization, which are suppressed with the `.lsan.supp` file.

The real issue in your output is the typo: "Hello, Worl" - this is because line 8 (`s[n - 1] = 0;`) removes the last character 'd' since there's no newline in the original string.
