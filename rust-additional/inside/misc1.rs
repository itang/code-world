fn main() {
    let a = 1;
    let s = r##"\sss"##;
    println!("{}, {}, {}", a, s, s.len());
    assert_eq!("\\sss", s);

    struct FnStore {
        f: fn(name: ~str) -> ~str
    }
    fn echo(name: ~str) -> ~str {
        name + ",world"
    }
    let fs = FnStore {f: echo };
   // (fs.f)(~"hello")
    let f = fs.f;
    println!("{}", f(~"hello"));

    let b = &1;
    let &bb = b;
    println!("bb: {}", bb);

    let t = (&1, &2);
    let (&t1, &t2) = t;
    let y = &(1, 2);
    let &(y1, y2) = y;
    println!("{} {}", t1, t2);
    println!("{} {}", y1, y2);
}
