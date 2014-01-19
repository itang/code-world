
use std::io::println;

struct Monster {
    health: int,
    attack: int
}

impl Monster {
  fn new(health: int, attack: int) -> Monster {
    Monster { health:health, attack:attack }
  }

  fn attack(&self) {
    println!("The monster attacks for {:d} damage.", self.attack)
  }

  fn count() {
    println("There are a bunch of monsters out tonight.");
  }
}

fn main() {
  let m = Monster { health: 10, attack: 20 };

  println(m.health.to_str());
  println(m.attack.to_str());
  println(format!("{:?}", m));

  m.attack();
  Monster::count();
  Monster::new(20, 40).attack();
}

