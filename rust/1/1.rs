fn base64_digit_to_char(digit: u8) -> char {
  let ascii_code: u8 = match digit {
    digit if digit < 26 => 65 + digit,
    digit if digit < 52 => 97 + digit - 26,
    _ => 48 + digit - 52
  } as u8;
  return ascii_code as char;
}

fn main() {
  // read from args
  let args: Vec<String> = std::os::args();
  let hex_string = args.get(1).as_bytes();

  // divide hex in chunks of 3 digits, which correspond to 2 base64 digits
  let hex_chunks = hex_string.chunks(3);

  let mut base64_digits: Vec<u8> = Vec::new();
  for hex_chunk in hex_chunks.clone() {
    let num = std::int::parse_bytes(hex_chunk, 16);
    base64_digits.push((num.unwrap() / 64) as u8);
    base64_digits.push((num.unwrap() % 64) as u8);
  }

  for digit in base64_digits.iter() {
    print!("{}", base64_digit_to_char(digit.clone()));
  }
  print!("\n");
}