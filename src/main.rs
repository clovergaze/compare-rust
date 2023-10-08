use std::env;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::process;

fn main() -> io::Result<()> {
  let args: Vec<String> = env::args().collect();
  if args.len() < 3 {
    println!("Syntax: compare <file1> <file2>");
    process::exit(0);
  }

  let filename1 = &args[1];
  let mut file1 = File::open(filename1).unwrap_or_else(|_| {
    println!("Error opening file \"{}\".", filename1);
    process::exit(1);
  });

  let filesize1 = fsize(&mut file1)?;

  let filename2 = &args[2];
  let mut file2 = File::open(filename2).unwrap_or_else(|_| {
    println!("Error opening file \"{}\".", filename2);
    process::exit(1);
  });

  let filesize2 = fsize(&mut file2)?;

  if filesize1 != filesize2 {
    println!("Files are not identical, the size is unequal.");
    process::exit(0);
  }

  let mut buffer1 = vec![0u8; filesize1 as usize];

  file1.read_exact(&mut buffer1).unwrap_or_else(|_| {
    println!("Error reading file \"{}\".", filename1);
    process::exit(1);
  });

  let mut buffer2 = vec![0u8; filesize2 as usize];

  file2.read_exact(&mut buffer2).unwrap_or_else(|_| {
    println!("Error reading file \"{}\".", filename2);
    process::exit(1);
  });

  for i in 0..filesize1 {
    if buffer1[i as usize] != buffer2[i as usize] {
      println!("Files are not identical.");
      process::exit(0);
    }
  }

  println!("Files are identical.");
  Ok(())
}

fn fsize(file: &mut File) -> io::Result<u64> {
  let size = file.seek(SeekFrom::End(0))?;
  file.seek(SeekFrom::Start(0))?;
  Ok(size)
}
