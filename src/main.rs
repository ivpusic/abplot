#![feature(phase)]
#[phase(plugin)]

extern crate regex_macros;
extern crate regex;
extern crate getopts;
extern crate serialize;
extern crate uuid;

use getopts::{optopt,optflag,getopts,short_usage};
use std::os;
use tester::ab::run;
use conf::parse;
use std::io::File;

mod plotter;
mod tester;
mod conf;
mod util;

fn main() {
    let args: Vec<String> = os::args();
    let opts = [
        optflag("h", "help", "print help"),
        optopt("c", "config", "JSON configuration file", "PATH")
    ];

    let matches = match getopts(args.tail(), opts) {
        Ok(m) => {m}
        Err(f) =>  {fail!(f.to_string())}
    };

    if matches.opt_present("h") {
        let help = short_usage("abplot", opts);
        println!("{}", help);
        return
    }

    let config: String = match matches.opt_str("c") {
        Some(config) => config,
        None => fail!("Please provide configuration file! You can use -c option")
    };

    let path = Path::new(config);
    let mut file = match File::open(&path) {
        Err(e) => fail!(e),
        Ok(file) => file
    };

    match file.read_to_string() {
        Err(f) => fail!(f),
        Ok(config_json)  => {
            let config = parse(config_json);
            util::mk_tmp_dir();
            run(config);
            util::rm_tmp_dir();
            println!("Images generated!");
        }
    };
}
