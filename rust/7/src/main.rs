extern crate base64;
extern crate openssl;

use openssl::symm::*;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn main() {
  let args: Vec<String> = env::args().collect();
  let mut file = BufReader::new(File::open(&args[1]).unwrap());
  let mut input: Vec<u8> = Vec::new();
  file.read_to_end(&mut input).unwrap();
  // remove line breaks
  input.retain(|b| *b != '\n' as u8);
  let bytes = base64::decode(&input).unwrap();
  let key: &[u8] = "YELLOW SUBMARINE".as_bytes();
  let decrypted = decrypt(Cipher::aes_128_ecb(), key, None, bytes.as_slice()).unwrap();
  println!("{}", String::from_utf8(decrypted).unwrap());
}
