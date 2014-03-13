extern crate rand;

use rand::{task_rng, Rng};
use std::io::println;

fn main() {
  let n = rand::random::<int>();

  println(n.to_str());

  let mut rng = task_rng();
  let n: uint = rng.gen_range(0u, 10);
  println!("{}", n);
  let m: f64 = rng.gen_range(-40.0, 1.3e5);
  println!("{}", m);

  let r = rand::task_rng().gen_range(0, 100);
  println(r.to_str());
}