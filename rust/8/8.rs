extern crate serialize;
use serialize::base64::FromBase64;
use std::io::File;
use std::io::BufferedReader;
use std::collections::HashSet;

fn is_aes_ecb(bytes: Vec<u8>) -> bool {
  let len = bytes.len() / 16;
  let mut blocks = bytes.as_slice().chunks(16);
  let mut set: HashSet<&[u8]> = HashSet::new();
  for block in blocks {
    set.insert(block);
  }

  len != set.len()
}

fn main() {
  let args: Vec<String> = std::os::args();
  let path = Path::new(args[1].clone());
  let mut file = BufferedReader::new(File::open(&path));
  for (i, line_iter) in file.lines().enumerate() {
    let input: Vec<u8> = line_iter.unwrap().into_bytes();
    let bytes = input.as_slice().from_base64().unwrap();
    if is_aes_ecb(bytes) {
      println!("Found ECB at line {}", i);
    }
  }
}
