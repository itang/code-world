#[feature(managed_boxes)];
//use std::rc::Rc;

fn main() {
  //managed pointers
  let x: @int = @10;
  println(format!("{:d}", *x));
  println(plus_one_managed(x).to_str());

  //owned pointer
  let y: ~int = ~100;
  let y_clone = y.clone();
  println((*y).to_str());
  println(plus_one_owned(y).to_str());

  //borrowed pointers
  println(plus_one(x).to_str());
  println(plus_one(y_clone).to_str());
}

fn plus_one_managed(x: @int) -> int {
  *x + 1
}

fn plus_one_owned(x: ~int) -> int {
  *x + 1
}

fn plus_one(x: &int) -> int {
  *x + 1
}

