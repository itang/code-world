#[crate_id="main"];

extern crate getopts;
extern crate extra;

use std::os;

mod rusttime;

fn main() {
  let args = ~os::args();
  let opts = ~[
    getopts::optopt("p", "pattern", "date format pattern", "COLS"),
    getopts::optflag("v", "version", "display version information"),
    getopts::optflag("h", "help", "display this help text and exit")
  ];

  let matches = match getopts::getopts(args.tail(), opts) {
    Ok(m) => m,
    Err(e) => {
      error!("error: {:s}", e.to_err_msg());
      fail!()
    }
  };

  let progname = args[0].clone();
  let usage = getopts::usage("time now by rust.", opts);
  let mode = if matches.opt_present("help") {
    Help
  }else if matches.opt_present("version") {
    Version
  }else {
    DisplayNow
  };

  match mode {
    Version    => version(),
    Help       => help(progname, usage),
    DisplayNow => {
      let pattern = match matches.opt_str("pattern") {
        Some(s) => s,
        None    => rusttime::pattern_default()
      };
      now(pattern);
    }
  }
}

enum Mode {
  DisplayNow,
  Help,
  Version
}

fn now(p: &str) {
  println!("{:s}", rusttime::now(p));
}

fn help(progname: &str, usage: &str) {
  println!("Usage: {:s} [OPTION]", progname);
  println!("");
  println!("{}", usage);

  let msg = ~"";
  println!("{}", msg);
}

fn version() {
  println!("rusttime 1.0.0");
}
