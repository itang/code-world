- &'static str -> ~ str ???

- ~str.to_owned  ==   let y = x.clone()  ??

let mut b = ~"abc";
    assert_eq!(b, b.to_owned());

- x1.eq(&x2) fn eq(&self, ys: &ListG<T>)
  x1 自动引用, x1.eq(&x2) => &x1.eq(&x2)

-  vector dereferenced

  fn map<T, U>(vector: &[T], function: |v: &T| -> U) -> ~[U]
  let d = map(&[1,2,3], |x| { *x + 1 });
  println!("map: {:?}", *d); // error: type ~[<VI6>] cannot be dereferenced

  let a = [1,2,3];
  let b =~a;
  println!("{:?}, {:?}", a, *b);

  let c = ~[1,2,3];
  // println!("{:?}", *c); // error: type ~[<VI17>] cannot be dereferenced


- automatic pointer dereferencing

- varargs ?

- default args && named args

- type @str cannot be dereferenced
    let j: @str = @"hello";
    //println!("j:{:?} {}", j, *j);
    let os: ~str = ~"hello";
    //println!("{:?}, {}", os, *os); 

- type ~[<VI2>] cannot be dereferenced
  let x = ~[1, 2, 3];
  let y = @*x;
  println!("{:?}{:?}",*x, *y);
  
-  error: borrowed value does not live long enough
fn vec() -> ~[int] {
  ~[1,2,3]
}

  for i in vec().iter() { // error: borrowed value does not live long enough
    println!("{:?}",i);
  }

  let v = vec();
  for i in v {

  }


-  so syntax?
for 'a |char| ??
impl<'a> CharEq for 'a |char| -> bool {
    #[inline]
    fn matches(&self, c: char) -> bool { (*self)(c) }

    fn only_ascii(&self) -> bool { false }
}

for extern "Rust" fn(char)??
impl CharEq for extern "Rust" fn(char) -> bool {
    #[inline]
    fn matches(&self, c: char) -> bool { (*self)(c) }

    fn only_ascii(&self) -> bool { false }
}

- method & or non-&
non-& is clone?
fn append(self, rhs: &str) -> ~str {
  let mut new_str = self;
  new_str.push_str_no_overallocate(rhs);
  new_str
}

{
  let mut a1 = A {a:1};
  a1.f1();
  println!("a1:{:?}", a1);

  let a2 = A {a:2};
  let a3 = a2.f2();
  println!("a2:{:?}, a3: {:?}", a2, a3);

  /*

  a1:A{a: 20}
  a2:A{a: 2}, a3: ~A{a: 10}

 */
}

struct A {
  a: int
}

impl A {
  pub fn f1(&mut self) -> ~A {
    (*self).a = 20;
    ~*self
  }

  pub fn f2(self) -> ~A {
    let mut new = self; // clone
    new.a = 10;
    ~new
  }
}

- @str clone
impl Clone for @str {
    #[inline]
    fn clone(&self) -> @str {
        *self // ??? *self -> @str
    }
}

- error: borrowed value does not live long enough
let mut r2 = &6;
  
r2 = &20; // error: borrowed value does not live long enough