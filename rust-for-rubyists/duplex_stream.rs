extern mod extra;

use std::io::println;
use extra::comm::DuplexStream;

fn plus_one(channel: &DuplexStream<int, int>) {
  let mut value: int;
  loop{
    value = channel.recv();
    channel.send(value + 1);
    if value == 0 { break; }
  }
}

fn main() {
  let (from_child, to_child) = DuplexStream::new();

  do spawn {
    plus_one(&to_child);
  }

  from_child.try_send(1);
  from_child.try_send(100);
  from_child.send(1000);
  from_child.send(10000);
  from_child.send(0);

  5.times(|| {
    let answer = from_child.recv();
    println(answer.to_str());
  });
}