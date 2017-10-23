extern crate base64;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;

fn is_aes_ecb(bytes: Vec<u8>) -> bool {
  let len = bytes.len() / 16;
  let blocks = bytes.as_slice().chunks(16);
  let mut set: HashSet<&[u8]> = HashSet::new();
  for block in blocks {
    set.insert(block);
  }

  len != set.len()
}

fn hex_to_bytes(s: &String) -> Vec<u8> {
  let mut bytes = Vec::new();
  for i in 0 .. (s.len() / 2) {
    let byte = u8::from_str_radix(&s[2*i .. 2*i+2], 16).unwrap();
    bytes.push(byte);
  }
  return bytes;
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let file = BufReader::new(File::open(&args[1]).unwrap());
  for (i, line) in file.lines().enumerate() {
    let bytes: Vec<u8> = hex_to_bytes(&line.unwrap());
    if is_aes_ecb(bytes) {
      println!("Found ECB at line {}", i);
    }
  }
}
