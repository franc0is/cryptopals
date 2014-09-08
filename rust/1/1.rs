extern crate serialize;
use serialize::base64::{ToBase64, STANDARD};

fn main() {
  let args: Vec<String> = std::os::args();
  let hex_string = args[1].as_bytes();

  let mut hex_chunks = hex_string.chunks(2);

  let mut bytes: Vec<u8> = Vec::new();
  for hex_chunk in hex_chunks {
    let num = std::int::parse_bytes(hex_chunk, 16);
    bytes.push(num.unwrap() as u8);
  }

  let base64 = bytes.as_slice().to_base64(STANDARD);

  println!("{}", base64);

}