#![feature(managed_boxes)]
struct Record {
  x: int,
  y: int
}

fn main() {
  // A borrowed pointer is always a pointer into memory owned by someone else, genrally a caller to the current function.
  let r: Record = Record{x: 10, y: 10}; // stored on the stack
  let p: &Record = &r; //p is a borrow of the record as a whole
  let q: &int = &r.x; //q is a borrow of the field x specifically

  println!("{:?}, {:?}, {:?}", r, p, q);

  fn applied_rvalue() {
    // One convenience : in C,the & operator can only be applied to lvalues(assignable locations).In Rust,
    // the & operator can also be applied to rvalues, in which case it means "allocate sime tempporray space on the stack and copy the value in there".
    let p = &Record { x: 10, y: 10 };
    println!("{:?}", p);
  }
  applied_rvalue();

  fn converting() {
    // the second way to create a borrowed pointer is by converting a shared or 
    // unique box into a borrowed pointer. These conversions happen implicitly on function and method call.
    fn distance_from_origin(_r: &Record) -> int {
      0
    }
    // Record stored on stack:
    let r1 = &Record{x:10, y:10};
    let d1 = distance_from_origin(r1);
    //Record store in shared box;
    let r2 = @Record{x:10, y:10};
    let d2 = distance_from_origin(r2);

    //Record stored in unique box:
    let r3 = ~Record{x:10, y:10};
    let d3 = distance_from_origin(r3);
    println!("{:?}, {:?}, {:?}", r1, r2, r3);
    println!("{:?}, {:?}, {:?}", d1, d2, d3);
  }
  converting();

  fn lifetimes() {
    // in the compiler, each borrowed pointer is associated with a lifetime(you may also have heard the term region).
    // A lifetime is a block or an expression during the pointer may be used.The compiler reports an error if a borrowed pointer
    // is used outside of its lifetime.
    // So far we have always written the type of a borrowed pointer as &T. In fact , the full type is &it.T Where It is a the name of a lifetime. You will rarely need to write this form 
    // but it may appear in error messages, and we will use it int he tutorial to clarify what's going on.
    // To make the idea of lifetimes more concrete.
    fn proc_int(_x: &int) {
      println!("{:d}", *_x);
    }
    fn lifetime_example(cond: bool) {
      if cond {
        let x: int = 10;
        proc_int(&x);
      }
    }
    lifetime_example(false);
    lifetime_example(true);
    // this example shows that lifetimes have a hierarchical relationship derived from the code itself. Every expression has a corresponding lifetime, and the lifetimes of subexpressions are nested inside of the lifetimes for the outer expressions.

  }
  lifetimes();

  fn lifetime_parameters() {
    // functions can be parameterized by lifetimes. In fact, this happens iplicityly whenever a parameter is a bowrrowed pointer.
    // this means that the compiler invents a synthetic lifetime, let's call if X, and syas "the lifetime of the parameter x is X"

    /*
    // cannot infer an appropriate lifetime due to conflicting requirements
    fn max1(a: &int, b: &int) -> &int {
      if *a > *b {a} else {b}
    }
    */

    fn max<'a>(a: &'a int, b: &'a int) -> &'a int {
      if *a > *b {a} else {b}
    }

    fn calls_max(cond: bool) {
      let x: int = 10;
      if cond {
        let y: int = 20;
        let z: &int = max(&x, &y);
        assert!(*z == 20);
      }
    }
    calls_max(true);
    
    fn calls_max2(_cond: bool) {
      let x: int = 10;
      let y: int = 20;
      let r = max(&x, &y);
      assert!(r == &20);
    }
    calls_max2(true);
  }
  lifetime_parameters();

  match None {
    Some::<int>(x) => println!("{:d}", x),
    None => println!("{:s}", "None")
  }
}