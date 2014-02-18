#[allow(dead_code)];

///
/// rustime.
///

extern crate extra;

use extra::time;

pub static PATTERN_DEFAULT: &'static str = "%Y-%m-%d %H:%M:%S";

pub fn now_default() -> ~str {
  now(PATTERN_DEFAULT)
}

pub fn now(pattern: &str) -> ~str {
  let now = time::now();
  now.strftime(pattern)
}

pub fn pattern_default() -> ~str {
  PATTERN_DEFAULT.to_owned()
}
