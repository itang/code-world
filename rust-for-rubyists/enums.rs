enum Monster {
  ScubaArgentine(int, int, int, int),
  IndustrialRaverMonkey(int, int, int, int)
}

impl Monster {
  fn attack(&self) {
    match *self {
      ScubaArgentine(_l,_s,_c,w) => println!("The monster attacks for {:d} damage.", w),
      IndustrialRaverMonkey(_l, _s, _c, w) => println!("The monster attacks for {:d} damage.", w)
    }
  }
}

fn main() {
  let irm: Monster = IndustrialRaverMonkey(46, 35, 91, 2);
  irm.attack();
}
