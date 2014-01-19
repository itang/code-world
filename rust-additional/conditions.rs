#[allow(unused_imports)];
#[allow(unreachable_code)];
#[allow(unused_variable)];
#[allow(unused_mut)];

extern mod extra;
use std::io::BufferedReader;
use std::io::File;
use std::task;
use std::io::println;

mod BufferedReader {
  use std::io::File;
  use std::io::MemReader;
  use std::io::BufferedReader;
  static s: &'static [u8] = bytes!("1 2\n\
    34 56\n\
    789 123\n\
    45 67\n\
    ");
  pub fn new(_inner: Option<File>) -> BufferedReader<MemReader> {
    BufferedReader::new(MemReader::new(s.to_owned()))
  }
}

fn main() {
  common(&"Option<T>", read_int_pairs);

  do spawn {
    println("task1: 1");
    fail!("throw task error!");
    println("task2: 2"); // warning: unreachable statement, #[warn(unreachable_code)] on by default
  }

  println("main task");
  let mut x = 0;
  5.times(|| {
    println!("x: {:?}", x);
    x += 1;
  });

  // Isolate failture withina subtask.
  let result = do task::try {
    common(&"fail-try", read_int_pairs_try);
  };
  if result.is_err() {
    println("parsing failed");
  }

  let sv = [1,2,3];
  let mut msv = [1,2,3];

  let ov = ~[1,2,3];
  let mut mov = ~[1,2,3];

  let bv = &[1,2,3];
  do spawn {
    println!("sv:{:?}", sv); // copy
    //println!("msv:{:?}", msv); // error: mutable variables cannot be implicitly captured
    println!("ov:{:?}", ov); // move
    // println!("mov:{:?}", mov); // error: mutable variables cannot be implicitly captured
    //println!("bv:{:?}", bv); //error: cannot capture variable of type `&[int]`, which does not fulfill `Send`, in a bounded closure
  }
}

fn common(msg: &str, c: || -> ~[(int, int)]) {
  println!("******** {:s}", msg);
  let pairs =c();
  for &(a, b) in pairs.iter() {
    println!("{:4.4d}, {:4.4d}", a, b);
  }
}

fn read_int_pairs() -> ~[(int,int)] {
  let mut pairs = ~[];

  //Path takes a generic by-value, rather than by reference
  let _g = std::io::ignore_io_error();
  let path = Path::new(&"data/numbers.txt");
  let mut reader = BufferedReader::new(File::open(&path));

  //1. Iterate over the lines of our file.
  for line in reader.lines() {
    //2. Split the line into fields ("world").
    let fields = line.words().to_owned_vec();
    //3. Match the vector of fields against a vector pattern.
    match fields {
      //4. When the line had two fields:
      [a, b] => {
        // 5. Try parsing both fields as ints.
        match (from_str::<int>(a), from_str::<int>(b)) {
          //6. If parsing successed for both, push both.
          (Some(a), Some(b)) => pairs.push((a, b)),
          _ => ()
        }
      }
      //8. Ignore lines that don't have 2 fields.
      _ => ()
    }
  }

  pairs
}

fn read_int_pairs_try() -> ~[(int, int)]{
  let mut pairs = ~[];
  let _g = std::io::ignore_io_error();
  let path = Path::new(&"data/numbers.txt");

  let mut reader = BufferedReader::new(File::open(&path));
  for line in reader.lines() {
    match line.words().to_owned_vec() {
      [a, b] => pairs.push((from_str::<int>(a).unwrap(),
        from_str::<int>(b).unwrap())),
      _ => fail!()
    }
  }
  pairs
}