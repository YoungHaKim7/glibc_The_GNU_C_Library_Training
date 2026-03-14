# 공부하는 순서
Based on my exploration of the glibc codebase, here's a structured approach to studying it:

## Recommended Study Path

### 1. **Start with Documentation** (`manual/`)
The manual/ directory contains comprehensive Texinfo documentation. Key files:
- `intro.texi` - Introduction and overview
- `creature.texi` - Library description and standards
- `startup.texi` - Program startup and initialization

### 2. **Foundation: C Startup** (`csu/`)
The "C Startup" directory contains crt0.o and related files - crucial for understanding how programs begin execution:
```bash
ls -la csu/
```

### 3. **Core Subsystems to Explore**

| Directory | Purpose | Good for Learning |
|-----------|---------|-------------------|
| `string/` | String operations (strcpy, strlen, etc.) | ✅ Simple, pure C |
| `ctype/` | Character classification | ✅ Straightforward |
| `stdlib/` | Standard library functions | ✅ Core utilities |
| `posix/` | POSIX compliance layer | ✅ System interfaces |
| `malloc/` | Memory allocation | ⚠️ Complex but important |
| `stdio-common/` | Standard I/O | ⚠️ More complex |
| `nptl/` | Native POSIX threads | ⚠️ Advanced |

### 4. **Header Organization** (`include/` and `bits/`)
- `include/` - Main public headers
- `bits/` - Architecture-specific fragments

### 5. **Build System**
- `Makeconfig`, `Makerules` - Build configuration
- Understanding the build helps understand the code organization

## Suggested First Steps

1. **Read `manual/intro.texi`** for the big picture
2. **Browse `string/`** for simple, readable implementations
3. **Examine `csu/`** to understand program startup
4. **Look at `stdlib/`** for fundamental utilities

Would you like me to show you a specific simple function to get started, such as strlen or strcmp?

# 책 읽는 방법
**The `info` command works!** Your system has the glibc documentation installed.

## Summary: How to View the glibc Manual

You have **3 options**:

### 1. **Use `info` command** (Best - interactive navigation)
```bash
info libc
```
- Navigate with arrow keys
- Press `n` for next, `p` for previous, `u` for up
- Press `q` to quit
- Press `Enter` on menu items to jump to sections

### 2. **Read `.texi` files directly** (Simple - just read)
The Texinfo files are readable text:
- `manual/intro.texi` - Introduction (what I showed above)
- `manual/creature.texi` - Library overview
- `manual/string.texi` - String functions
- `manual/startup.texi` - Program startup

### 3. **Online documentation**
Visit https://www.gnu.org/software/libc/manual/

---

**Which topic interests you most?** Based on your README, I'd recommend starting with:
- `manual/intro.texi` - Big picture overview
- `manual/string.texi` - Simple, readable implementations (good starting point)

Would you like me to show you a specific section?
