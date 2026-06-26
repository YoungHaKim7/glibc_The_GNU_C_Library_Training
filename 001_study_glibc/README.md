# 커널 이해하기(C언어 개념으로 이해해야함)
- https://youtu.be/iqqf8YWJhSs?si=wKzFVK7XLl_SlhjG

# linux man-pages
- https://man7.org/linux/man-pages/index.html

- 이 텍스트는 Linux 매뉴얼 페이지를 웹에서 볼 수 있는 Linux man-pages online 사이트의 첫 화면입니다. 보통 개발자들은 터미널에서:

```bash
man printf
man mmap
man pthread_create
```

- 처럼 사용하지만, 이 사이트에서는 같은 내용을 브라우저에서 HTML로 볼 수 있습니다.

- 주요 항목은 다음과 같습니다.
  - by section : 매뉴얼 섹션별 목록
  - alphabetically : 알파벳 순 목록
  - by project : 프로젝트별 목록
  - 1 ~ 8 : 각 매뉴얼 섹션

- Linux 매뉴얼 섹션은 보통 다음 의미입니다.


| 섹션 | 내용                                                |
| -- | ------------------------------------------------- |
| 1  | 사용자 명령어 (`ls`, `grep`, `cat`)                     |
| 2  | 시스템 호출 (`open`, `read`, `mmap`)                   |
| 3  | C 라이브러리 함수 (`printf`, `malloc`, `pthread_create`) |
| 4  | 장치 파일                                             |
| 5  | 설정 파일 형식 (`passwd`, `fstab`)                      |
| 6  | 게임                                                |
| 7  | 개념 및 규약 (`signal`, `tcp`, `utf-8`)                |
| 8  | 시스템 관리 명령어 (`mount`, `fsck`)                      |

# 예를 들어 Rust에서 `libc::mmap` 을 공부하고 싶다면:

```bash
man 2 mmap
```

- 또는 웹에서 `mmap(2)` 페이지를 보면 됩니다. 시스템 호출은 섹션 2에 있기 때문입니다.

- Rust + libc 프로그래밍을 자주 한다면 특히 많이 보는 섹션은:
  - `man 2` — 시스템 호출
  - `man 3` — C 라이브러리 함수
  - `man 7` — 개념 설명

- 입니다.

- 예를 들어 최근에 질문했던 `libc`, `fork`, `signal`, `mmap`, `pthread` 관련 내용은 대부분 Section 2 와 Section 3 에 있습니다.

- 또한 macOS에서도 터미널에서 동일하게 사용할 수 있습니다.

```bash
man printf
man pthread_create
man mmap
```

- 특정 함수가 어느 섹션에 있는지 모르면:

```bash
man -k mmap
```

- 또는

```bash
apropos mmap
```
- 으로 검색할 수 있습니다.

<hr />

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

#
Perfect! The glibc manual has been built successfully. The HTML documentation is now available at:

**`glibc/build/manual/libc/`**

You can open the main index page in your browser:

```bash
xdg-open glibc/build/manual/libc/index.html
```

Or if you want to view it from the terminal with a text-based browser:

```bash
lynx glibc/build/manual/libc/index.html
```

**Summary:** The original error occurred because glibc requires building from a separate build directory (not from the source tree directly). The fix was to:

1. Create a build directory: `mkdir build && cd build`
2. Configure glibc: `../configure --prefix=/usr`
3. Build the manual: `make -C ../manual objdir=$(pwd) html`
