extern crate serialize;
use serialize::base64::FromBase64;
use std::ascii::OwnedAsciiExt;
use std::io::File;
use std::io::BufferedReader;
use std::collections::SmallIntMap;

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

fn find_keysize(bytes: &Vec<u8>) -> uint {
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

fn transpose(bytes: &Vec<u8>, n: uint) -> Vec<Vec<u8>> {
  let mut out: Vec<Vec<u8>> = Vec::new();
  for i in range(0u, n) {
    out.push(bytes.iter() // get an iterator
               .enumerate() // get (position, value) paris
               .filter_map(|(idx, val)| { if idx % n == i { Some(*val) } else { None } }) // filter and map based on index
               .collect()); // collect the results into a new vector
  }
  return out;
}

fn english_freq(c: &char) -> f32 {
  match *c {
    'E' => 0.1202,
    'T' => 0.0910,
    'A' => 0.0812,
    'O' => 0.0768,
    'I' => 0.0731,
    'N' => 0.0695,
    'S' => 0.0628,
    'R' => 0.0602,
    'H' => 0.0592,
    'D' => 0.0432,
    'L' => 0.0398,
    'U' => 0.0288,
    'C' => 0.0271,
    'M' => 0.0261,
    'F' => 0.0230,
    'Y' => 0.0211,
    'W' => 0.0209,
    'G' => 0.0203,
    'P' => 0.0182,
    'B' => 0.0149,
    'V' => 0.0111,
    'K' => 0.0069,
    'X' => 0.0017,
    'Q' => 0.0011,
    'J' => 0.0010,
    'Z' => 0.0007,
    _ => 0.0
  }
}

fn score_string(bytes: Vec<u8>) -> f32 {
  // try converting to string
  let try = String::from_utf8(bytes.clone());
  if try.is_err() {
    return 0.0;
  }

  let string: Vec<u8> = match try.unwrap().into_ascii_opt() {
    Some(x) => String::from_utf8(bytes).unwrap().into_ascii_upper().as_bytes().to_vec(),
    None => vec!(0)
  };
  let english_chars = ['E', 'T', 'A', 'O', 'I', 'N', 'S', 'R', 'H',
                       'D', 'L', 'U', 'C', 'M', 'F', 'Y', 'W', 'G',
                       'P', 'B', 'V', 'K', 'X', 'Q', 'J', 'Z'];

  // count how many of each character we got
  let (char_count, total): (SmallIntMap<uint>, uint) =
      string.iter().fold((SmallIntMap::new(), 0u), |(mut m, s), c| {
        m.update(*c as uint, 1, |mut old, _| { old += 1; old });
        let t = s + 1;
        (m, t)
      });

  let mut score: f32 = 0.0;
  for c in english_chars.iter() {
    let count = *char_count.find(&(*c as uint)).unwrap_or(&0u);
    unsafe {
      score += std::intrinsics::sqrtf32(english_freq(c) * (count as f32 / total as f32));
    }
  }

  return score;
}

fn break_xor(bytes: &Vec<u8>) -> u8 {
  let mut best_score: f32 = 0.0;
  let mut best_key: u8 = 0;
  for i in range(1u8, 255u8) {
    let xored = bytes.iter().map(|c| c ^ i).collect();
    let score = score_string(xored);
    if score > best_score {
      best_score = score;
      best_key = i;
    }
  }

  return best_key;
}

fn decrypt(bytes: Vec<u8>, secret: Vec<u8>) -> Vec<u8> {
  let decrypted: Vec<u8>  = bytes.iter()
                       .enumerate()
                       .map(|(idx, byte)| { byte ^ secret[idx % secret.len()] })
                       .collect();
  return decrypted;
}

fn test() {
  // check that we didn't break the hamming distance
  assert!(hamming_distance("this is a test".as_bytes(), "wokka wokka!!!".as_bytes()) == 37);

  // check that we can find the key for a known dataset
  let input: Vec<u8> = [0x0b, 0x36, 0x37, 0x27, 0x2a, 0x2b, 0x2e, 0x63, 0x62, 0x2c, 0x2e, 0x69,
                      0x69, 0x2a, 0x23, 0x69, 0x3a, 0x2a, 0x3c, 0x63, 0x24, 0x20, 0x2d, 0x62,
                      0x3d, 0x63, 0x34, 0x3c, 0x2a, 0x26, 0x22, 0x63, 0x24, 0x27, 0x27, 0x65,
                      0x27, 0x2a, 0x28, 0x2b, 0x2f, 0x20, 0x43, 0x0a, 0x65, 0x2e, 0x2c, 0x65,
                      0x2a, 0x31, 0x24, 0x33, 0x3a, 0x65, 0x3e, 0x2b, 0x20, 0x27, 0x63, 0x0c,
                      0x69, 0x2b, 0x20, 0x28, 0x31, 0x65, 0x28, 0x63, 0x26, 0x30, 0x2e, 0x27,
                      0x28, 0x2f].to_vec();
  assert!(find_keysize(&input) == 3);

  // Check that we can break it
  let transposed = transpose(&input, 3);
  let mut key: Vec<u8> = Vec::new();
  for block in transposed.iter() {
    key.push(break_xor(block));
  }
  println!("KEY: {}", String::from_utf8(key).unwrap());
//   assert!(key == "ICE".as_bytes().to_vec());

  // check that we can decrypt it fine
  let decrypted = decrypt(input, "ICE".as_bytes().to_vec());
  assert!(String::from_utf8(decrypted).unwrap() ==
          String::from_str("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal"));

  // all tests passed
  println!("Passed tests");
}

fn main() {
  test();
  let args: Vec<String> = std::os::args();
  let path = Path::new(args[1].clone());
  let mut file = BufferedReader::new(File::open(&path));
  let input: Vec<u8> = file.read_to_end().unwrap();
  let bytes = input.as_slice().from_base64().unwrap();
  let keysize = find_keysize(&bytes);
  let transposed = transpose(&bytes, keysize);
  let mut key: Vec<u8> = Vec::new();
  for block in transposed.iter() {
    key.push(break_xor(block));
  }
  let decrypted = decrypt(bytes, key);
  println!("{}", String::from_utf8(decrypted).unwrap());
}


