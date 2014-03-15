fn main() {
  for _ in range(0, 5) {
    spawn(proc() { println!("Hello"); });
  }

  let (tx, rx) :(Sender<int>, Receiver<int>) = channel();
  
  spawn(proc() {
    tx.send(100); 
  });

  println!("{:d}", rx.recv());
}