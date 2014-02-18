fn main() {
  for _ in range(0, 5) {
    spawn(proc() { println!("Hello"); });
  }

  let (port, chan) :(Port<int>, Chan<int>) = Chan::new();
  
  spawn(proc() {
    chan.send(100); 
  });

  println!("{:d}", port.recv());
}