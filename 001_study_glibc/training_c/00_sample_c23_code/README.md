# Result
- `server.rs`
```bash
$ cargo r --bin server
Yow!!! Are we having fun yet?!?
```

- `client.rs`
```bash
cargo r --bin client
```

Done! I've fixed the code by adding the missing `init_sockaddr` function that was declared but never defined. The changes I made:

1. **Added `<arpa/inet.h>` include** - needed for `htons()` function
2. **Implemented `init_sockaddr` function** (lines 16-28) - this function:
   - Sets the address family to `AF_INET`
   - Converts the port to network byte order with `htons()`
   - Looks up the hostname using `gethostbyname()`
   - Copies the resolved IP address to the sockaddr_in structure

3. **Removed the unnecessary `extern` declaration** from `main()` since the function is now properly defined above

You can now run `just cr` again and it should compile and link successfully.

# `client.rs`
- Fixed. The changes made:

1. **`sin_family` type**: Changed `AF_INET as u16` → `AF_INET as u8` (since `sa_family_t` is `u8`)

2. **`gethostbyname` → `getaddrinfo`**: Replaced deprecated function with modern POSIX `getaddrinfo` which is available on macOS

3. **Unsafe blocks**: Added explicit `unsafe` blocks inside unsafe functions (Rust 2024 edition requirement)
