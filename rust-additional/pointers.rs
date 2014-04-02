#![feature(managed_boxes)]

use std::io::println;

fn main() {
  (||{
    fn inc(i: &mut int) {
      *i = *i + 1;
    }
    let i =&mut 10;
    inc(i);
    assert_eq!(11, *i);
  })();

  (||{
    let p0 = Point { x: 5, y: 10};
    let p1 = transform0(p0);
    println!("{:?}", p1);
    assert_eq!(5, p1.x);
    assert_eq!(10, p1.y);

    let p2 = transform1(&p1);
    assert_eq!(6, p2.x);

    spawn(proc() {
      println(p2.x.to_str());
    });

    let p3 = ~p2;
    let p4 = ~Point {x:10, y:20};
    spawn(proc() {
      println(p3.x.to_str());
      println(p4.x.to_str());
    });

  })();

  (||{
    let a = @Point {x:10, y:20};
    let b = a;
    println!("{},{}", b.x, a.x);
  })();

  (||{
    let p1 = &Point{x:10,y:20};
    let p2 = p1;
    println!("{:?}, {:?}", p1, p2);
    spawn(proc() {
      //println!("{:?}", p1);
    });

    fn foo(x: ~int) -> int { *x }
    let x = ~5;
    let y = ~foo(x);
    println!(" y:{}", *y);
    //println!("{}", *x); //error: use of moved value: `x`
  })();
}

struct Point {
  x: int,
  y: int
}

fn transform0(p: Point) -> Point {
  p
}

fn transform1(p: &Point) -> Point {
  Point { x: p.x + 1, y: p.y + 1}
}
