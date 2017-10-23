extern crate base64;

use base64::{encode};
use std::env;
use std::u8;

fn main() {
  let args: Vec<String> = env::args().collect();
  let mut bytes = Vec::new();

  for i in 0 .. (args[1].len() / 2) {
    let res = u8::from_str_radix(&args[1][2*i .. 2*i+2], 16);
    match res {
      Ok(v) => bytes.push(v),
      Err(e) => println!("Problem with hex: {}", e),
    };
  }

  let b64 = encode(bytes.as_slice());

  println!("{}", b64);
}