extern crate kokmath;

extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Kokmath")
        .version("0.1.1")
        .author("Creative GP <social@cretgp.com>")
        .about("Do awesome things")
        .arg(Arg::with_name("INPUT")
             .help("Input string to throw to this program.")
             .required(true)
             .index(1))
        .get_matches();

    let input = matches.value_of("INPUT").unwrap();
    println!("{}", input);
}
