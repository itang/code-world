fn main() {
  5.times (|| {
    do spawn { println("Hello"); }
  });

  let (port, chan) :(Port<int>, Chan<int>) = Chan::new();
  
  do spawn || {
    chan.send(100); 
  }

  println(port.recv().to_str());
}