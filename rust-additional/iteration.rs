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

  iterator_adaptors();
}

fn iterator_adaptors() {
  let xs = [1, 9, 2, 3, 14, 12];
  let result = xs.iter().fold(0, |a, i|{
    a - *i
  });
  assert_eq!(result, -41);

  let ys = [5,2,1,8];
  let sum =xs.iter().chain(ys.iter()).fold(0, |a, b| a + *b);
  assert_eq!(sum, 57);

  {
    let xs = [1,2,3,4,5];
    let ys = ["foo", "bar", "baz", "foobar"];
    let mut it = xs.iter().zip(ys.iter());
    for (x, y) in it {
      println!("{} {}", *x, *y);
      if *x == 3 {
        break;
      }
    }
    println!("last: {:?}", it.next());
    // the iterator is now fully consumed
    assert!(it.next().is_none());   
   };

   (||{
     println("## Conversion");
     let xs = [0,1,1,2,3,5,8];
     let ys = xs.rev_iter().skip(1).map(|&x| x * 2).collect::<~[int]>();
     assert_eq!(ys, ~[10,6,4,2,2,0]);
   })();

   (||{
     println("## Double-ended iterators");
     let xs = [1,2,3,4,5,6];
     let mut it = xs.iter();
     println!("{:?}", it.next());
     println!("{:?}", it.next_back()); //
     println!("{:?}", it.next_back()); 

     for &x in it.invert() {
       println!("{}", x);
     }
   })();

   (||{
    println("## reverse_");
    let mut ys = [1,2,3,4,5];
    ys.mut_iter().reverse_();
    assert_eq!(ys, [5,4,3,2,1]);
   })();

  (||{
    println("## Random-access iterators");
    let xs = [1,2,3,4,5];
    let ys = ~[7,9,11];
    let it = xs.iter().chain(ys.iter());
    println!("{:?}", it.idx(0));
    println!("{:?}", it.idx(7));
    println!("{:?}", it.idx(8));

    println!("xs[0]:{:d}", xs[0]);
    let mut i =0;
    let max = xs.len();
    while i < max {
      println!("{:d}", xs[i]);
      i += 1;
    }

    println("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
    for x in range(0, xs.len()) {
      println!("{:d}", xs[x]);
    }
  })();
}