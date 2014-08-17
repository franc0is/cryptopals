
fn main() {
  let args: Vec<String> = std::os::args();
  let input_bytes = args[1].as_bytes();
  let xor_bytes = args[2].as_bytes();

  let num_bytes = input_bytes.len();
  let mut i = 0;
  let mut xored_bytes: Vec<u8> = Vec::new();
  while i < num_bytes {
    let input_byte = std::int::parse_bytes(&[input_bytes[i]], 16).unwrap() as u8;
    let xor_byte = std::int::parse_bytes(&[xor_bytes[i]], 16).unwrap() as u8;
    xored_bytes.push(input_byte ^ xor_byte);
    i = i + 1;
  }

  for byte in xored_bytes.iter() {
    print!("{:x}", byte.clone());
  }
  print!("\n");
}
