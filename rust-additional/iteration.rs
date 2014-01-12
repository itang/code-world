/// An infinite stream of zeroes
struct InfiniteZeroStream;

impl Iterator<int> for InfiniteZeroStream {
  fn next(&mut self) -> Option<int> {
    Some(0)
  }
}

struct ZeroStream {
  priv remaining: uint
}

impl ZeroStream {
  fn new(n: uint) -> ZeroStream {
    ZeroStream { remaining: n }
  }
}

impl Iterator<int> for ZeroStream {
  fn next(&mut self) -> Option<int> {
    if self.remaining == 0 {
      None
    } else {
      self.remaining -= 1;
      Some(0)
    }
  }
}

fn main() {
  iteration_protocol();
  container_iterators();
}

fn iteration_protocol() {
  let mut j = 0;
  let mut z = InfiniteZeroStream;
  for i in z {
    print!("{:d},", i);
    if j > 100 {
      break;
    }
    j += 1;
  }

  println("");

  let mut zstream = ZeroStream::new(10);
  j = 0;
  for i in zstream {
    println!("{}, {}", j, i);
    j += 1;
  }
}

fn container_iterators() {
  let v = [1,2,3];
  for i in v.iter() {
    println!("i: {:?}, *i: {:d}",i, *i);
  }

  for &i in v.iter() {
    println!("&i: {:d}",i);
  }

  /*
  for i in v { // type `&mut [<VI8>, .. 3]` does not implement any method in scope named `next`
    println!("i: {:d}",i);
  }
  */

  let v2 = &[3,4,5];
  for i in v2.iter() {
    println!("i: {:d}", *i);
  }

  println("unique vector iter:");
  let v3 = ~[6,7,8];
  for i in v3.iter() {
    print!("{:d}", *i);
  }
  println("rev_iter:");
  for i in v3.rev_iter() {
    print!("{:d}", *i);
  }
  println("");

  let v4 = &mut[1,2,4];
  println("mut_iter:");
  for i in v4.mut_iter() {
    *i = 10;
  }
  println!("{:?}", v4);

  println("move_iter:");
  let v5 = ~[~"a", ~"b"];
  for i in v5.move_iter() {
    print!("{:s},", i);
  }
  println("");
}
