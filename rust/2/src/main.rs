use std::env;
use std::u8;

fn main() {
  let args: Vec<String> = env::args().collect();
  let input = &args[1];
  let xor = &args[2];

  let num_bytes = input.len() / 2;
  let mut output: Vec<u8> = Vec::new();

  for i in 0 .. num_bytes {
    let input_byte = u8::from_str_radix(&input[2 * i .. 2 * i + 2], 16).unwrap();
    let xor_byte = u8::from_str_radix(&xor[2 * i .. 2 * i + 2], 16).unwrap();
    output.push(input_byte ^ xor_byte);
  }

  for byte in output.iter() {
    print!("{:02x}", byte);
  }
  print!("\n");
}
