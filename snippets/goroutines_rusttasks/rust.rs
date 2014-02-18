use private::times;
use range_zero = private::range_with_start_zero;

pub fn par_sum(to: int) -> int {
    let (port, chan) = Chan::new();
    times(to, |i| {
      let c = chan.clone();
      spawn(proc() { c.send(i); });
    });
    range_zero(to).fold(0, |s, _|  s + port.recv() )
}

#[cfg(not(test))]
fn main() {
    let N = 100_000;
    println!("par_sum({:d}): {:d}", N, par_sum(N));
}

///////////////////////////////////////////////////

mod private {
    use std::ops::Add;
    use std::cmp::Ord;
    use std::clone::Clone;
    use std::num::{One, Zero};
    use std::default::Default;
    use std::iter::Range;

    pub fn times(to: int, f: |x:int|) {
        for i in range(0, to) { f(i); }
    }

    pub fn range_with_start_zero<A: Add<A, A> + Ord + Clone + One + Default + Zero>(stop: A) -> Range<A> {
        range(Zero::zero(), stop)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sum() {
        assert_eq!(4999950000, ::par_sum(100000));
    }
}
