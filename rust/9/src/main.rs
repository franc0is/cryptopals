#![feature(str_escape)]

use std::env;
use std::usize;
use std::u8;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut msg = args[1].clone().into_bytes();
    let block_size = usize::from_str_radix(&args[2], 10).unwrap();

    if block_size > u8::MAX as usize {
        println!("Block sizes above 256 bytes are invalid!");
        return;
    }

    let to_pad = block_size - msg.len() % block_size;
    let mut padding = vec![to_pad as u8; to_pad];
    msg.append(&mut padding);

    println!("{}", String::from_utf8(msg).unwrap().escape_default());
}
