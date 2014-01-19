extern mod extra;

use extra::future::Future;

fn main() {
  version1();

  version2();
}

fn version1() {
  let mut p1 = do Future::spawn { (|a: int| a * 2)(10) };
  let mut p2 = do Future::spawn { (|a: int| a * 2)(20) };
  let mut p3 = do Future::spawn { (|a: int, b: int| a + b )(30, 40) };

  let (x, y, z) = (p1.get(), p2.get(), p3.get());
  println!("{:d} + {:d} + {:d} = {:d}", x, y, z, x + y + z);
}

fn version2() {
  let mut v = ~[do Future::spawn { (|a: int| a * 2)(10) },
                do Future::spawn { (|a: int| a * 2)(20) },
                do Future::spawn { (|a: int, b: int| a + b )(30, 40) }];
  let ret = v.mut_iter().map(|x| x.get()).collect::<~[int]>();
  println!("{:d} + {:d} + {:d} = {:d}", ret[0], ret[1], ret[2], ret.iter().fold(0, |a, &f| a + f));
}
