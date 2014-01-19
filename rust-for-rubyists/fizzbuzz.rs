use std::io::println;

fn main() {
  //100.times(|| {
   //println("num");
 //})
  for num in range(1, 101)  {
    //println(format!("{:d}", num));
    //println(num.to_str());
    /*
    let mut answer = "";
    if is_fifteen(num){
      answer = "FizzBuzz";
    }else if is_three(num) {
      answer = "Fizz";
    }else if is_five(num) {
      answer = "Buzz";
    }else {
      answer = "";
    }*/
    /*let answer =
    if is_fifteen(num){
      "FizzBuzz"
    } else if is_three(num) {
      "Fizz"
    } else if is_five(num) {
      "Buzz"
    } else {  
      "" 
    };*/

    let answer = if is_fifteen(num) { ~"FizzBuzz" }
    else if is_three(num) { ~"Fizz" }
    else if is_five(num) { ~"Buzz" }
    else {num.to_str() };
    println(format!("{:d}: {:s}", num, answer))
  }
}

fn is_three(num: int) -> bool {
  //return (num % 3 == 0);
  num % 3 == 0
}

fn is_five(num: int) -> bool {
  num % 5 == 0
}

fn is_fifteen(num: int) -> bool {
  is_three(num) && is_five(num)
}

#[test]
fn test_is_three() {
  if is_three(1) {
    fail!("One is not three");
  }

  if !is_three(3) {
    fail!(~"Three should be three");
  }
}

#[test]
fn test_is_five() {
  if is_three(1) {
    fail!("One is not five");
  }

  if !is_five(5) {
    fail!(~"Five should be five");
  }
}

#[test]
fn test_is_fifteen() {
  if is_fifteen(1) {
    fail!("One is not fifteen");
  }

  if !is_fifteen(15) {
    fail!(~"15 should be fifteen");
  }
}