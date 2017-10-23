use std::env;
use std::u8;

fn hex_to_bytes(s: &String) -> Vec<u8> {
  let mut bytes = Vec::new();
  for i in 0 .. (s.len() / 2) {
    let byte = u8::from_str_radix(&s[2*i .. 2*i+2], 16).unwrap();
    bytes.push(byte);
  }
  return bytes;
}

fn score_for_char(c: char) -> i32 {
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
    x if x > 'z' => -99,
    x if x < ' ' => -99,
    _ => 0,
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let hex_string = &args[1];
  let bytes = hex_to_bytes(hex_string);

  let mut best_string: String = String::new();
  let mut best_score: i32 = 0;
  for i in 1u8 .. 255u8 {
    let decoded: Vec<u8> = bytes.iter().map(|c| c ^ i).collect();
    let score = decoded.iter().fold(0, |s, &c| score_for_char(c as char) + s);
    let decoded_string = String::from_utf8(decoded);
    if decoded_string.is_ok() && score > best_score {
      best_string = decoded_string.unwrap();
      best_score = score;
    }
  }

  println!("{} (score: {})", best_string, best_score);
}