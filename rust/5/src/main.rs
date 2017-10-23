use std::env;

fn print_hex(string: Vec<u8>) {
  for byte in string.iter() {
    print!("{:02x}", byte.clone());
  }
  print!("\n");
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let input: Vec<u8> = args[1].as_bytes().to_vec();
  let secret: Vec<u8> = args[2].as_bytes().to_vec();

  let decrypted = input.iter()
                       .enumerate()
                       .map(|(idx, byte)| { byte ^ secret[idx % secret.len()] })
                       .collect();

  print_hex(decrypted);

}
