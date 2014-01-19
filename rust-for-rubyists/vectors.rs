use std::io::println;

fn main() {
  let your_favorite_numbers = ~[1,2,3];
  let my_favorite_numbers = ~[4,5,6];

  let our_favorite_numbers = your_favorite_numbers + my_favorite_numbers;

  println(format!("The third favorite number is {:d}.", our_favorite_numbers[2]));

  // mutability inheritance
  let a_vector = ~[1,2,3];
  let mut another_vector = ~[];
  another_vector.push_all(a_vector);
  another_vector[2] = 5;

  println(format!("The first number is {:d}.", another_vector[0]));
  for i in another_vector.iter() {
    println(i.to_str());
  }
}