#![feature(managed_boxes)]
#![allow(deprecated_owned_vector)]
#![feature(phase)]

extern crate collections;
extern crate rand;
#[phase(syntax, link)]
extern crate log;

use collections::HashMap;

use std::f64;
use std::mem::size_of;
use std::rc::Rc;
use std::gc::Gc;
use rand::random;
use std::task::spawn;
use puts = std::io::println;

fn main() {
  tprintln();
  t1();
  tlet();
  tstatic();
  identifiers_named();
  expr_semicolons();
  primitive_types_literals();
  operations();
  syntax_extensions();
  control_structures();
  data_structures_struct();
  data_structures_enum();
  tuples();
  functions();
  destructors();
  ownership();
  boxes();
  references();
  test_list_g();
  boxes_more();
  borrowed_pointers();
  freezing();
  dereferenced();
  vectors();
  strings();
  ownership_escape_hatches();

  closures();
  do_syntax();
  methods();
  generics();

  traits();
}

//////////////////////
fn tprintln() {
  print!("{}", "hello, ")   // => hello, 
  println!("{:?}", "world") // => "world"

  assert_eq!(~"abcdefghi", format!("{}{}{}","abc","def","ghi"));

  //rust-0.9 The `concat!` syntax extension performs compile-time string concatenation.
  // compile-time 
  assert_eq!("abcdefghi", concat!("abc","def","ghi"));
  /*
  static s: &'static str = "hello";
  concat!(s, ", world"); // error: expected a literal
  */
  puts("Hello, World! by puts");
}

////////////////////
mod universe {
  pub fn recalibrate() -> bool {
    true
  }
}

fn t1 () {
  /* A simple loop */
  loop {
    // A tricky calculation 
    if universe::recalibrate() {
      return;
    }
  }
}

////////////////////////////
fn tlet() {
  let hi = "hi";
  assert_eq!("hi", hi);

  let hii: &'static str = "hello";
  assert_eq!("hello", hii);
  assert_eq!(~"hello", format!("{:s}", hii));

  let mut count = 0;
  while count < 3 {
    count += 1;
  }
  assert_eq!(3, count);
}

///////
fn tstatic() {
  static M: f64 = 10.0;    //static item, require a type annotation
  let m = M;
  assert_eq!(10.0, m);

  //let msize = M * 10.0;  // unused variable: `msize` #[warn(unused_variable)]
  let _msize = M * 10.0;   // with underscore, unwarnning
  let msize = 50;
  assert_eq!(50, msize);    // => Local variables may shadow earlier declarations. msize: 50 , not M * 10.0

  //assert_eq!(100.0, _msize);
}

///////
fn identifiers_named() {
  let _a = "a";
  let _a11212_ = 1;
  let abc_123_def =3;
  assert!(_a == "a" &&  _a11212_ == 1 && abc_123_def == 3);
  // "The preferred style is to write function, variable, and module names with lowercase letters, using underscores where they help readability, while writing types in camel case");
  type MyType = int;
  let i: MyType = 10;
  assert!(10 == i);
}

fn expr_semicolons() {
  let item = "salad";
  let price;
  if item == "salad" {
    price = 3.50;
  }else if item == "muffin" {
    price = 2.25;
  }else {
    price = 2.00;
  }
  assert_eq!(3.5, price);

  let price = 
      if item == "salad" { 4.50 }  //  the lack of a semicolon after the last statement in a braced block gives the whole block the value of that last expression.
      else if item == "muffin" {2.25}
      else { 2.00 };
  assert_eq!(4.5, price);
 
  //let price = if item == "salad" { 100.00; } else { 1000 };  //  error: if and else have incompatible types: expected `()` but found `<VI0>` (expected () but found integral variable)
  //println!("new-new price:{:?}", price);

  fn is_four(i: int) -> bool { 
    i == 4
  }
  assert_eq!(true, is_four(4));
}

