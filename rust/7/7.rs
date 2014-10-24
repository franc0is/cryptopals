#![feature(globs)]

extern crate openssl;
extern crate serialize;
use openssl::crypto::symm::*;
use serialize::base64::FromBase64;
use std::io::File;
use std::io::BufferedReader;

fn main() {
  let args: Vec<String> = std::os::args();
  let path = Path::new(args[1].clone());
  let mut file = BufferedReader::new(File::open(&path));
  let input: Vec<u8> = file.read_to_end().unwrap();
  let bytes = input.as_slice().from_base64().unwrap();
  let key: &[u8] = "YELLOW SUBMARINE".as_bytes();
  let decrypted = decrypt(AES_128_ECB, key, Vec::new(), bytes.as_slice());
  println!("{}", String::from_utf8(decrypted).unwrap());
}
