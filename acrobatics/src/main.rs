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


fn prompt(text: &str, time_per_character: u32) {
    for char in text.chars() {
        print!("{}", char);
        std::io::stdout().flush().unwrap();
        sleep(Duration::from_millis(time_per_character.into()));
    }
}

fn to_ascii(text: &str) -> String {
    let mut string = String::new();
    let mut strings = vec![];

    for char in text.chars() {
        strings.push(to_ascii_char(&BASIC_FONTS.get(char).unwrap()));
    }

    for i in 0..8 {
        for lines in &strings {
            lines.get(i).map(|x| string.push_str(x));
        }
        string.push('\n');
    }


    string
}

fn to_ascii_char(char: &[u8; 8]) -> Vec<String> {
    let mut string = vec![];

    for x in char {
        let mut line = String::new();
        for bit in 0..8 {
            match *x & 1 << bit {
                0 => line.push(' '),
                _ => line.push('█'),
            }
        }
        string.push(line);
    };

    string
}


fn main() {
    //   prompt("Welcome to the circus.\n", 50);
    //  prompt("...\n", 500);

   let lemon = traceme().is_err();
    //prompt(&to_ascii("Opening act:  les elephants."), 0);
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


        // let mut line = String::new();
        // std::io::stdin().read_line(&mut line).unwrap(); // including '\n'
        // line.pop(); // remove trailing newline
        //
        // let mut message = line.as_bytes().to_vec();
        //
        //
        // if message.len() % 8 != 0 {
        //     for i in 0..(message.len() % 8) {
        //         message.push(b'\x02');
        //     }
        // }
        // // for i in 0..8 {
        // //     message.push(b'\x01');
        // // }
        // let message = message.as_slice();
        //
        // let mut index = 0;
        // let mut readbuf = [0u8; 4];
        // while index + 8 < message.len() {
        //     let mut a: [u8; 8] = Default::default();
        //
        //     a.copy_from_slice(&message[index..index + 8]);
        //     let message = sign(a);
        //     stream.write(&message);
        //     if let Ok(_) = stream.read_exact(&mut readbuf) {
        //         println!("{:?}", readbuf);
        //         if readbuf == [0, 1, 2, 3] {
        //             readbuf = [0, 0, 0, 0];
        //             index += 8;
        //         }
        //     }
         }
        // index = 0;
    
//    sleep(Duration::from_millis(500));
    //  println!("{}", std::str::from_utf8(&verify(message).unwrap()).unwrap());
} /*else {
        let mut connection = TcpStream::connect("localhost:3333").unwrap();
        connection.set_read_timeout(Some(Duration::from_millis(100)));
        let mut buf = [0u8; 4];
        while &buf != &[1, 1, 1, 1] {
            connection.write(&sign(*b"flagflag"));
            connection.read(&mut buf);
        }
       // println!("{}", std::str::from_utf8(&verify(buf).unwrap()).unwrap());
    }*/


