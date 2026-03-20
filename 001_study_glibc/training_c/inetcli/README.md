Done! I've fixed the code by adding the missing `init_sockaddr` function that was declared but never defined. The changes I made:

1. **Added `<arpa/inet.h>` include** - needed for `htons()` function
2. **Implemented `init_sockaddr` function** (lines 16-28) - this function:
   - Sets the address family to `AF_INET`
   - Converts the port to network byte order with `htons()`
   - Looks up the hostname using `gethostbyname()`
   - Copies the resolved IP address to the sockaddr_in structure

3. **Removed the unnecessary `extern` declaration** from `main()` since the function is now properly defined above

You can now run `just cr` again and it should compile and link successfully.
