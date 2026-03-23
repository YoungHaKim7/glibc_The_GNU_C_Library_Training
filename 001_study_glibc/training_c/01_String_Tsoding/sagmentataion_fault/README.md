# Result

```bash
opt/homebrew/opt/llvm/bin/clang -g -fsanitize=address -fno-omit-frame-pointer -c ./src/main.c
/opt/homebrew/opt/llvm/bin/clang -g -fsanitize=address *.o
mv a.out *.o ./target
./target/a.out
AddressSanitizer:DEADLYSIGNAL
=================================================================
==29004==ERROR: AddressSanitizer: BUS on unknown address (pc 0x000102b1885c bp 0x00016d2e6460 sp 0x00016d2e6420 T0)
==29004==The signal is caused by a WRITE memory access.
==29004==Hint: this fault was caused by a dereference of a high value address (see register values below).  Disassemble the provided pc to learn which register was used.
    #0 0x000102b1885c in main+0xac (a.out:arm64+0x10000085c)
    #1 0x000192ecdd50  (<unknown module>)

==29004==Register values:
 x[0] = 0x000000000000000c   x[1] = 0x0000000102b189a0   x[2] = 0x000000000000000c   x[3] = 0x000000019329e9f0
 x[4] = 0x0000000000000001   x[5] = 0x0000000000000010   x[6] = 0x0000000000000041   x[7] = 0x0000000000000000
 x[8] = 0x0000000102b189ab   x[9] = 0x000000000000000b  x[10] = 0x000000000000000b  x[11] = 0x000000000000000b
x[12] = 0x0000000000000001  x[13] = 0x0000000000000005  x[14] = 0xf9f9050000000000  x[15] = 0x0000000000000000
x[16] = 0x000000019329eaa0  x[17] = 0x0000000200b1f268  x[18] = 0x0000000000000000  x[19] = 0x00000001ff2ac060
x[20] = 0x00000001ff5592c8  x[21] = 0x00000001ff2acdf0  x[22] = 0xfffffffffffffff0  x[23] = 0x00000001ff55c9e0
x[24] = 0x0000000000000001  x[25] = 0x000000016d2e6660  x[26] = 0x00000001ff55c9f0  x[27] = 0x0000000000000000
x[28] = 0x0000000000000000     fp = 0x000000016d2e6460     lr = 0x0000000102b187e4     sp = 0x000000016d2e6420
AddressSanitizer can not provide additional info.
SUMMARY: AddressSanitizer: BUS (a.out:arm64+0x10000085c) in main+0xac
==29004==ABORTING
```

# dtruss(macOS)

```bash
$ sudo dtruss ./a.out

SYSCALL(args) 		 = return
AddressSanitizer:DEADLYSIGNAL
=================================================================
==29304==ERROR: AddressSanitizer: BUS on unknown address (pc 0x000104df885c bp 0x00016b006ab0 sp 0x00016b006a70 T0)
==29304==The signal is caused by a WRITE memory access.
==29304==Hint: this fault was caused by a dereference of a high value address (see register values below).  Disassemble the provided pc to learn which register was used.
    #0 0x000104df885c in main+0xac (a.out:arm64+0x10000085c)
    #1 0x000192ecdd50  (<unknown module>)

==29304==Register values:
 x[0] = 0x000000000000000c   x[1] = 0x0000000104df89a0   x[2] = 0x000000000000000c   x[3] = 0x000000019329e9f0
 x[4] = 0x0000000000000001   x[5] = 0x0000000000000010   x[6] = 0x0000000000000041   x[7] = 0x0000000000000000
 x[8] = 0x0000000104df89ab   x[9] = 0x000000000000000b  x[10] = 0x000000000000000b  x[11] = 0x000000000000000b
x[12] = 0x0000000000000001  x[13] = 0x0000000000000005  x[14] = 0xf9f9050000000000  x[15] = 0x0000000000000000
x[16] = 0x000000019329eaa0  x[17] = 0x0000000200b1f268  x[18] = 0x0000000000000000  x[19] = 0x00000001ff2ac060
x[20] = 0x00000001ff5592c8  x[21] = 0x00000001ff2acdf0  x[22] = 0xfffffffffffffff0  x[23] = 0x00000001ff55c9e0
x[24] = 0x0000000000000001  x[25] = 0x000000016b006cb0  x[26] = 0x00000001ff55c9f0  x[27] = 0x0000000000000000
x[28] = 0x0000000000000000     fp = 0x000000016b006ab0     lr = 0x0000000104df87e4     sp = 0x000000016b006a70
AddressSanitizer can not provide additional info.
SUMMARY: AddressSanitizer: BUS (a.out:arm64+0x10000085c) in main+0xac
==29304==ABORTING
crossarch_trap(0x0, 0x0, 0x0)		 = -1 Err#45
csrctl(0x0, 0x16B006A9C, 0x4)		 = -1 Err#1
proc_info(0xF, 0x7278, 0x0)		 = 0 0
thread_selfid(0x0, 0x0, 0x0)		 = 72509 0
crossarch_trap(0x0, 0x0, 0x0)		 = -1 Err#45
csrctl(0x0, 0x16B00693C, 0x4)		 = -1 Err#1
crossarch_trap(0x0, 0x0, 0x0)		 = -1 Err#45
fsgetpath(0x16B006CB0, 0x400, 0x16B006B88)		 = 14 0
shared_region_check_np(0x16B006CA0, 0x0, 0x0)		 = 0 0
getpid(0x0, 0x0, 0x0)		 = 29304 0
munmap(0x104F5C000, 0xA0000)		 = 0 0
munmap(0x104FFC000, 0x8000)		 = 0 0
munmap(0x105004000, 0x4000)		 = 0 0
munmap(0x105008000, 0x4000)		 = 0 0
munmap(0x10500C000, 0x24000)		 = 0 0
munmap(0x105030000, 0x60000)		 = 0 0
crossarch_trap(0x0, 0x0, 0x0)		 = -1 Err#45
open(".\0", 0x100000, 0x0)		 = 3 0
fcntl(0x3, 0x32, 0x16B006238)		 = 0 0
close(0x3)		 = 0 0
fsgetpath(0x16B006248, 0x400, 0x16B006228)		 = 154 0
fsgetpath(0x16B006258, 0x400, 0x16B006238)		 = 14 0
csrctl(0x0, 0x16B00665C, 0x4)		 = -1 Err#1
__mac_syscall(0x192F623DE, 0x2, 0x16B006580)		 = 0 0
csrctl(0x0, 0x16B00666C, 0x4)		 = -1 Err#1
sysctl([unknown, 3, 0, 0, 0, 0] (2), 0x16B0065E8, 0x16B0065E0, 0x192F591BA, 0x20)		 = 0 0
sysctl([unknown, 100, 101, 0, 0, 0] (3), 0x16B00667C, 0x16B006670, 0x0, 0x0)		 = 0 0
__mac_syscall(0x192F5F135, 0x5A, 0x16B006600)		 = 0 0
```