fn primitive_types_literals() {
  let a = 1; // a is an int 
  let b = 10i; // b is an int, due to the 'i' suffix
  let c = 100u; // c is a uint 
  let d = 10000i32; // d is an i32 
  assert!(a == 1 && b == 10i && c == 100u && d == 10000i32);

  //two floating-point types: f32, and f64
  let e = 1.0;
  let f = 10.0f32;
  let g = 20.0f64;
  assert_eq!(~"11020", format!("{}{}{}", e, f, g));
  assert_eq!(~"1,10,20",format!("{:f},{:f},{:f}", e, f, g));

  //literals of type bool
  let t1 = true;
  let t2 = false;
  assert_eq!(~"true,false", format!("{:b},{:b}", t1, t2));

  //Characters, the char type, are four-byte unicode codepoints
  // whois literals are written between single quotes, as in 'x'.
  let c1 = 'a';
  let c2 = 'A';
  let c3 = '\n';
  let c4 = "abc\tABC";
  assert_eq!(~"CHAR:a,A,\n,abc\tABC:RAHC", format!("CHAR:{},{},{},{}:RAHC", c1, c2, c3, c4));

  let c5 = r##"blah\n##"##;
  let c6 = r"abc\tABC";
  let c7 = r####"hello,world"####;
  assert_eq!(~r"blah\n##, abc\tABC, hello,world", format!("{}, {}, {}", c5, c6, c7));

  // nil type,written (), has a single value, also written ().
  let n: () = (); // primitive_types_literals
  //println!("{}", n); // failed to find an implementation of trait std::fmt::Default for ()
  assert_eq!(~"NIL:():LIN", format!("NIL:{:?}:LIN", n));
  assert_eq!( (), n);

  // type range
  let i :u16 = 65535u16; // pow(2, 16) - 1;
  assert_eq!(~"max_value::<u16> = 65535", format!("max_value::<u16> = {:u}", i));
  assert_eq!(i, std::u16::MAX);
}

fn operations() {
  assert_eq!(3, 1 + 2);
  assert_eq!(-1, 1 - 2);
  assert_eq!(2, 1 * 2);
  assert_eq!(0, 1 / 2);

  assert_eq!(1, 1 % 2);

  let a = -(1000 + 50);
  let b = /*+*/1000;
  assert_eq!(- 1050, a);
  assert_eq!(1000, b);

  let c1 = 1 << 2;
  let c2 = 8 >> 2;
  assert!(c1 == 4 && c2 == 2);

  assert_eq!(2, 2 & 2);
  assert_eq!(0, 2 & 1);
  assert_eq!(3, 2 | 1);
  assert_eq!(1, 1 | 1);

  assert_eq!(0, 1 ^ 1);  // ??

  assert_eq!(-2, !1); // bitwise NOT;

  assert_eq!(true, 1 == 1);
  
  fn f1() -> bool {
    true
  }

  fn f2() -> bool {
    println!("f2");
    false
  }

  assert_eq!(false, f1() && f2());
  assert_eq!(true, f1() || f2());

  // compile-time type casting, use 'as' operator.
  // as is only used with the primitive numberic types or pointers, and is not overloadable.
  // transmute can be used for unsafe C-like casting of sanme-sized types.
  let x: f64 = 4.0;
  let y: uint = x as uint;
  assert!(y == 4u);
}

fn syntax_extensions() {
  let a = format!("{}", 1);
  assert!(a == ~"1");

  assert_eq!(~"the answer is 43",format!("{} is {}", "the answer", 43)); // {} will print the "default format"  
  assert_eq!(~"()", format!("{:?}", ())); // {:?} will conveniently print any type 
}

fn control_structures() {
  // if
  let ret = if false { "that's odd" }
  else if true { "right" }
  else { "neither true nor false" };

  assert_eq!("right", ret);

  fn signum(x: int) -> int {
    if x < 0 { -1 } else if x > 0 { 1 } else { 0 }
  }

  assert!(signum(1) == 1);
  assert!(signum(0) == 0);
  assert!(signum(-100) < 0);

  // pattern matching
  let my_number = 1;
  assert!( "one or two" == match my_number {
    0 => "zero",
    1|2 => "one or two",
    3..10 => "three to ten",
    _ => "something else"
  });

  assert!(match 100 {
    0..100 => true,
    _ => false
  });
  assert!(match 0 {
    0..100 => true,
    _ => false
  });

  fn angle(vector: (f64, f64)) -> f64 {
    let pi = f64::consts::PI;
    match vector {
      (0.0, y) if y < 0.0 => 1.5 * pi,
      (0.0, _) => 0.5 * pi,
      (x, y) => (y/x).atan()
    }
  }
  let r1 = (200.0f64/100.0).atan();
  let r2 = angle((100.0, 200.0));
  assert_eq!(r1, r2);

  fn gettupleoftwoints() -> (int, int) {
    (1, 1)
  }

  let (a, b) = gettupleoftwoints();
  assert!(a == 1)
  assert!(b == 1)

  // Loops
  let mut cakeamount = 8; 
  let mut count = 0;
  while cakeamount > 0 {
    if cakeamount == 2 {
      break;
    }

    count += 1;
    cakeamount -= 1;
    if cakeamount == 5 {
      continue;
    }
  }
  assert!(cakeamount == 2);
  assert!(count == 6);

  let mut x = 5u;
  loop {
    x += x - 3;
    if x % 5 == 0 {
      break;
    }
  }

  println!("size_of::<uint>(): {}", size_of::<uint>());
}

