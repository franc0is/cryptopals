#![feature(io)]

extern crate base64;
extern crate openssl;

use openssl::symm::*;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn main() {
  let args: Vec<String> = env::args().collect();

  // read file data into byte array
  let file = BufReader::new(File::open(&args[1]).unwrap());
  let chars = file.chars();
  let mut b = [0; 4];
  let bytes = chars.map(|c| c.unwrap())
                   .filter(|&c| c != '\n')
                   .map(|c| base64::decode(c.encode_utf8(&mut b)).unwrap());
  /*
  let mut input: Vec<u8> = Vec::new();
  file.read_to_end(&mut input).unwrap();
  // remove line breaks
  input.retain(|b| *b != '\n' as u8);
  let bytes = base64::decode(&input).unwrap();
  */

  let key: &[u8] = "YELLOW SUBMARINE".as_bytes();
  //let iv: &[u8] = [0; 3];

  let decrypted = decrypt(Cipher::aes_128_ecb(), key, None, bytes).unwrap();
  println!("{}", String::from_utf8(decrypted).unwrap());
}
