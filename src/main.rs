use std::env;
use std::fs::File;
use std::io::{Read, Result};
use std::process;

fn main() -> Result<()> {
  let args: Vec<String> = env::args().collect();
  if args.len() < 3 {
    println!("Syntax: compare <file1> <file2>");
    process::exit(0);
  }

  let filename1 = &args[1];
  let (mut file1, filesize1) = open_file(filename1)?;

  let filename2 = &args[2];
  let (mut file2, filesize2) = open_file(filename2)?;

  if filesize1 != filesize2 {
    println!("Files are not identical, the size is unequal.");
    process::exit(0);
  }

  let buffer1 = read_file(&mut file1, filesize1)?;
  let buffer2 = read_file(&mut file2, filesize2)?;

  if buffer1 != buffer2 {
    println!("Files are not identical.");
    process::exit(0);
  }

  println!("Files are identical.");
  Ok(())
}

fn open_file(filename: &str) -> Result<(File, u64)> {
  let file = File::open(filename).unwrap_or_else(|_| {
    println!("Error opening file \"{}\".", filename);
    process::exit(1);
  });
  let filesize = file.metadata()?.len();
  Ok((file, filesize))
}

fn read_file(file: &mut File, filesize: u64) -> Result<Vec<u8>> {
  let mut buffer = vec![0u8; filesize as usize];
  file.read_exact(&mut buffer).unwrap_or_else(|_| {
    println!("Error reading file.");
    process::exit(1);
  });
  Ok(buffer)
}
