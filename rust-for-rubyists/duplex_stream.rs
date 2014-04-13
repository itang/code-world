extern crate sync;

use sync::DuplexStream;

fn plus_one(channel: &DuplexStream<int, int>) {
  let mut value: int;
  loop{
    value = channel.recv();
    channel.send(value + 1);
    if value == 0 { break; }
  }
}

fn main() {
  let (from_child, to_child) = sync::duplex();

  spawn(proc() {
    plus_one(&to_child);
  });

  let _ = from_child.send_opt(1);
  let _ = from_child.send_opt(100);
  from_child.send(1000);
  from_child.send(10000);
  from_child.send(0);

  for _ in range(0, 5) {
    let answer = from_child.recv();
    println!("{:d}", answer);
  }
}