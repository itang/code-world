/**
 * task.
 */

extern mod extra;

use std::io::println;

fn main() {
  // In general, all Rust code executes inside a task, including the `main` function.
  fn use_std_task_spawn() {
    fn print_message() {
      println("I an running in a different task!");
    }

    spawn(print_message);
    spawn(proc() println("I am also running in a different task"));
    spawn(proc() {
      println("I too an running in a different task!");
    });

    spawn(proc() {
      println("I too too an running in a different task!");
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
    let (port, chan) = SharedChan::new();
    let MAX = 10000;
    for inti_val in range(0u, MAX) {
      // Create a new channel handle to distribute to th echild task
      let child_chan = chan.clone();
      spawn(proc() {
        child_chan.send((|x| x)(inti_val));
      });
    }
    let mut result = 0u;
    for _ in range(0, MAX) {
      result += port.recv();
    }
    println!("result:{}", result);
  }
  shared_chan();

  fn advance() {
    let ports = std::vec::from_fn(3, |init_val| {
      let (port, chan) = Chan::new();
      spawn(proc() {
        println(init_val.to_str());
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

    let mut delayed_fib = extra::future::Future::spawn(proc() fib(30));
    make_a_sandwich();
    println!("fib(30) = {:?}", delayed_fib.get());

    fn partial_sum(start: uint) -> f64 {
      let M = 100000;
      let mut local_sum = 0f64;
      for num in range(start*M, (start+1)*M){
        local_sum += std::f64::pow(num as f64 + 1.0, -2.0);
      }
      local_sum
    }

    let mut futures = std::vec::from_fn(1000, |ind| extra::future::Future::spawn (proc(){ partial_sum(ind)}));

    let mut final_res = 0f64;
    for ft in futures.mut_iter() {
      final_res += ft.get();
    }
    println!("n^2/6 is not far from: {}", final_res);
  }
  futures();

  fn duplex_stream() {
    fn stringifier(channel: &extra::comm::DuplexStream<~str,uint>){
      loop {
        let value = channel.recv();
        channel.send(value.to_str());
        if value == 0 { break; }
      }
    }

    let (from_child, to_child) = extra::comm::DuplexStream::new();

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
}
