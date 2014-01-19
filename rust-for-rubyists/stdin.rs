use std::io;
use std::io::BufferedReader;
use std::io::{print, println};

fn main() {
  println("INPUT:");
  let mut stdin = BufferedReader::new(io::stdin());
  for line in stdin.lines() {
    println("YOU TYPED:");
    print(line);
  }
}