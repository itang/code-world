extern mod hello;

fn main() {
  println("$ rustpkg install github.com/steveklabnik/hello");
  hello::world();
}