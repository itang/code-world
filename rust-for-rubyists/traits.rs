trait Monster {
  fn attack(&self);
}

struct IndustrialRaverMonkey {
  strength: int
}

impl Monster for IndustrialRaverMonkey {
  fn attack(&self) {
     println(format!("The monkey attacks for {:d}.", self.strength))
  }
}

impl Monster for int {
  fn attack(&self) {
    println(format!("The int attacks for {:d}.", *self))
  }
}

fn main() {
  let monkey = IndustrialRaverMonkey{strength:32};

  monkey.attack();

  let i = 10;
  i.attack();
}