fn print_vec(v: &[int]) {
  for i in v.iter(){
    println(i.to_str())
  }
}

fn print_vec_str(v: &[~str]) {
  for i in v.iter() {
    println(*i)
  }
}

fn print_vec_generics<T: ToStr>(v: &[T]) {
  /*v.iter().advance(|x| {
    println(format!("{:?}", *x));
    true
  });*/
  v.iter().advance(|x| {
    println((*x).to_str());
    true
  });
}

fn main() {
  let vec = [1,2,3];

  print_vec(vec);

  let str_vec = [~"hey", ~"there", ~"yo"];
  print_vec_str(str_vec);

  print_vec_generics(vec);
  print_vec_generics(str_vec);
}