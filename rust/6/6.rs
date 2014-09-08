extern crate serialize;
use serialize::base64::FromBase64;

#[allow(dead_code)]
fn print_hex(string: Vec<u8>) {
  for byte in string.iter() {
    print!("{:02x}", byte.clone());
  }
  print!("\n");
}

fn hamming_distance(s1: &[u8], s2: &[u8]) -> uint {
  let mut distance: uint = 0;
  let mut it1 = s1.iter();
  let mut it2 = s2.iter();
  loop {
    match (it1.next(), it2.next()) {
      (None, None) => break,
      (None, Some(x)) | (Some(x), None) => distance += x.count_ones() as uint,
      (Some(x), Some(y)) => distance += (*x ^ *y).count_ones() as uint
    }
  }

  return distance;
}

fn find_keysize(bytes: Vec<u8>) -> uint {
  let mut best_score: f32 = 99999.9;
  let mut best_size: uint = 0;

  // compare all combinations of the first 3 samples
  for keysize in range(3u, 40u) {
    if 4 * keysize > bytes.len() {
      break;
    }

    let s1 = bytes.slice(0 * keysize, 1 * keysize - 1);
    let s2 = bytes.slice(1 * keysize, 2 * keysize - 1);
    let s3 = bytes.slice(2 * keysize, 3 * keysize - 1);
    let s4 = bytes.slice(3 * keysize, 4 * keysize - 1);
    let score: f32  = (hamming_distance(s1, s2) +
                       hamming_distance(s1, s3) +
                       hamming_distance(s1, s4) +
                       hamming_distance(s2, s3) +
                       hamming_distance(s2, s4) +
                       hamming_distance(s3, s4))
                       as f32 / 6.0 / keysize as f32;
    if score < best_score {
      // smaller is better
      best_score = score;
      best_size = keysize;
    }
  }

  return best_size;
}

fn test() {
  // check that we didn't break the hamming distance
  assert!(hamming_distance("this is a test".as_bytes(), "wokka wokka!!!".as_bytes()) == 37);

  // check that we can find the key for a known dataset
  let input: &[u8] = [0x0b, 0x36, 0x37, 0x27, 0x2a, 0x2b, 0x2e, 0x63, 0x62, 0x2c, 0x2e, 0x69,
                      0x69, 0x2a, 0x23, 0x69, 0x3a, 0x2a, 0x3c, 0x63, 0x24, 0x20, 0x2d, 0x62,
                      0x3d, 0x63, 0x34, 0x3c, 0x2a, 0x26, 0x22, 0x63, 0x24, 0x27, 0x27, 0x65,
                      0x27, 0x2a, 0x28, 0x2b, 0x2f, 0x20, 0x43, 0x0a, 0x65, 0x2e, 0x2c, 0x65,
                      0x2a, 0x31, 0x24, 0x33, 0x3a, 0x65, 0x3e, 0x2b, 0x20, 0x27, 0x63, 0x0c,
                      0x69, 0x2b, 0x20, 0x28, 0x31, 0x65, 0x28, 0x63, 0x26, 0x30, 0x2e, 0x27,
                      0x28, 0x2f];
  assert!(find_keysize(Vec::from_slice(input)) == 3);
}

fn main() {
  test();
  let args: Vec<String> = std::os::args();
  let input = args[1].as_bytes();
  let bytes = input.from_base64().unwrap();
  let keysize = find_keysize(bytes);
  println!("{}", keysize);
}





