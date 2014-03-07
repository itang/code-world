/**
 * task.
 */

extern crate sync;

fn main() {
  // In general, all Rust code executes inside a task, including the `main` function.
  fn use_std_task_spawn() {
    fn print_message() {
      println!("I an running in a different task!");
    }

    spawn(print_message);
    spawn(proc() println!("I am also running in a different task"));
    spawn(proc() {
      println!("I too an running in a different task!");
    });

    spawn(proc() {
      println!("I too too an running in a different task!");
    });
  }
  use_std_task_spawn();

  fn capture() {
    fn generate_task_number() -> int { 0 }
    let child_task_number = generate_task_number();
    spawn(proc() {
      println!("I am child number {}", child_task_number);
    });
  }
  capture();

  fn pipes() {
    let (port, chan): (Port<int>, Chan<int>) = Chan::new();
    spawn(proc() {
      fn some_expensive_computation() -> int {
        100.0 as int
      }
      let result = some_expensive_computation();
      chan.send(result);
    });

    let ret = port.recv();
    println!("ret:{}", ret);
  }
  pipes();

  fn shared_chan() {
    let (port, chan) = Chan::new();
    let max = 10000;
    for inti_val in range(0u, max) {
      // Create a new channel handle to distribute to th echild task
      let child_chan = chan.clone();
      spawn(proc() {
        child_chan.send((|x| x)(inti_val));
      });
    }
    let mut result = 0u;
    for _ in range(0, max) {
      result += port.recv();
    }
    println!("result:{}", result);
  }
  shared_chan();

  fn advance() {
    let ports = std::vec::from_fn(3, |init_val| {
      let (port, chan) = Chan::new();
      spawn(proc() {
        println!("{:s}", init_val.to_str());
        chan.send((|x: uint| x + 1)(init_val));
      });
      port
    });

    let result = ports.iter().fold(0, |a, p| a + p.recv());
    println!("advance, result: {}", result);
  }
  advance();

  fn futures() {
    fn make_a_sandwich() {}

    fn fib(n: u64) -> u64 {
      // lengthy computation returning an uint
      match n {
        0|1 => n,
        _ => fib(n - 1) + fib(n - 2)
      }
    }

    let mut delayed_fib = sync::Future::spawn(proc() fib(30));
    make_a_sandwich();
    println!("fib(30) = {:?}", delayed_fib.get());

    fn partial_sum(start: uint) -> f64 {
      let m = 100000;
      let mut local_sum = 0f64;
      for num in range(start*m, (start+1)*m){
        local_sum += std::f64::pow(num as f64 + 1.0, -2.0);
      }
      local_sum
    }

    let mut futures = std::vec::from_fn(1000, |ind| sync::Future::spawn (proc(){ partial_sum(ind)}));

    let mut final_res = 0f64;
    for ft in futures.mut_iter() {
      final_res += ft.get();
    }
    println!("n^2/6 is not far from: {}", final_res);
  }
  futures();

  fn duplex_stream() {
    fn stringifier(channel: &sync::DuplexStream<~str,uint>){
      loop {
        let value = channel.recv();
        channel.send(value.to_str());
        if value == 0 { break; }
      }
    }

    let (from_child, to_child) = sync::DuplexStream::new();

    spawn(proc() {
      stringifier(&to_child);
    });
    from_child.send(22);
    assert!(from_child.recv() == ~"22");

    from_child.send(23);
    from_child.send(0);

    assert!(from_child.recv() == ~"23");
    assert!(from_child.recv() == ~"0");
  }
  duplex_stream();

  test_select();
}

fn test_select() {
  let (mut p1, c1) = Chan::new();
  let (mut p2, c2) = Chan::new();
  let sel = std::comm::Select::new();
  let mut h1 = sel.handle(&mut p1);
  let mut h2 = sel.handle(&mut p2);

  spawn(proc() {
    c2.send(2);
    c1.send(1);
  });
  let id = sel.wait();
  let data = if h1.id() ==id { h1.recv() } else { h2.recv() };
  println!("id: {}, data: {}", id, data);
}