struct Name {
  field1: ~str,
  field2: uint
}

fn data_structures_struct() {
  let n = Name {field1: ~"itang", field2: 100};
  assert_eq!(~r##"Name{field1: ~"itang", field2: 100u}"##,format!("{:?}", n));
  assert_eq!(~"itang", n.field1);
  assert_eq!(~"itang", format!("{:s}", n.field1));

  // n.field1 = ~"tqibm"; // cannot assign to immutable field
  let mut nn = Name { field1: ~"tqibm"
  ,field2: 300};
  assert_eq!(~"tqibm", nn.field1);
  nn.field1 = ~"live.tang";
  assert_eq!(~"live.tang", nn.field1);

  struct Point { x: f64, y: f64 }

  let mypoint = Point {x: 0.0, y: 0.0};
  match mypoint {
    Point {x:0.0, y: yy} => assert_eq!(~"0", yy.to_str()),
    Point {x: xx, y: yy} => assert_eq!(~"0.0 0.0", (xx.to_str() + " " + yy.to_str()))
  }

  match mypoint {
    Point {x, .. } => assert_eq!(~"0", x.to_str())
  }

  let mut m = mypoint;
  m.y = 100.0;
  let c = Point { x: 10.0, ..m }; // "merge copy"
  assert_eq!(10.0, c.x);
  assert_eq!(100.0, c.y);

  struct Pair { x: int, y: int }
  impl Pair {
    fn zeroed_x_copy(self) -> Pair {
      Pair { x: 0, ..self }
    }
    fn replace_x(&mut self, newx: int) { self.x = newx; }
  }
  
  let p1 = Pair { x:1, y:1 };
  let p2 = p1.zeroed_x_copy();
  assert_eq!(0, p2.x);
  assert_eq!(1, p2.y);
  let mut p3 = p2;
  p3.replace_x(100);
  assert_eq!(100, p3.x);
}

fn data_structures_enum() {
  struct Point { x: f64, y: f64 }

  enum Shape {
    Circle(Point, f64),
    Rectangle(Point, Point)
  }

  enum Direction {
    North,
    East,
    South,
    West
  }

  let c: Shape = Circle(Point {x:0.0, y:0.0}, 100.0 );
  assert_eq!(~"Circle(data_structures_enum::Point{x: 0f64, y: 0f64}, 100f64)",format!("{:?}", c));

  match c {
    Circle(p, r) => assert!(p.x == 0.0 && r == 100.0),
    Rectangle(_, _) => fail!("no access here")
  }

  fn area(sh: Shape) -> f64 {
    match sh {
      Circle(_, size) => f64::consts::PI * size * size,
      Rectangle(Point{x, y}, Point{x: x2, y: y2}) => (x2 - x) * (y2 - y)
    }
  }

  assert_eq!(f64::consts::PI * 20.0 * 20.0, area(Circle(Point{x:10.0, y:10.0}, 20.0)));

  let e: Direction = North;
  println!("Direction: {:?}", e);
  match e {
    North => assert!(true),
    _ => fail!("fail")
  }

  //
  enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff
  }
  assert!( match Red { Red => true, _ => false} && 0xff0000 == Red as int);
  assert!( match Blue { Blue => true, _ => false} && 0x0000ff == Blue as int);

  enum Type {
    T1 = 0,
    T2 = 1 << 1,
    T3 = 1 << 2,
    T4 = 1 << 3
  }

  assert!(0 == (T1 as int));
  assert!(2 == (T2 as int));
  assert!(4 == (T3 as int));
  assert!(8 == (T4 as int));

  enum Category {
    G1, G2
  }

  assert_eq!(G1 as int, 0);
  assert_eq!(G2 as int, 1);
}

fn tuples() {
  let mytup: (int, int, f64) = (10, 20, 30.0);
  println!("log_enabled!(std::logging::INFO): {}", log_enabled!(log::INFO));
  match mytup {
    (a, b, c) => info!("sum of {}", a + b + (c as int))
  }

  struct MyTup(int, int, f64);
  let mytup: MyTup = MyTup(10, 20, 30.0);
  match mytup {
    MyTup(a, b, c) => assert_eq!(10 + 20 + 30,  a + b + (c as int))
  }

  let MyTup(_, b, _) = MyTup(20, 40, 100.0);
  assert_eq!(40, b);

  //newtypes
  struct Gizmold (int);
  let mygizmoid: Gizmold = Gizmold(10);
  //let idint: int = *mygizmoid;  //non-passed for rust-0.9: single-field tuple-structs can no longer be dereferenced
  let Gizmold(idint) = mygizmoid;
  assert_eq!(10, idint);
}

