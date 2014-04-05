#![allow(unused_unsafe)]

extern crate sync;

use sync::Arc;
use sync::RWLock;
use std::io::timer::Timer;

fn main() {
  println!("Arc...");
  /// Arc stands for 'atomically reference counted', and it's a way to share immutable data between multiple tasks.
  fn arc() {
    let numbers = [1, 2, 3];
    let numbers_arc = Arc::new(numbers);

    for num in range(0u, 3) {
      let (tx, rx) = channel();
      tx.send(numbers_arc.clone());

      spawn(proc() {
        let local_arc = rx.recv();
        let task_numbers = *local_arc;
        println!("{:d}", task_numbers[num]);
      });
    }
  }
  arc();

  println!("RWArc...");
  /// Rust provides a tools for shared mutable state: RWArc. This variant of an Arc allows the contents of the Arc to be mutated.
  fn rwarc() {
    let numbers = [1,2,3];
    let numbers_arc = Arc::new(RWLock::new(numbers));

    for num in range(0u, 3) {
      let (tx, rx) = channel();
      tx.send(numbers_arc.clone());

      spawn(proc() {
        let local_arc = rx.recv();
         let mut val = local_arc.write();
         val[num] +=1;
         println!("{:d}", local_arc.read()[num]);
      });
    }

    println!("rwarc:{:?}", *numbers_arc.read())
  }
  rwarc();

  /// within an `unsafe` block, Rust turns off many of its safety checks.
  /// If something bad happens to your program, you only have to audit what you've done inside unsafe, and not the entire program itself.
  /// interfacing with external code, such as doing FFI into a C library,
  /// performance(in certain cases) 
  /// to provide a safe abstraction around operations that normally would not be safe.
  fn the_unsafe() {
    unsafe {
    }
  }
  the_unsafe();

  arc_chan_tasks();

  let mut timer = Timer::new().unwrap();
  timer.sleep(1000);
}


fn arc_chan_tasks() {
    let s: ~str = ~"";
    let rwarc = Arc::new(RWLock::new(s));

    let mut i = 0;
    for _ in range(0, 100000) {
        let (tx, rx) = channel();
        tx.send((i,rwarc.clone()));
        i = i + 1;
        spawn(proc() {
           let (index, local_arc) = rx.recv();
           let mut val = local_arc.write();
           val.push_char((index % 128) as u8 as char);
        });
    }

    println!("arc_chan_tasks: {:?}",  *rwarc.read());
}