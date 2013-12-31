fn message(i: int) {
  match i {
    1 => println("ONE!"),
    2 => println("Two is a prime."),
    3 => println("THERE!"),
    _ => println("no idea what that is, boss")
  }
}

fn main() {
  for i in range(1, 5) {
    message(i);
  }
}