fn functions() {
  fn line(a: int, b: int, x: int) -> int {
    return a * x + b;
  }

  assert!(1 * 2 + 3 == line(1,2,3));

  fn donothingthehardway() -> () {
    return ();
  }
  donothingthehardway();
  fn donothingthehardway1() {
  }
  donothingthehardway1();

  fn oops(a: int, b: int, x: int) -> () {
    a * x + b;  // Ending the function with a semicolon like so is equivalent to returning ()
  }
  assert!(() == oops(5, 3, 1));

  fn first((value, _): (int, f64)) -> int {
    value
  }

  assert!(first((100, 200.0)) == 100);
}


fn destructors() {
  let y = ~10;
  assert_eq!(10, *y);
}

fn ownership() {
  struct Foo { x: int, y: ~int }
  let a = Foo {x: 5, y: ~ 10};
  let mut b = Foo { x: 5, y: ~10};
  b.x = 10;
  assert!(5 + 10 == a.x + b.x);
}

#[deriving(Clone)]
enum List {
  Cons(u32, ~List),
  Nil
}

fn boxes() {
  struct Foo {a: u32, b: u32, c: u32, d: u32 }
  assert_eq!(size_of::<Foo>(), size_of::<u32>() * 4);

  struct Bar {
      a: Foo,
      b: Foo,
      c: Foo,
      d: Foo
  }
  assert_eq!(size_of::<Bar>(), size_of::<u32>() * 16);

  let nil: List = Nil;
  assert!(~"Nil" == format!("{:?}", nil));
  let list = Cons(10, ~Nil);
  assert_eq!(~"Cons(10u32, ~Nil)", format!("{:?}", list));
  let list2 = Cons(10, ~Cons(20, ~Nil));
  match list2 {
    Cons(_, ~Cons(x, _)) => assert_eq!(20, x),
    Cons(_,_) => fail!("fail"),
    Nil => fail!("fail")
  }

  fn println_list_node_values(list: List) {
    match list {
      Cons(x, y) => {
        x+1;
        println_list_node_values(*y);
      }
      _ => {}
    }
  }

  println_list_node_values(list2);

  // Move semantics
  let xs = Cons(1, ~Cons(2, ~Cons(3, ~Nil)));
  assert_eq!(~"Cons(1u32, ~Cons(2u32, ~Cons(3u32, ~Nil)))", format!("{:?}", xs));
  let ys = xs; // copies `Cons(u32, pointer)` shallowly
  // println!("{:?}", xs); // Rust will consider a shallow copy of a type with a destructor like List to move ownership of the value 
  assert_eq!(~"Cons(1u32, ~Cons(2u32, ~Cons(3u32, ~Nil)))", format!("{:?}", ys));

  let x = ~5;
  let mut y = x.clone(); // y is a newly allocated box
  let mut z = x;
  //println!("{:?}", x); // use of moved value: `x`
  *z = 10;
  *y = 100;
  assert_eq!(10, *z);

  let x1 = Cons(5, ~Nil);
  let y1 = x1.clone();
  let z1 = x1;

  assert_eq!(~"Cons(5u32, ~Nil)", format!("{:?}", y1));
  assert_eq!(~"Cons(5u32, ~Nil)", format!("{:?}", z1));

  let r = ~13;
  let mut s = r;
  *s += 1;
  let t = s;
  assert_eq!(~14, t);

  fn prepend(xs: List, value: u32) -> List {
    Cons(value, ~xs)
  }

  let mut xs = Nil;
  xs = prepend(xs, 1);
  xs = prepend(xs, 2);
  xs = prepend(xs, 3);
  println_list_node_values(xs);
}

fn references() {
  // A reference is a *non-owning* view of a valu
  // A reference can be obtained with the `&` (address-of)
  // operator. It can be dereferenced by using the `*` operator. 
  fn list_eq(xs: &List, ys: &List) -> bool {
    match (xs, ys) {
      (&Nil, &Nil) => true,
      (&Cons(x, ~ref next_xs), &Cons(y, ~ref next_ys)) if x == y => list_eq(next_xs, next_ys),
      _ => false
    }
  }

  assert!(list_eq(&Cons(1, ~Nil), &Cons(1, ~Nil)));

  let a = &1;
  assert!(a == &1 && *a == 1);
  fn inc(i: &uint) -> int {
    (*i + 1) as int
  }
  assert!(inc(&1u) == 2 && inc(&100u) == 101);

  assert!(&1 == &1);
  assert!(&2 != &1);
}

