#[feature(macro_rules)];
#[feature(struct_variant)];

extern crate collections;
extern crate rand;

use collections::HashMap;
use collections::TreeMap;

use std::from_str::FromStr;

fn main() {
    test_ToStr();
    test_macro();
    test_destructuring_pattern_matching();
    test_conv_from_str();
    test_hashmap();

    // 2014-02-11
    test_params_builder();
 }

// In order to have a default implementaton of to_str() you need to add #[deriving(, Rand)]
#[deriving(Rand,Show)]
enum Weapons {
    Sword,
    Club,
    Gaze
}

fn test_ToStr() {
    // prelude: pub use to_str::{ToStr, IntoStr};
    /*
    /// A generic trait for converting a value to a string
    pub trait ToStr {
    /// Converts the value of `self` to an owned string
    fn to_str(&self) -> ~str;
    }
    */
    println!("{:s}", Club.to_str());

    /*
    The alternative, workin' in most cases, would be using format!.
    The formatting string {:?} means it will use reflection to display the struct name and values.
    This is similar to Ruby's #inspect.
    */
    enum Monsters {
      Goblin,
      Orc
    }
    println!("{:s}", format!("{:?}", Orc));
}

fn test_macro() {
    macro_rules! p(
        ($ident:ident) => (
            println!("{:?}", $ident);
        );
    )

    struct Foo {
        bar: uint
    }

    let f = Foo { bar: 10 };
    p!(f);
}

fn test_destructuring_pattern_matching() {
    // let 
    let tuple = (1, 2);
    let (a, b) = tuple;
    assert_eq!(1, a);
    assert_eq!(2, b);

    fn test_match_in_range() {
        match 5 {
            1..5 => println!("true"),
            8|9 => println!("unknow"),
            _ => println!("false")
        }
    }
    test_match_in_range();

    //structs
    struct Foo { x: (uint, uint), y: uint }
    let foo = Foo { x: (1, 2), y: 3 };
    let Foo { x: tuple @ (a, b), .. } = foo;
    assert_eq!((1, 2), tuple);
    assert_eq!(a, 1);
    assert_eq!(b, 2);

    struct Point {
        x: int,
        y: int
    }
    let p = Point{ x: 1, y: 2 };
    let Point { x: nx, y: ny } = p;
    assert_eq!(1, nx);
    assert_eq!(2, ny);
    let Point{ y, x } = p;
    assert!( y ==2 && x == 1 );

    let Point {y:newy, ..} = p;
    assert_eq!(2, newy);

    enum Fagazi {
      Fob { a: int },
      Fooo { b: int, c: int }
    }

    let foo = Fooo { b: 1, c: 2 };
    match foo {
        Fooo { b, c } if b > 2 => assert!((b, c) == (1, 2)),
        Fooo { b, c }          => assert!((b, c) == (1, 2)),
        _                      => unreachable!() //fail!("This will never happen, but the compiler doesn't know")
        // unreachable!() expands to fail!("internal error: entered unreachable code").
    }

    //match allows to match on concrete values:
    let value = Some(100);
    match value {
        Some(1) => println!("one"),
        Some(2) => println!("two"),
        Some(v) => println!("value is {:d}", v),
        None    => unreachable!()
    }

    fn test_vectors() {
        // removing unique vectors from the language
        /*
        let v = ~[1,2,3,4,5];
        match v {
            [] => println!("empty"),
            [elem] => println!("{}", elem),
            [first, second, ..rest] => println!("first:{}, second:{}, rest:{:?}", first, second, rest)
        }

        match v {
            [first, ..] => assert_eq!(1, first),
            [] => unreachable!()
        }

        match v {
            [.., last] => assert_eq!(5, last),
            [] => unreachable!()
        }

        match v {
            [first, ..middle, last] => { 
                assert_eq!(1, first);
                assert_eq!(5, last);
                assert_eq!(&[2,3,4], middle);
            }
            [_oneelem] => unreachable!(),
            [] => unreachable!()
        }


        let ss = ~[~"abc", ~"def"];
        match ss {
            [~"abc"] => unreachable!(),
            [a, _b] => assert_eq!(~"abc", a),
            _ => unreachable!()
        }
        */
        let v = ~[1,2,3,4,5];

        match v.len() {
            0 => println!("empty"),
            1 => println!("{}", v[0]),
            _ => println!("first:{}, second:{}, rest:{:?}", v[0], v[1], v.tail())
        }

        match v.len() {
            0 => unreachable!(),
            _ => assert_eq!(1, v[0])
        }

        match v.len() {
            0 => unreachable!(),
            _ => assert_eq!(&5, v.last().unwrap()),
        }

        match v.len() {
            0 => unreachable!(),
            1 => unreachable!(),
            _ => {
                let first = v[0];
                let last = v.last().unwrap();
                let middle = v.slice(1, v.len()-1);
                println!("middle: {:?}", middle);
                assert_eq!(1, first);
                assert_eq!(&5, last);
                assert_eq!(&[2,3,4], middle);
            }
        }


        let ss = ~[~"abc", ~"def"];
        match ss.len() {
            1 => unreachable!(),
            2 => assert_eq!(~"abc", ss[0]),
            _ => unreachable!()
        }
    }
    test_vectors();

    fn test_func_args(){
        fn myfunc((a,b): (uint, uint)) -> uint {
            a + b
        }
        assert_eq!(3, myfunc((1, 2)));
    }
    test_func_args();

    fn test_loop() {
        struct Pair { x: int, y: int }
        let pairs = ~[Pair {x:10, y:20}, Pair{x:30, y:40}];
        for &Pair{x, y} in pairs.iter() {
            println!("{}-{}", x, y);
        }
    }
    test_loop();
}

