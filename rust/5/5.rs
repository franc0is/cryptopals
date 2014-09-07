fn print_hex(string: Vec<u8>) {
  for byte in string.iter() {
    print!("{:02x}", byte.clone());
  }
  print!("\n");
}

fn main() {
  let args: Vec<String> = std::os::args();
  let input = args[1].as_bytes();
  let secret = args[2].as_bytes();
  let mut xored_bytes: Vec<u8> = Vec::new();

  let mut secret_idx: uint = 0;
  for byte in input.iter() {
    xored_bytes.push(byte ^ secret[secret_idx]);
    secret_idx = (secret_idx + 1) % secret.len();
  }

  print_hex(xored_bytes);

}
