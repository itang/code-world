//use std::fmt::Result;
//use std::fmt::Show;
//use std::fmt::Formatter;

#[deriving(Clone, Eq, Show)]
struct Foo {
    x: int
}

/*
impl Show for Foo {
    fn fmt(&self, f: &mut Formatter) -> Result {
       write!(f.buf, "Foo\\{x: {:d}\\}", self.x)
    }
}
*/

/*
impl Clone for Foo {
    fn clone(&self) -> Foo {
        Foo {..*self}
    }
}*/

impl Drop for Foo {
    fn drop(&mut self) {
      println!("drop: {}", *self);
    }
}

fn main() {
    let f1 = Foo { x: 100 };
    //let f2 = f1;            //when impl Drop,  use of moved value: `f1` ??
    let mut f2 = f1.clone();         
    f2.x = 1000;
    println!("{} \n{}", f1, f2);
}
