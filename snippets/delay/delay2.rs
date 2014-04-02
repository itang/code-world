// Sendable
extern crate time;

use std::clone::Clone;

trait IDelay<T> {
    fn force(&mut self) -> T;
    fn is_forced(&self) -> bool;
}

struct Delay<T> {
    value: Option<T>,
    func:  Option<proc() -> T>
}

impl<T: Clone> Delay< T> {
    fn new(p: proc() -> T) -> Delay< T> {
        Delay{ value: None, func: Some(p) }
    }
}

impl<T: Clone> IDelay<T> for Delay<T> {
    fn force(&mut self) -> T {
        match self.value {
            None => {
                let function = self.func.take_unwrap();
                let v = function();
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
    let mut d = Delay::new(proc() {
        println!("force!!!!!");
        Value { v:range(0, 10000000).fold(0, |a, x| { a + x } ) }
    });
    println!("{:?}", d);

    assert!(!d.is_forced());

    let s1 = time::now();
    let v1 = d.force();
    let s2 = time::now();
    let v2: Value = d.force();
    let s3 = time::now();

    assert!(d.is_forced());

    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
    println!("v3: {:?}", d.force());
    println!("s1: {:?}", s1);
    println!("s2: {:?}", s2);
    println!("s3: {:?}", s3);

    //TODO: send
   /* spawn(proc() {
        let mut copyd = d;
        println!("spawn: {:?}", copyd.force());
        println!("spawn: {:?}", copyd.is_forced());
        assert!(copyd.is_forced());
    });
*/
}
