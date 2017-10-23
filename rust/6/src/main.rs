extern crate base64;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn hamming_distance(s1: &[u8], s2: &[u8]) -> u32 {
  let mut distance: u32 = 0;
  let mut it1 = s1.iter();
  let mut it2 = s2.iter();
  loop {
    match (it1.next(), it2.next()) {
      (None, None) => break,
      (None, Some(x)) | (Some(x), None) => distance += x.count_ones() as u32,
      (Some(x), Some(y)) => distance += (*x ^ *y).count_ones() as u32
    }
  }

  return distance;
}

fn find_keysize(bytes: &Vec<u8>) -> u32 {
  let mut best_score: f32 = 99999.9;
  let mut best_size: u32 = 0;

  // compare all combinations of the first 9 samples
  for keysize in 8u32 .. 60 {
    if 4 * keysize > bytes.len() as u32 {
      break;
    }

    let mut chunks = Vec::new();
    for i in 0 .. 8 {
        let chunk = &bytes[i * keysize as usize .. (i + 1) * keysize as usize - 1];
        chunks.push(chunk);
    }

    let mut score = 0.0;
    let mut num_combos = 0;

    for (i, c1) in chunks.iter().enumerate() {
        for c2 in chunks[i..].iter() {
            score += hamming_distance(c1, c2) as f32;
            num_combos += 1;
        }
    }

    score = score / num_combos as f32 / keysize as f32;

    if score < best_score {
      // smaller is better
      best_score = score;
      best_size = keysize;
    }
  }

  return best_size as u32;
}

fn transpose(bytes: &Vec<u8>, n: u32) -> Vec<Vec<u8>> {
  let mut out: Vec<Vec<u8>> = Vec::new();
  for i in 0 .. n {
    out.push(bytes.iter() // get an iterator
               .enumerate() // get (position, value) paris
               .filter_map(|(idx, val)| { if idx as u32 % n == i { Some(*val) } else { None } }) // filter and map based on index
               .collect()); // collect the results into a new vector
  }
  return out;
}

fn score_for_char(c: char) -> i32 {
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
  let mut best_score: i32 = 0;
  let mut best_key: u8 = 0;
  for i in 1u8 .. 255u8 {
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
  let args: Vec<String> = env::args().collect();
  let mut file = BufReader::new(File::open(&args[1]).unwrap());
  let mut input: Vec<u8> = Vec::new();
  file.read_to_end(&mut input).unwrap();
  // remove line breaks
  input.retain(|b| *b != '\n' as u8);
  let bytes = base64::decode(&input).unwrap();
  let keysize = find_keysize(&bytes);
  let transposed = transpose(&bytes, keysize);
  let mut key: Vec<u8> = Vec::new();
  for block in transposed.iter() {
    key.push(break_xor(block));
  }
  let decrypted = decrypt(bytes, key);
  println!("{}", String::from_utf8(decrypted).unwrap());
}

