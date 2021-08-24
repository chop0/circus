#![feature(asm)]
const BITMASK: u128 = 0xFFFE48D0F1;
const PRECISION: u8 = 13;

use nix::sys::ptrace::traceme;
use std::time::{Duration, SystemTime};
use std::sync::mpsc::RecvTimeoutError::Timeout;

fn get_time() -> u128{
    {
        let upper: u32;
        let lower: u32;
        unsafe {
            asm!("rdtsc", out("edx") upper, out("eax") lower);
        };
        (((upper as u64) << 32) | ((lower >> (32 - PRECISION)) as u64)) as u128
    }
}

const fn is_prime(n: u64) -> bool {
    if n <= 3 {
        return n > 1;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    };
    let mut i: u64 = 5;
    while i.pow(2) < n as u64{
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn find_prime_from(num: u64) -> u64 {
    let mut i = num;
    while !is_prime(i) {
        i += 1;
    }
    i
}
pub fn sign(text: [u8; 8]) -> [u8; 16] {
    let current_time = get_time();
//    println!("sign {}", current_time);


    let time_signature = find_prime_from((current_time ^ BITMASK) as u64) as u128;
//println!(" {}", time_signature);
    let signed_message = (u64::from_le_bytes(text) as u128 * time_signature);
    signed_message.to_le_bytes()
}

pub fn verify(text: [u8; 16], lemon: bool) -> Option<[u8; 8]> {
    let mut current_time = get_time();
   if lemon {
       current_time /= 2;
   }

    let time_signature = find_prime_from((current_time ^ BITMASK) as u64) as u128;
   //println!(" {}", time_signature);
    let msg = u128::from_le_bytes(text);
    if msg % time_signature == 0 {
        Some(((msg / (time_signature)) as u64).to_le_bytes())
    } else {
        None
    }
}
