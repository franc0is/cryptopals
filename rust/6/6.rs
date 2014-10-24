extern crate serialize;
use serialize::base64::FromBase64;
use std::io::File;
use std::io::BufferedReader;

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

  // compare all combinations of the first 9 samples
  for keysize in range(8u, 60) {
    if 4 * keysize > bytes.len() {
      break;
    }

    let s1 = bytes.slice(0 * keysize, 1 * keysize - 1);
    let s2 = bytes.slice(1 * keysize, 2 * keysize - 1);
    let s3 = bytes.slice(2 * keysize, 3 * keysize - 1);
    let s4 = bytes.slice(3 * keysize, 4 * keysize - 1);
    let s5 = bytes.slice(4 * keysize, 5 * keysize - 1);
    let s6 = bytes.slice(5 * keysize, 6 * keysize - 1);
    let s7 = bytes.slice(6 * keysize, 7 * keysize - 1);
    let s8 = bytes.slice(7 * keysize, 8 * keysize - 1);
    let s9 = bytes.slice(8 * keysize, 9 * keysize - 1);
    let score: f32  = (hamming_distance(s1, s2) +
                       hamming_distance(s1, s3) +
                       hamming_distance(s1, s4) +
                       hamming_distance(s1, s5) +
                       hamming_distance(s1, s6) +
                       hamming_distance(s1, s7) +
                       hamming_distance(s1, s8) +
                       hamming_distance(s1, s9) +
                       hamming_distance(s2, s3) +
                       hamming_distance(s2, s4) +
                       hamming_distance(s2, s5) +
                       hamming_distance(s2, s6) +
                       hamming_distance(s2, s7) +
                       hamming_distance(s2, s8) +
                       hamming_distance(s2, s9) +
                       hamming_distance(s3, s4) +
                       hamming_distance(s3, s5) +
                       hamming_distance(s3, s6) +
                       hamming_distance(s3, s7) +
                       hamming_distance(s3, s8) +
                       hamming_distance(s3, s9) +
                       hamming_distance(s4, s5) +
                       hamming_distance(s4, s6) +
                       hamming_distance(s4, s7) +
                       hamming_distance(s4, s8) +
                       hamming_distance(s4, s9) +
                       hamming_distance(s5, s6) +
                       hamming_distance(s5, s7) +
                       hamming_distance(s5, s8) +
                       hamming_distance(s5, s9) +
                       hamming_distance(s6, s7) +
                       hamming_distance(s6, s8) +
                       hamming_distance(s6, s9) +
                       hamming_distance(s7, s8) +
                       hamming_distance(s7, s9) +
                       hamming_distance(s8, s9))
                       as f32 / 36.0 / keysize as f32;
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

fn score_for_char(c: char) -> int {
  // this is pretty naive
  match c {
    'e' => 20,
    't' => 19,
    'a' => 18,
    'o' => 17,
    'i' => 16,
    'n' => 15,
    's' => 14,
    'h' => 13,
    'r' => 12,
    'd' => 11,
    'l' => 10,
    'u' => 9,
    x if (x > 'z' || x < ' ') => -30,
    _ => 0
  }
}

fn break_xor(bytes: &Vec<u8>) -> u8 {
  let mut best_score: int = 0;
  let mut best_key: u8 = 0;
  for i in range(1u8, 255u8) {
    let score = bytes.iter()
                     .fold(0, |s, c| score_for_char((c ^ i) as char) + s);
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

fn main() {
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

