extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("Rzip")
        .version("0.1.0")
        .author("Hastur (Lukas Brasseleur), Iankkkk (Yani Foughali), Lexinor (Alessandro Alterno)")
        .about("A zip & dezip tool made in Rust")
        .arg(Arg::with_name("action")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("What do you want to do ? 'zip' / 'unzip' "))
        .get_matches();
    let action = matches.value_of("action").unwrap();
    println!("{}", action);
}
