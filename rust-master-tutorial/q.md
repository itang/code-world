- &1 == &1 引用比较， 是自动解引用后的值的比较？

- match a {
    (ref a1, ref a2) => { a1 == a2 }
    _ => { false }
  }
  
  ref a1: a1 = &match_value?
  
- x1.eq(&x2) fn eq(&self, ys: &ListG<T>)
  x1 自动引用, x1.eq(&x2) => &x1.eq(&x2)

-   let mut x = 5;
  {
    let y = &x;  // x is now frozen, it cannot be modified

    //let mut y1 = &x;
    //*y1 = 10; // cannot assign to immutable dereference of & pointer

- fn map<T, U>(vector: &[T], function: |v: &T| -> U) -> ~[U]
  let d = map(&[1,2,3], |x| { *x + 1 });
  println!("map: {:?}", *d); // error: type ~[<VI6>] cannot be dereferenced

-   let a = [1,2,3];
  let b =~a;
  println!("{:?}, {:?}", a, *b);

  let c = ~[1,2,3];
  // println!("{:?}", *c); // error: type ~[<VI17>] cannot be dereferenced


- mutable & immutable
  -  let mut r2 = &6; 

     //*r2 = 10; // the variable is mutable, but not the contents of the box

  -  let r1 = &mut 5;
     *r1 = 10;   // the variable is immutable, but not the contents of the box, because of '&mut'

  - owned, when variable is mutable, the contents of the box is mutable
     let mut o1 = ~5;
     *o1 =  10; //

     let o2 = ~5;
     *o2 = 10; // error: cannot assign to immutable dereference of ~ pointer
  - managed
     let mut m2 = @5;
     *m2 = 10; // error: cannot assign to immutable dereference of @ pointe

- utomatic pointer dereferencing

- varargs ?

- default args && named args