fn test_conv_from_str() {
    /*
    pub trait FromStr {
    /// Parses a string `s` to return an optional value of this type. If the
    /// string is ill-formatted, the None is returned.
    fn from_str(s: &str) -> Option<Self>;
    }

    /// A utility function that just calls FromStr::from_str
    pub fn from_str<A: FromStr>(s: &str) -> Option<A> {
    FromStr::from_str(s)
    }
    */
    let a = from_str::<int>("1").unwrap();
    assert_eq!(1, a);
    assert_eq!(None, from_str::<int>("1a1"));
    assert!(from_str::<uint>("1aa1").is_none());

    struct Name(~str);
    impl FromStr for Name {
        fn from_str(s: &str) -> Option<Name> {
            Some(Name(s.to_owned()))
        }
    }
    let n: Option<Name> = from_str::<Name>("hello");
    println!("name: {:?}", n);

    let f : f64 = from_str("1.2").unwrap(); // type inter
    assert_eq!(1.2, f);

    assert_eq!(10, from_str::<int>("10").unwrap_or(10));

    match from_str::<int>("100") {
        None => unreachable!(),
        Some(v) => assert_eq!(100, v)
    }
}

fn test_hashmap() {
    fn add_to_map(map: &mut HashMap<~str, uint>, k: &str, v: uint) {
        map.insert(k.to_owned(), v);
    }
    let mut map: HashMap<~str,uint> = HashMap::new();
    add_to_map(&mut map, "hello", 1);
    println!("map: {:?}", map);

    fn create_insert_find() {
        let mut map = HashMap::new();
        map.insert(~"foo", 1);

        assert_eq!(1, map.len());
        assert!(map.contains_key(&~"foo"));

        let mut map2: HashMap<~str, int> = HashMap::new();
        map2.insert(~"ss", 2);
        assert_eq!(1, map.len());
        assert!(map2.contains_key(&~"ss"));

        let mut m1 = HashMap::<~str, &'static str>::new();
        m1.insert(~"name", "itang");
        assert!(m1.contains_key(&~"name"));

        {
            let a = m1.find_or_insert(~"fun", "programming");
            println!("a: {:?}", a); // a: &mut "programming"
        }

        m1.find_or_insert_with(~"age", |_k| "20" );
        assert!(m1.contains_key(&~"age"));
        assert_eq!("20", *m1.get(&~"age"));

        m1.insert_or_update_with(~"incoming", "100w", |_k, v| *v = "1000w");
        assert_eq!("100w", *m1.get(&~"incoming"));
        m1.insert_or_update_with(~"incoming", "100w", |_k, v| *v = "1000w");
        assert_eq!("1000w", *m1.get(&~"incoming"));
     }
    create_insert_find();

/*
    fn test_mangle() {
        let mut map = HashMap::<~str, uint>::new();
        assert!(!map.contains_key(&~"foo"));

        map.mangle(~"foo", 1, 
            |_k, a| a + 10,
            |_k, v, _a| *v -=2 );
        assert_eq!(*map.get(&~"foo"), 11);

        let x = *map.mangle(~"foo", 1,
           |_k, a| a + 10,
           |_k, v, _a| {
            println!("{:?} {:?} {:?}", _k, v, _a);
              *v -=  2 // take the current value in ~"foo" and apply the function on it
           }
           );

        println!("x:{:?}", x);
        assert_eq!(*map.get(&~"foo"), 9);
    }
    test_mangle();
*/

    fn test_get() {
        let mut map = HashMap::<~str, uint>::new();
        map.insert(~"age",10);
        {
            let r = map.get(&~"age");
            assert!(*r == 10);
        }
        {
            let s = map.get_copy(&~"age");
            assert!(s == 10);
        }
        {
            let rr = map.get_mut(&~"age");
            *rr = 100;
        }
        assert!(*map.get(&~"age") == 100);

        let result = match map.find(&~"age") {
            None => 1000,
            Some(v) => *v
        };
        assert_eq!(100, result);

        let result1 = match map.find_copy(&~"age1") {
            None => 1000,
            Some(v) => v
        };
        assert_eq!(1000, result1);
    }
    test_get();

    fn test_pop() {
        let mut map = HashMap::<~str,uint>::new();
        map.insert(~"age", 90u);
        let r = match map.pop(&~"age") {
            None => 1000,
            Some(v) => v
        };
        assert_eq!(90, r);
        assert_eq!(false, map.contains_key(&~"age"));
    }
    test_pop();

    fn test_iterating() {
        let mut map = HashMap::<~str, uint>::new();
        map.insert(~"foo", 1);
        map.insert(~"bar", 2);

        let mut numbers = ~[];
        for (_k, v) in map.iter() {
            numbers.push(*v);
        }
        assert!(numbers.contains(&2));
        assert!(numbers.contains(&1));

        println!("{:?}", map.values().map(|v| v.clone()).to_owned_vec());
        println!("{:?}", map.values().map(|v| v.clone()).collect::<~[uint]>());

        for (k, v) in map.move_iter() {
            println!("k:{:?}, v:{:?}", k, v);
        }
        // println!("{:?}", map); //error: use of moved value: `map`
    }
    test_iterating();
    
    fn test_treemap() {
        let mut treemap = TreeMap::<~str, uint>::new();
        treemap.insert(~"age", 10);
        treemap.insert(~"name", 20);
        let mut numbers = ~[];
        for (_k, v) in treemap.iter() {
            numbers.push(*v);
        }

        assert_eq!(~[10, 20], numbers);
    }
    test_treemap();
}

#[deriving(Eq, Show)]
struct ParamsBuilder {
    name: ~str,
    age: int
}

impl ParamsBuilder {
    fn default() -> ParamsBuilder{
        ParamsBuilder { name: ~"", age: 0 }
    }

    fn name(mut self, n: ~str) -> ParamsBuilder {
        self.name = n;
        self
    }

    fn age(mut self, a: int) -> ParamsBuilder {
        self.age = a;
        self
    }
}

fn test_params_builder() {
    let pb = ParamsBuilder::default().name(~"itang").age(10);
    assert_eq!(ParamsBuilder{ name:~"itang", age: 10}, pb);

    let pb1 = ParamsBuilder { name:~"tqibm", ..pb };
    assert_eq!(~"tqibm", pb1.name);
    assert_eq!(10, pb1.age);
}
