#[allow(unused_unsafe)];

extern mod sync;

use sync::Arc;
use sync::RWArc;

fn main() {
  println!("Arc...");
  /// Arc stands for 'atomically reference counted', and it's a way to share immutable data between multiple tasks.
  fn arc() {
    let numbers = [1, 2, 3];
    let numbers_arc = Arc::new(numbers);

    for num in range(0, 3) {
      let (port, chan) = Chan::new();
      chan.send(numbers_arc.clone());

      spawn(proc() {
        let local_arc = port.recv();
        let task_numbers = local_arc.get();
        println!("{:d}", task_numbers[num]);
      });
    }
  }
  arc();

  println!("RWArc...");
  /// Rust provides a tools for shared mutable state: RWArc. This variant of an Arc allows the contents of the Arc to be mutated.
  fn rwarc() {
    let numbers = [1,2,3];
    let numbers_arc = RWArc::new(numbers);

    for num in range(0, 3) {
      let (port, chan) = Chan::new();
      chan.send(numbers_arc.clone());

      spawn(proc() {
        let local_arc = port.recv();
        local_arc.write(|nums| nums[num] += 1 );
        local_arc.read(|nums| println!("{:d}", nums[num]) );
      });
    }

    numbers_arc.read(|nums| { println!("{:?}", nums); });
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
}


fn arc_chan_tasks() {
    let s: ~str = ~"";
    let rwarc = RWArc::new(s);

    let mut i = 0;
    for _ in range(0, 100000) {
        let (p, c) = Chan::new();
        c.send((i,rwarc.clone()));
        i = i + 1;
        spawn(proc() {
           let (index, local_arc) = p.recv();
            local_arc.write(|it| {
                it.push_char( (index % 128) as u8 as char);
            });
        });
    }

    rwarc.read(|it|{
        println!("{:?}", it);
    });
}