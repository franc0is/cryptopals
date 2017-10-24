#![feature(io)]

extern crate base64;
extern crate itertools;
extern crate openssl;

use itertools::Itertools;
use openssl::symm::*;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() != 3 {
    println!("Please provide 2 args: the file and the block size");
    return;
  }

  // get file and blocksize arguments
  let file = BufReader::new(File::open(&args[1]).unwrap());
  let block_size = usize::from_str_radix(&args[2], 10).unwrap();


  // create a lazy iterator that yields 4 base64 characters at a time
  // from the file. This allows us to convert them into 3 bytes
  let base64_chunks = file.chars()
                          .map(|c| c.unwrap())
                          .filter(|&c| c != '\n')
                          .chunks(4);

  // decode and decrypt
  let mut previous_block :Vec<u8> = vec![0; block_size]; // initialized to IV: all 0s
  let mut current_block : Vec<u8> = Vec::new();
  let mut leftover_bytes : Vec<u8> = Vec::new();

  for chunk in &base64_chunks {
    let s : String = chunk.collect::<Vec<char>>().into_iter().collect();
    let mut bytes = base64::decode(&s).unwrap();
    let bytes_needed = block_size - current_block.len();

    if bytes_needed == 0 {
        // we've got a full block, crunch the numbers!
        println!("{:?}", current_block);
        previous_block = current_block;
        current_block = leftover_bytes;
        leftover_bytes = Vec::new();
    } else if bytes_needed < bytes.len() {
        leftover_bytes.extend(bytes.drain(bytes_needed..));
    }

    current_block.append(&mut bytes);
  }


  let key: &[u8] = "YELLOW SUBMARINE".as_bytes();
  //let iv: &[u8] = [0; 3];

  //let decrypted = decrypt(Cipher::aes_128_ecb(), key, None, eight_bytes).unwrap();
  //println!("{}", String::from_utf8(decrypted).unwrap());
}