enum ListG<T> {
  ConsG(T, ~ListG<T>),
  NilG
}

fn prepend<T>(xs: ListG<T>, value: T) -> ListG<T> {
  ConsG(value, ~xs)
}

impl<T: Eq> Eq for ListG<T> {
  fn eq(&self, ys: &ListG<T>) -> bool {
    match (self, ys) {
      (&NilG, &NilG) => true,
      (&ConsG(ref x, ~ref next_xs), &ConsG(ref y, ~ref next_ys)) if x == y => next_xs == next_ys,
      _ => false
    }
  }
}

fn test_list_g() {
  let mut xs = NilG::<int>;
  xs = prepend(xs, 10);
  xs = prepend(xs, 15);
  xs = prepend(xs, 20);

  fn each<T>(list: &ListG<T>) {
    match list {
      &NilG => {}
      &ConsG(ref x, ~ref y) => {
        info!("{:?}", *x);
        each(y);
      }
    }
  }

  each(&xs);

  let x1 = ConsG(5, ~ConsG(10, ~NilG));
  let x2 = ConsG(5, ~ConsG(10, ~NilG));
  assert!(x1.eq(&x2));
  assert!(&x1.eq(&x2));
  assert!(x1 == x2);
  assert!(!x1.ne(&x2));
  assert!(!(x1 != x2));
}

fn boxes_more() {
  let x = 5;    // immutable
  let mut y = 5;// mutable
  y +=2;
  assert!(x == 5)
  assert!(y == 7);

  let x1 = ~5; //immutable;
  let mut y1 = ~5; // mutable
  *y1 +=2;
  assert!( (*y1) == 7 );
  assert!( y1 == ~y );
  assert!( x1 == ~5 )
}

fn borrowed_pointers() {
  struct Point {
    x: f64,
    y: f64
  }

  let on_the_stack: Point = Point { x: 3.0, y: 4.0 };
  let managed_box: @Point = @Point { x: 5.0, y: 1.0 };
  let owned_box : ~Point = ~Point { x:7.0, y:9.0 };

  fn compute_distance(p1: &Point, p2: &Point) -> f64 {
    let xd = p1.x - p2.x;
    let yd = p1.y - p2.y;
    (xd * xd + yd * yd).sqrt()
  }
  compute_distance(&on_the_stack, managed_box);
  compute_distance(owned_box, managed_box);

  let r0 = &5;
  let r1 = &mut 5;
  let mut r2 = &6;
  // let o1 = ~mut 5;// not ~mut error: found `mut` in ident position
  assert_eq!(6, *r2);

  // *r0 = 10; // cannot assign to immutable dereference of & pointer
  *r1 = 10;
  //*r2 = 10;    // error: cannot assign to immutable dereference of & pointer
  let tt = &20;
  r2 = tt;
  //r2 = &20;

  let mut o1 = ~5;
  let o2 = ~5;
  *o1 = 10;
  //*o2 = 10;

  let m1 = @5;
  let mut m2 = @5;
  // *m1 = 10;
  //*m2 = 10; //  error: cannot assign to immutable dereference of @ pointer
  assert_eq!(5, *m2);
  m2 = @10;

  assert!(*r0 ==5 && *r1 == 10 && *r2 == 20 && *o1 == 10 && *o2 == 5 && *m1 == 5 && *m2 == 10);
}

fn freezing() {
  let mut x = 5;
  {
    let y = &x;
    assert!(x == 5);
    assert!(*y == 5);

    //let mut y1 = &x;
    //*y1 = 10; // cannot assign to immutable dereference of & pointer
    // x = 20; // cannot assign to `x` because it is borrowed
  }
  x = 10;
  assert!(x == 10);

  /*
  non-passed for rust-0.9
  // Mutable managed boxes
  let x = @mut 5;  // * `@mut` has been removed. Use `std::cell::{Cell, RefCell}` instead.
  let y = x;
  {
    let z = &*y;
    // *x = 10; //task '<main>' failed
    println!("@mut x: {}", *x);
    println!("*z: {}", *z);
  }
  *y = 20;
  println!("@mut x: {}", *x);
  */
}

