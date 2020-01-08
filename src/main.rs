extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("Rzip")
        .version("0.1.0")
        .author("Hastur (Lukas Brasseleur), Iankkkk (Yani Foughali), Lexinor (Alessandro Alterno)")
        .about("A zip & dezip tool made in Rust")
        .arg(Arg::with_name("title")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("Name of the zip file "))
        .get_matches();
    let zip_name = matches.value_of("title").unwrap();
    println!("{}", zip_name);
}
