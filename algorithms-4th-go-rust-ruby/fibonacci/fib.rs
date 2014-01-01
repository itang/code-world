fn fib(i: int) -> int {
  if i==0 || i ==1 { i }
  else { fib(i-1) + fib(i-2) }
}

fn main() {
  for i in range(0, 20) {
    println(format!("fib({:d})={:d}", i , fib(i)));
  }
}