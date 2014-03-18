/**
 * fibonacci rust version
 */

use fibonacci::fib;
use fibonacci::fibs;

mod fibonacci {
  use std::vec_ng::Vec;

  pub fn fib(i: int) -> int {
    if i==0 || i ==1 { i }
    else { fib(i-1) + fib(i-2) }
  }

  pub fn fibs(max: int) -> Vec<int> {
    let mut ret = Vec::new();
    for i in range(0, max) {
      ret.push(fib(i));
    }
    ret
  }
}

fn main() {
  let fs = fibs(20);
  for (i, v) in fs.iter().enumerate() {
   println!("fib({:u})={:d}", i, *v);
  }
  assert_eq!(4181, fib(19));
}

#[cfg(test)]
mod testutil {
  pub struct Td<A, B> {
    input: A, 
    expected: B
  }

  impl<A, B> Td<A, B> {
      pub fn new(input: A, expected: B) -> Td<A, B> {
        Td{input: input, expected: expected}
      }
  }
}

#[cfg(test)]
mod tests {
  extern crate test;

  use fibonacci::fib;
  use Td = testutil::Td;

  #[test]
  #[ignore(cfg(target_os = "win32"))]
  fn test_fib() {
    let data = [
    Td::new(0, 0), 
    Td::new(1, 1), 
    Td::new(2, 1),
    Td::new(3, 2),
    Td::new(19, 4181)];

    for it in data.iter() {
      assert_eq!(it.expected, fib(it.input));
    }
  }

  #[test]
  #[should_fail]
  fn test_fib2() {
    assert_eq!(1, fib(0));
  }

  #[bench]
  fn bench_it(b: &mut test::BenchHarness) {
    b.iter(|| { fib(30); });
  }
}
