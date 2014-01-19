extern mod hello;

use std::io::println;

fn main() {
  println("$ rustpkg install github.com/steveklabnik/hello");
  hello::world();
}