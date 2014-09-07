use std::intrinsics::ctpop8;

fn hamming_distance(mut s1: Vec<u8>, mut s2: Vec<u8>) -> uint {
  let mut distance: uint = 0;
  unsafe { // not sure why ctpop8 is unsafe...
    loop {
      match (s1.pop(), s2.pop()) {
        (None, None) => break,
        (None, Some(x)) => distance += ctpop8(x) as uint,
        (Some(x), None) => distance += ctpop8(x) as uint,
        (Some(x), Some(y)) => distance += ctpop8(x ^ y) as uint,
      }
    }
  }
  return distance;
}

fn main() {
  let args: Vec<String> = std::os::args();
  let s1 = Vec::from_slice(args[1].as_bytes());
  let s2 = Vec::from_slice(args[2].as_bytes());
  let distance = hamming_distance(s1, s2);
  println!("distance: {}", distance);
}
