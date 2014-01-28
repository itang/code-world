- match 可以作为statement（不需要分号结尾）或者expression（赋值表达式需要分号）
  同if

- match a {
    (ref a1, ref a2) => { a1 == a2 }
    _ => { false }
  }
  
  ref a1: 

  a1 = &match_value


  fn tvec() -> ~[(~int, ~int)] {
  ~[(~1,~2), (~3,~4)]
  }

  for &(ref a, ref b) in t.iter() {
    println!("a:{:?}, b:{:?}", *a, *b);
    /*
    a:~1, b:~2
    a:~3, b:~4
    */
  }

- &1 == &1 引用比较， 是自动解引用后的值的比较

- frozen
  let mut x = 5;
  {
    let y = &x;  // x is now frozen, it cannot be modified

    //let mut y1 = &x;
    //*y1 = 10; // cannot assign to immutable dereference of & pointer

- mutable & immutable
  -  let mut r2 = &6; 

     //*r2 = 10; // the variable is mutable, but not the contents of the box

  -  let r1 = &mut 5;
     *r1 = 10;   // the variable is immutable, but not the contents of the box, because of '&mut'

     just like:
     let mut t = 5;
     let r1 = &t;


  - owned, when variable is mutable, the contents of the box is mutable
     let mut o1 = ~5;
     *o1 =  10; //

     let o2 = ~5;
     *o2 = 10; // error: cannot assign to immutable dereference of ~ pointer
  - managed
     let mut m2 = @5;
     *m2 = 10; // error: cannot assign to immutable dereference of @ pointe

- ~str.as_ptr()

- non-move
pub fn eq(a: &~str, b: &~str) -> bool {
    eq_slice(*a, *b)
}
pub fn eq_slice(a: &str, b: &str) -> bool {
    eq_slice_(a, b)
}

- &'static str
```rust
let s = "hello, world";
```

The string then has the type `&'static str` meaning that the string is valid for the `'static`
lifetime, otherwise known as the lifetime of the entire program. As can be
inferred from the type, these static strings are not mutable.

- ~str chars
  let s = ~"收获1";
  for i in range(0, s.char_len()) {
    print!("{}", s.char_range_at(i).ch);
  }
  println("");

  for i in s.chars() {
    print!("{:c}", i);
  }

-  &[char], ~[char], @[char] viewed as object

- ptr operator

  #[test]
    fn test_as_ptr() {
        let buf = "hello".as_ptr();
        unsafe {
            assert_eq!(*ptr::offset(buf, 0), 'h' as u8);
            assert_eq!(*ptr::offset(buf, 1), 'e' as u8);
            assert_eq!(*ptr::offset(buf, 2), 'l' as u8);
            assert_eq!(*ptr::offset(buf, 3), 'l' as u8);
            assert_eq!(*ptr::offset(buf, 4), 'o' as u8);
        }
    }

- pub use
  reexported namespace in declare mod.
  pub use result::{Result, Ok, Err};

- root mod for lib from 'crate' 
  "#[crate_id = "active_support"];"
  use active_support::Period;

- struct new, return value, not pointers, caller decise how to use.

- mut self
  copy, modify, return
  pub fn years(mut self, years: f32) -> TimeChange {
    self.years = Some(years);
    self
  }

- type default value

```rust
    pub fn range_with_start_zero<A: Add<A, A> + Ord + Clone + One + Default>(stop: A) -> Range<A> {
        range(Default::default(), stop)
    }
```