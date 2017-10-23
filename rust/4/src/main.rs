use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;

fn hex_to_bytes(s: &String) -> Vec<u8> {
  let mut bytes = Vec::new();
  for i in 0 .. (s.len() / 2) {
    let byte = u8::from_str_radix(&s[2*i .. 2*i+2], 16).unwrap();
    bytes.push(byte);
  }
  return bytes;
}

fn score_char(c: char) -> i32 {
  // this is pretty naive
  match c {
    'e' => 12,
    't' => 11,
    'a' => 10,
    'o' => 9,
    'i' => 8,
    'n' => 7,
    's' => 6,
    'h' => 5,
    'r' => 4,
    'd' => 3,
    'l' => 2,
    'u' => 1,
    '\n' => 0,
    x if x > 'z' => -10,
    x if x < ' ' => -10,
    _ => 0,
  }
}

fn decrypt_string(s: String) -> (String, i32) {
  let hex_bytes: Vec<u8> = hex_to_bytes(&s);

  let mut best_string: String = String::new();
  let mut best_score: i32 = 0;
  for i in 1u8 .. 255u8 {
    let decoded: Vec<u8> = hex_bytes.iter().map(|c| c ^ i).collect();
    let score = decoded.iter().fold(0, |s, &c| score_char(c as char) + s);
    let decoded_string = String::from_utf8(decoded);
    if decoded_string.is_ok() && score > best_score {
      best_string = decoded_string.unwrap();
      best_score = score;
    }
  }

  return (best_string, best_score)
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let file = BufReader::new(File::open(&args[1]).unwrap());

  let mut best_string: String = String::new();
  let mut best_score: i32 = 0;
  for line_iter in file.lines() {
    let line = line_iter.unwrap();
    let (string, score) = decrypt_string(line);
    if score > best_score {
      best_string = string;
      best_score = score;
    }
  }

  println!("{} (score: {})", best_string, best_score);
}