extern crate clap;

use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("Rzip")
        .version("0.1.0")
        .author("Hastur (Lukas Brasseleur), Iankkkk (Yani Foughali), Lexinor (Alessandro Alterno)")
        .about("A zip & dezip tool made in Rust")
//        .subcommand(
//            SubCommand::with_name("zip")
                .arg(Arg::with_name("action")
                    .required(true)
                    .multiple(true)
                    .number_of_values(1),
//            ),
    )
    .get_matches();
    let action = matches.value_of("action").unwrap();

    if action == "zip" {
        zip();
    } else {
        unzip();
    }
}

fn zip() {
    println!("Do a zip");
}

fn unzip() {
    println!("Don't do a zip");
}
