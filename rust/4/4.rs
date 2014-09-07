use std::io::File;
use std::io::BufferedReader;

fn chunk_to_u8(chunk: &[u8]) -> u8 {
  let parsed = std::int::parse_bytes(chunk, 16);
  let result = parsed.unwrap_or(0) as u8;
  return result;
}

fn score_char(c: char) -> int {
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
    x if x > 'z' => -99,
    x if x < ' ' => -99,
    _ => 0,
  }
}

fn decrypt_string(s: String) -> (String, int) {
  // 2 digits / byte
  let hex_string = s.as_bytes();
  let hex_chunks = hex_string.chunks(2);
  let hex_bytes: Vec<u8> = hex_chunks.map(|chunk| chunk_to_u8(chunk)).collect();

  let mut best_string: String = String::new();
  let mut best_score: int = 0;
  for i in range(1u8, 255u8) {
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
  let args: Vec<String> = std::os::args();
  let path = Path::new(args[1].clone());
  let mut file = BufferedReader::new(File::open(&path));

  let mut best_string: String = String::new();
  let mut best_score: int = 0;
  for line_iter in file.lines() {
    let line: String = match line_iter { Ok(x) => x, Err(e) => fail!(e) };
    let (string, score) = decrypt_string(line.clone());
    if score > best_score {
      best_string = string;
      best_score = score;
    }
  }

  println!("{} (score: {})", best_string, best_score);
}