fn dereferenced() {
  let managed = @10;
  let owned = ~20;

  let value = 30;
  let borrowed = &value;

  let sum = *managed + *owned + *borrowed;
  assert_eq!(10 + 20 + 30, sum);

  let m = @10;
  let mut o = ~20;
  let mut v = 30;

  let b = &mut v;

  *o = *b + 100;
  *b = *m + 1000;

  assert_eq!(10 + 30 + 100 + 10 + 1000, *m + *o + *b);

  // Pointers have high operator precedence, but lower precedence than the dot operator used for filed and method access.
  struct Point {
    x: f64,
    y: f64
  }

  let start = @Point{ x: 100.0, y:20.0 };
  let end = ~Point {x:(*start).x + 100.0, y:(*start).y + 100.0};
  assert_eq!(100.0, end.x - start.x);

  // to combat this ugliness the dot operator applies "automatic pointer dereferencing" to the receiver(the value on the left-hand size of the dot), so in most cases, explicityly dereferencing the receiver is not necessary.
  let start1 = @Point{x:100.0,y:20.0};
  let end1 = ~Point{x:start.x +100.0, y:start.y + 100.0};
  assert_eq!(100.0, end1.x - start1.x);

  let point = &@~Point { x: 10.0, y: 20.0 };
  assert_eq!(10.0 + 20.0, point.x + (*(*(*point))).y);

  //let arr: ~@[int] = ~@[1, 3, 5]; // error: obsolete syntax: managed vector
  let arr = std::rc::Rc::new(~[1,3,5]);
  //assert_eq!(size_of::<~int>(), size_of::<~@[int]>());
 // assert_eq!(size_of::<~int>(),  size_of::<@[int]>());
  assert_eq!(size_of::<int>() * 10, size_of::<[int, ..10]>());
  assert_eq!(~"~[1, 3, 5]", format!("{:?}", *arr));
  assert_eq!((1,5), ((*arr)[0], (*arr)[2]));

  let arr1 = std::rc::Rc::new([1,2,3]);
  assert_eq!(~"[1, 2, 3]", format!("{:?}", *arr1));
  assert_eq!(1, (*arr1)[0]);
}

fn vectors() {
  // A fixed-size vector
  let numbers = [1, 2, 3];
  assert!([1, 2, 3] == numbers);

  let more_numbers = numbers;
  assert!([1,2,3] == more_numbers);
  assert!([1,2,3] == numbers);
  // assert!(&more_numbers == &numbers); // failed to find an implementation of trait std::cmp::Eq for [int, .. 3]

  // the type of a fixed-size vector is writeten as '[Type, ..length]'
  let five_zeroes: [int, ..5] = [0, ..5];
  //five_zeroes[0] = 10; // cannot assign to immutable vec content
  let mut fz = five_zeroes;
  fz[0] = 10;
  assert!([0,0,0,0,0] == five_zeroes);
  assert!([10,0,0,0,0] == fz);
  fz.iter().advance(|x| {
    x.to_str();
    true
  });
  let fz: [int, ..3] = [0, 1, 2];
  assert!([0, 1, 2] == fz);
  let mut mfz = fz; // value clone
  mfz[0] = 10;
  assert!(fz[0] == 0);
  assert!(mfz[0] == 10);

  //A unique vector is dynamically sized, and has a destructor to clean up allocated memory on the heap.
  //A unque vector owns the elements it contains, so the elements are mutable if the vector is mutable.

  // A dynamically sized vector (unique vector)
  let mut numbers1= ~[1,2,3];
  numbers1.push(4);
  numbers1.push(5);
  // type type of a unique vector is written as ~[int]
  let more_numbers1: ~[int] = numbers1;
  assert_eq!(1, more_numbers1[0]);
  assert_eq!(5, more_numbers1[4]);

  // // The original ‘numbers‘ value can no longer be used, due to move semantics.
  // println!("{:?}", numbers1); // use of moved value: `numbers1`

  //slices are similar to fixed-size vectors, but the lenght is not part of the type.
  // They simply point into a block of memery and do not have ownership over the elements.

  // A slice
  let xs = &[1, 2, 3];
  //Slices have their type written as &[int]
  let ys: &[int] = xs;

  assert!(&[1,2,3] == xs && &[1,2,3] == ys);

  // Other vector types coerce to slices
  let three = [1, 2, 3];
  let zs: &[int] = three;
  assert!(three == zs);

  let mut s1 = [1, 2, 3];
  s1[0] = 100;
  assert!(s1[0] == 100); assert!(s1[1] == 2); assert!(s1[2] == 3);

  let view = s1.mut_slice(0, 2); // &mut [1, 2]
  assert!(view[0] == 100); assert!(view[1] == 2);
  view[0] = 200;
  assert!(s1[0] == 200);

  // the type of a muatable slice is wirtten as &mut [T]
  let s2: &mut [int] = &mut [1, 2, 3];
  assert!(s2[0] == 1); assert!(s2[1] == 2); assert!(s2[2] == 3);
  s2[2] = 1000;
  assert!(s2[0] == 1); assert!(s2[1] == 2); assert!(s2[2] == 1000);

  // Dquare brackets denote indexing into a vector:
  let crayons: [int, ..3] = [1, 2, 3];
  assert!(crayons[0] == 1); assert!(crayons[1] == 2); assert!(crayons[2] == 3);

  // A vector can be destructured using pattern matching
  let ss: &[int] = &[1, 2, 3];
  let score = match ss {
    [] => 0,
    [a] => a * 10,
    [a, b] => a * 6 + b * 4,
    [a, b, c, ..rest] => a * 5 + b * 3 + c * 2 + rest.len() as int 
  };
  assert_eq!(1 * 5 + 2 * 3 + 3 * 2, score);

  fn vec_each<T>(v: &[T], c: |a: &T|) {
    match v {
      [ref a, ..rest] => {
        c(a);
        vec_each(rest, c);
      }
      _ => {}
    }
  }

  vec_each([1,2,3], |a| { format!("v {:?}", a); });

  let nn = [0, 1, 2];
  for &x in nn.iter() {
    format!("{} is a number!", x);
  }
}

