// TODO:
// Should be Sendable ?
extern crate extra;

use std::clone::Clone;
use extra::time;

trait IDelay<T> {
    fn force(&mut self) -> T;
    fn is_forced(&self) -> bool;
}

struct Delay<'a, T> {
    value: Option<T>,
    func:  'a || -> T
}

impl<'a, T: Clone> Delay<'a, T> {
    fn new(p: 'a || -> T) -> Delay<'a, T> {
        Delay{ value: None, func: p }
    }
}

impl<'a, T: Clone> IDelay<T> for Delay<'a, T> {
    fn force(&mut self) -> T {
        match self.value {
            None => {
                let v = (self.func)();
                self.value = Some(v.clone());
                v
            },
            Some(ref v) => v.clone()
        }
    }

    fn is_forced(&self) -> bool {
        self.value.is_some()
    }
}

///////////////////////////////////////////////////////////////////
// test

#[deriving(Clone)]
struct Value {
    v: int
}

fn main() {
    let mut count = 0;
    let mut d = Delay::new(|| {
        println!("force!!!!!");
        count += 1;
        Value { v:range(0, 10000000).fold(0, |a, x| { a + x } ) }
    });
    println!("{:?}", d);

    assert!(!d.is_forced());
    let c1 = count;
    assert_eq!(0, c1);

    let s1 = time::now();
    let v1 = d.force();
    let s2 = time::now();
    let v2 = d.force();
    let s3 = time::now();

    assert!(d.is_forced());
    let c2 = count;
    assert_eq!(1, c2);

    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
    println!("v3: {:?}", d.force());
    println!("s1: {:?}", s1);
    println!("s2: {:?}", s2);
    println!("s3: {:?}", s3);
}
