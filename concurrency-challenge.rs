extern mod extra;

use extra::future::Future;

fn main() {
  let mut p1 = do Future::spawn { (|a: int| a * 2)(10) };
  let mut p2 = do Future::spawn { (|a: int| a*2)(20) };
  let mut p3 = do Future::spawn { (|a: int, b: int| a + b )(30, 40)};

  let (x, y, z) = (p1.get(), p2.get(), p3.get());
  println!("{:d} + {:d} + {:d} = {:d}", x, y, z, x + y + z);
}