fn strings() {
  let mut string = ~"fo";
  string.push_char('o');
  for c in range(0, string.len()){
    format!("{:u},", string[c]);
  }

  // An unadorned string literal is an immutable string slice
  let s = "foobar";
  // A string slice type is written as &str
  let view: &str = s.slice(0, 3);
  assert!(s == "foobar" && "foo" == view);
}

fn ownership_escape_hatches() {
  let x = Rc::new([1,2,3,4,5,6,7,8,9,10]);
  let y = x.clone(); // a new owner
  let z = x; // this moves 'x' into 'z', rather than creating a new owner

  assert!(*y == [1,2,3,4,5,6,7,8,9,10]);
  assert!(*z == [1,2,3,4,5,6,7,8,9,10]);

  // the variable is mutable, but not the contents of the box
  let mut a = Rc::new([10,9,8,7,6,5,4,3,2,1]); 
  assert!([10,9,8,7,6,5,4,3,2,1] == *a);
  a = z;
  assert!([1,2,3,4,5,6,7,8,9,10] == *a);

  let x1 = Gc::new([1,2,3,4,5,6,7,8,9,10]);
  let y1 = x1; // does not perform a move, unlike with 'Rc'
  let z1 = x1;

  assert!(*x1.borrow() == [1,2,3,4,5,6,7,8,9,10]);
  assert!(*y1.borrow() == [1,2,3,4,5,6,7,8,9,10]);
  assert!(*z1.borrow() == [1,2,3,4,5,6,7,8,9,10]);
}

fn closures() {
  fn call_closure_with_ten(b: |int| -> uint) -> uint {
    b(10)
  }

  let captured_var = 20;
  let closure = |arg: int| { format!("captured_var={}, arg={}", captured_var, arg); arg as uint };

  call_closure_with_ten(closure);

  let square = |x: int| -> uint { (x * x) as uint };
  format!("call_closure_with_ten(square): {:u}", call_closure_with_ten(square));

  //
  let mut max = 0;
  //[1,2,3,200,100].iter().map(|x| println!("{}", x));
  for &x in  [1,2,3,200,100].iter() {
    if x > max { max = x }
  }

  //[1,2,3,200,100].iter().map(|x| if *x > max { max = *x });
  assert_eq!(200, max);

  fn max_it(i: ~[int]) -> int {
    let mut max = 0;
    for &x in i.iter() {
      if x > max { max = x }
    }
    max
  }

  fn max_stack(i: [int, ..4]) -> int {
    let mut max = 0;
    for &x in i.iter() {
      if x > max { max = x }
    }
    max
  }

  assert_eq!(400, max_it(~[1,2,3,400]));
  assert_eq!(222, max_stack([1,2,3,222]));

  fn call_twice(f: ||) {
    f();
    f();
  }
  let c = || {
    format!("I'm a closure, and it doesn't matter what type I am!");
  };
  fn f() { format!("I'm a normal function"); }
  call_twice(c);
  call_twice(f);
}

fn do_syntax() {
  fn call_it(op: proc(v: uint)) {
    let x:uint = random();
    op(x % 100)
  }

  call_it(proc(n){
    format!("invoke proc: {}", n.to_str());
  });

  call_it(proc(n) {
    format!("do invoke: {}", n.to_str());
  });

  spawn(proc(){
    "I'm a task, whatever";
  });

  spawn(proc() {
    "I'm a task yet, whatever";
  });
}

