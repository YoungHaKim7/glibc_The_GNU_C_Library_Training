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
