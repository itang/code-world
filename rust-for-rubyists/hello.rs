use std::io::println;

fn main() {
  println("Hello, world!");

  //parallel
  for _ in range(0, 10) {
    spawn(proc() {
      let greeting_message = "Hello?";
      println(greeting_message);
    });
  }
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