fn methods() {
 struct Point {
  x: f64,
  y: f64
 }

 enum Shape {
  Circle(Point, f64),
  Rectangle(Point, Point)
 }

 impl Shape {
  fn draw(&self) {
    match *self {
      Circle(p, f) => { format!("draw circle {:?} {:?}", p, f); }
      Rectangle(p1, p2) => { format!("draw rect {:?} {:?}", p1, p2); }
    }
  }
 }

 let c = Circle(Point{x:1.0, y: 2.0}, 10.0);
 c.draw();
 let r = Rectangle(Point{x:10.0,y:10.0}, Point{x:20.0, y:20.0});
 r.draw();

 struct CircleStruct {
    radius: f64
 }

 impl CircleStruct {
  fn area(&self) -> f64 {
    self.radius * self.radius
  }

  fn new(area: f64) -> CircleStruct {
    CircleStruct{radius: (area/f64::consts::PI).sqrt()}
  }
 }

 let c1 = CircleStruct::new(100.0);
 format!("c1:{:?}, area:{:f}", c1, c1.area());
}

fn generics() {
  fn map<T, U>(vector: &[T], func: |&T| -> U) -> ~[U] {
    let mut accumulator = ~[];
    for element in vector.iter() {
      accumulator.push(func(element));
    }
    accumulator
  }

  assert_eq!(~[2,3,4], map([1,2,3], |x| { *x + 1 }));
  let d = map(&[1,2,3], |x| { *x + 1 });
  assert_eq!(~[2,3,4], d);
  map([1,2,3], |x| { 
    format!("{:?}", x);
    *x + 1 
  });

  let a = [1,2,3];
  let b =~a;
  assert!([1,2,3] == a && [1,2,3] == *b);

  let c = ~[1,2,3];
  assert_eq!(~[1,2,3], c);
  //println!("{:?}", *c);

  type Set<T> = HashMap<T, ()>;

  struct Stack<T> {
    elements: ~[T]
  }

  let s = Stack{elements: ~[1,2,3]};
  let x = Stack::<int>{elements: ~[1,2,3]};
  assert_eq!((~[1,2,3], ~[1,2,3]), (s.elements, x.elements));

  enum MyOption<T> {
    MySome(T),
    MyNone
  }

  fn value(i: int) -> MyOption<uint> {
    if i == 0 {
      MyNone
    }else {
      MySome(i as uint * 10)
    }
  }

  match value(1) {
    MySome(i) => { format!("has value {}", i); }
    MyNone => { format!("no value"); }
  }

  match value(0) {
    MySome(i) => { format!("has value, {}", i); }
    MyNone => { format!("no value"); }
  }
}

fn traits() {
  fn head<T: Clone>(v: &[T]) -> T {
    v[0].clone()
  }

  assert_eq!(1, head(&[1,2,3]));

  struct TimeBomb {
    ex: uint
  }

  impl Drop for TimeBomb {
    fn drop(&mut self) {
      for i in range(0, self.ex) {
        format!("blam! {}", i);
      }
    }
  }

  let t  = TimeBomb { ex: 2 };
  assert_eq!(2, t.ex);

  trait Printable {
    fn print(&self);
  }

  impl Printable for int {
    fn print(&self) {
      format!("printable: {}", *self);
    }
  }

  impl Printable for ~str {
    fn print(&self) { format!("Printable: {:s}", *self); }
  }

  let i = 100;
  i.print();
  let s: ~str = ~"hello";
  s.print();

  trait Seq<T> {
    fn length(&self) -> uint;
  }

  impl<T> Seq<T> for ~[T] {
    fn length(&self) -> uint {
      self.len()
    }
  }

  let arr = ~[1,1,2,100];
  assert_eq!(arr.len(), arr.length());

  trait Shape { fn new(area: f64) -> Self; }
  struct Circle { radius: f64 }
  struct Square { length: f64 }

  impl Shape for Circle {
    fn new(area: f64) -> Circle {
      Circle { radius: (area / f64::consts::PI).sqrt() }
    }
  }

  impl Shape for Square {
    fn new(area: f64) -> Square {
      Square { length: (area).sqrt() }
    }
  }

  let c1: Circle = Shape::new(10.0);
  let s1: Square = Shape::new(10.0);
  println!("cirlce: {:?}", c1);
  println!("square: {:?}", s1);

  fn print_all<T: Printable>(printable_things: ~[T]) {
    for thing in printable_things.iter() {
      thing.print();
    }
  }
  print_all(~[100,200,300]);

  fn p<T: Printable + Clone>(ps: ~[T]) {
    let mut i = 0;
    while i < ps.len() {
      let c = ps[i].clone();
      c.print();
      i += 1;
    }
  }

  p(~[~"abc", ~"def"]);
}
