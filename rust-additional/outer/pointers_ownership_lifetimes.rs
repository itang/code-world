use std::io::println;

fn main() {
    fn owned_seven() -> ~int {
        // Allocate an int with value '3' on the heap, 'three' points to it.
        let three: ~int = ~3;
        // The same for four
        let four: ~int = ~4;
        // Dereference both 'three' and 'four', add them, store the result
        // in a newly allocated variable on the heap
        ~(*three + *four)
    }

    let seven: ~int = owned_seven();
    // writing (*seven).to_str() is not really , because the '.'
    // operator auto-dereferences, so we can also write 'seven.to_str()'
    println("3 + 4 = " + (*seven).to_str());
    // <-- seven goes out of scope and the memory it points to is 
    // deallocated here

    fn borrowed_points() {
        let three: &int = &3;
        let four: &int = &4;
        println("3 + 4 = " + (*three + * four).to_str());
    }
    borrowed_points();

    fn shallow_copy_semantics() {
        // in general Rust has what we meight call shallow copy semantics: When an object
        // is initialized via assgnment fo rcall-by-value then its memory is a bitwise copy of the object used to assign it.
        // However, this is changed when an object contains an owned pointer: because the owned pointer has move semantics, the object containing it must also have move semantics, otherwise we would again incur two independent owning copies.
        struct Pod {
            x: int, y: uint, z: [int, ..3]
        }
        struct WithOptr {x: int, p:~int}

        let a1 = Pod {x:3, y:4u, z: [1,2,3]};
        let a2 = a1;
        println!("{:?}, {:?}", a1, a2);
        let b1 = WithOptr{x:3, p:~4};
        let b2 = b1;
        println!("{:?}", b2);
        //println!("{:?}", b1); // error: use of moved value: `b1`


        enum MyEnum {
            X(int),
            Y(~int)
        }

        fn match_and_print(e: &MyEnum) {
            match *e {
                X(x) => println!("{}", x),
                // Y(y) => println!("{}", *y) // error: cannot move out of dereference of & pointer

                // ref: pass-by-value -> pass-by-borrowed-pointer semantics
                Y(ref y) => println!("{:?}", y)
            }
        }

        fn match_and_print_by_value(e: MyEnum) {
            match e {
                X(x) => println!("{}", x),
                Y(y) => println!("{:?}", y)
            }
        }

        fn match_and_print_mutable(e: &mut MyEnum) {
            match *e {
                X(x) => println!("{}", x),
                Y(ref mut y) => {
                    **y = 5;
                    println!("{}", **y);
                }
            }
        }
        match_and_print(&Y(~10));
        match_and_print_by_value(Y(~10));
        let y = &mut Y(~4);
        match_and_print_mutable(y);
        println!("y: {:?}",y);

        // Standard pattern matches are pass-by-value, meaning that the contents of the enum is either copied or moved.
        // However, this can only be done when we have ownership over the values to be moved. When when we apply match to a dereferenced borrowed pointer, we cannot move because we don't have ownership.
    }
    shallow_copy_semantics();

    fn ref_mut_let() {
        let mut x = 3;
        let ref mut y = x;
        *y = 4;
        println!("{:?}", y);
        assert_eq!(4, *y);
    }
    ref_mut_let();

    fn lifetimes() {
        // Rust must take a number of precautions to ensure these scenarios do not happen.
        // First,the memory that a borrowed pointer to must not be freed during that borrowed pointers lifetime.
        // Second, this memory must not change while it is borrowed.
        let name = ~"world";
        if 3 < 4 {
            let bname = &name;
            println!("Hello, {}!", name);
            println!("Hello, {:?}!", *bname);
        }
    }
    lifetimes();
}
