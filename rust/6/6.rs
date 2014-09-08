use std::intrinsics::ctpop8;
use std::int::parse_bytes;

fn hamming_distance(mut s1: Vec<u8>, mut s2: Vec<u8>) -> uint {
  let mut distance: uint = 0;
  loop {
    match (s1.pop(), s2.pop()) {
      (None, None) => break,
      (None, Some(x)) | (Some(x), None) => distance += x.count_ones() as uint,
      (Some(x), Some(y)) => distance += (x ^ y).count_ones() as uint
    }
  }

  return distance;
}

fn int_to_bytes(mut number: int) -> Vec<u8> {
  let mut bytes: Vec<u8> = Vec::new();
  while number > 0 {
    bytes.push((number & 0xff) as u8);
    number = number >> 8;
  }

  return bytes;
}

fn base64_to_bytes(base64: &[u8]) -> Vec<u8> {
  // 2 base 64 digits to 3 hex digits
  let base64_chunks = base64.chunks(2);
  let mut bytes: Vec<u8> = Vec::new();
  for chunk in base64_chunks.clone() {
    let value = parse_bytes(chunk, 64).unwrap();
    bytes.push_all_move(int_to_bytes(value));
  }
  return bytes;
}

fn main() {
  let args: Vec<String> = std::os::args();
  let s1 = Vec::from_slice(args[1].as_bytes());
  let s2 = Vec::from_slice(args[2].as_bytes());
  let distance = hamming_distance(s1, s2);
  println!("distance: {}", distance);
}
