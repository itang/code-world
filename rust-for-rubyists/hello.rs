use std::io::println;

fn main() {
  println("Hello, world!");

  //parallel
  10.times(|| {
    do spawn {
      let greeting_message = "Hello?";
      println(greeting_message);
    }
  })
}

/**
 * Unit testing in Rust
 * rustc --test hello.rs
 * ./hello
 */
#[test]
fn this_tests_code() {
  let a = 1;
  if(a != 1){
    fail!("1==2")
  }
}

