#![feature(managed_boxes)]

/**
 * lifetimes.
 */

struct Point {
    x: f64,
    y: f64
}

fn main() {
    // References never cleim any kind of ownership over the data that they point to:
    // instead, they are used for cases where you would like to use data for a short time.
    // 
    fn example(){
 

        let on_the_stack :  Point = Point { x: 3.0, y: 4.0 };
        let managed_box  : @Point = @Point { x: 5.0, y: 1.0 };
        let owned_box    : ~Point = ~Point { x: 7.0, y: 9.0 };

        fn sqrt(_f: f64) -> f64 { 0.0 }

        fn compute_distance(p1: &Point, p2: &Point) -> f64 {
            let x_d = p1.x - p2.x;
            let y_d = p1.y - p2.y;
            sqrt(x_d * x_d + y_d * y_d)
        }

        println!("{}", compute_distance(&on_the_stack, managed_box));
        println!("{}", compute_distance(managed_box, owned_box));
    }
    example();

    fn taking_its_address() {
        // Applying '&' to an rvalue (non-assignable location) is just a convenient shorthand for creating a temporary and taking its address.

        let p: &Point = &Point { x: 1.0, y: 1.0 };
        println!("{:?}, {:?}", p, *p);

        // A more verbose way to write the same code is:
        let tmp = Point { x: 1.0, y: 10.};
        let on_the_stack2 : &Point = &tmp;
        println!("{:?}, {:?}", tmp, on_the_stack2);
    }
    taking_its_address();
}
