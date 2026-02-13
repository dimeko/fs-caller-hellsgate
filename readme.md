#### hellgate

#### WinDBG notes

peb: `0x000000812063f000`
ldr: `0x00007ffbb3f939c0`
in_memory_order_module_list: `0x000001d6bc6a5920`

```
Symbol search path is: srv*
Executable search path is: 
ModLoad: 00007ff7`40760000 00007ff7`40792000   rust_syscalls.exe
ModLoad: 00007ffb`b3dc0000 00007ffb`b4028000   ntdll.dll
(340.11b0): C++ EH exception - code e06d7363 (first chance)
ModLoad: 00007ffb`afcb0000 00007ffb`afccb000   C:\WINDOWS\SYSTEM32\kernel.appcore.dll
ModLoad: 00007ffb`b2bd0000 00007ffb`b2c99000   C:\WINDOWS\System32\KERNEL32.DLL
ModLoad: 00007ffb`b3820000 00007ffb`b38c9000   C:\WINDOWS\System32\msvcrt.dll
ModLoad: 00007ffb`b11d0000 00007ffb`b15bd000   C:\WINDOWS\System32\KERNELBASE.dll
ntdll!NtTerminateProcess+0x14:
00007ffb`b3f22354 c3              ret
0:000> x get_module_base
0:000> x rust_syscalls!"rust_syscalls:get_module_base"
0:000> x rust_syscalls!"rust_get_module_base"
0:000> x rust_syscalls!"get_module_base"
0:000> x rust_syscalls!*get_module_base*
00007ff7`407627b0 rust_syscalls!rust_syscalls::get_module_base (void)
0:000> bp 00007ff7`407627b0
0:000> g
ModLoad: 00007ffb`adf50000 00007ffb`adfee000   C:\WINDOWS\SYSTEM32\apphelp.dll
ModLoad: 00007ffb`b1080000 00007ffb`b11cb000   C:\WINDOWS\System32\ucrtbase.dll
ModLoad: 00007ffb`a5e80000 00007ffb`a5e9e000   C:\WINDOWS\SYSTEM32\VCRUNTIME140.dll
(ef4.178c): Break instruction exception - code 80000003 (first chance)
ntdll!LdrpDoDebuggerBreak+0x35:
00007ffb`b3ee4419 cc              int     3
1:002> bp 00007ff7`407627b0
1:002> g
Breakpoint 0 hit
rust_syscalls!rust_syscalls::get_module_base:
00007ff7`407627b0 55              push    rbp
1:002> t
rust_syscalls!rust_syscalls::get_module_base+0x23:
00007ff7`407627d3 65488b142560000000 mov   rdx,qword ptr gs:[60h] gs:00000000`00000060=????????????????
1:002> t
rust_syscalls!rust_syscalls::get_module_base+0x40:
00007ff7`407627f0 488b4568        mov     rax,qword ptr [rbp+68h] ss:00000081`205ef168=000001d6bc6a5920
1:002> dq 0x000001d6bc6a5920
000001d6`bc6a5920  000001d6`bc6a5670 00007ffb`b3f939d0
000001d6`bc6a5930  000001d6`bc6a5680 00007ffb`b3f939e0
000001d6`bc6a5940  00000000`00000000 00000000`00000000
000001d6`bc6a5950  00007ff7`40760000 00007ff7`4077ef60
000001d6`bc6a5960  00000000`00032000 00000000`00840082
000001d6`bc6a5970  000001d6`bc6a5310 00000000`00240022
000001d6`bc6a5980  000001d6`bc6a5370 ffffffff`00002acc
000001d6`bc6a5990  00007ffb`b3f94190 00007ffb`b3f94190
1:002> dq 0x00007ffbb3f939c0
00007ffb`b3f939c0  00000001`00000058 00000000`00000000
00007ffb`b3f939d0  000001d6`bc6a5920 000001d6`bc6b66f0
00007ffb`b3f939e0  000001d6`bc6a5930 000001d6`bc6b6700
00007ffb`b3f939f0  000001d6`bc6a5690 000001d6`bc6b6710
00007ffb`b3f93a00  00000000`00000000 00000000`00000000
00007ffb`b3f93a10  00000000`00000000 000001d6`bc6a5670
00007ffb`b3f93a20  00000000`00000000 98da6000`0fff6df8
00007ffb`b3f93a30  00000000`00000000 00000000`00000000
1:002> dq 0x000000812063f000
00000081`2063f000  00000000`04010000 ffffffff`ffffffff
00000081`2063f010  00007ff7`40760000 00007ffb`b3f939c0
00000081`2063f020  000001d6`bc6a4cc0 00000000`00000000
00000081`2063f030  000001d6`bc6a0000 00007ffb`b3f937c0
00000081`2063f040  00000000`00000000 00000000`00000000
00000081`2063f050  00000000`00000004 00000000`00000000
00000081`2063f060  00000000`00000000 000001d6`bc4e0000
00000081`2063f070  00000000`00000000 00007ffb`b3f8f220
```