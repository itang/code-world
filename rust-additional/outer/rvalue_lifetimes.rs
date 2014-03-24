#[allow(deprecated_owned_vector)];

extern crate collections;

use collections::HashMap;
use std::cell::RefCell;

fn main () {
    let map = &mut HashMap::<int, int>::new();
    println!("map: {:?}", map);
    /* output
    &mut std::hashmap::HashMap<int,int>{k0: 14410165065180691900u64, k1: 14928154657159894810u64, resize_at: 24u, size: 0u, buckets: ~[None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None]}
    */
    let mut _temp = HashMap::<int, int>::new();
    let the_map = &mut _temp;
    println!("the_map: {:?}", the_map);

    let ref mut map2 = HashMap::<int, int>::new();
    map2.insert(1,2);
    map2.insert(2,200);
    println!("map2: {:?}", map2);

    let map3: RefCell<HashMap<int, int>> = RefCell::new(HashMap::<int, int>::new());
    let mut r = map3.borrow_mut();
    let data: &mut HashMap<int,int> = &mut *r;
    data.insert(1,100);
    data.insert(1,200);
    data.insert(2,300);
    println!("map3 data: {:?}", data);

    let map4: RefCell<HashMap<int, int>> = RefCell::new(HashMap::new());
    (*map4.borrow_mut()).insert(1,100);
    (*map4.borrow_mut()).insert(2,200);
    println!("map4: {:?}", map4);

    fn rvalue() {
        fn vec() -> ~[int] {
            ~[1,2,3]
        }

        for i in vec().iter() { 
            println!("{:?}", *i);
        }

        for &i in [1,2,3].iter() {
            println!("{:?}", i);
        }
    }
    rvalue();
}