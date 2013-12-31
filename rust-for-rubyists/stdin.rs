use std::io;
use std::io::buffered::BufferedReader;

fn main() {
  println("INPUT:");
  let mut stdin = BufferedReader::new(io::stdin());
  for line in stdin.lines() {
    println("YOU TYPED:");
    print(line);
  }
}