#[macro_use]
extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("Rzip")
        .version(crate_version!()) // take the cargo.toml version
        .author("Hastur (Lukas Brasseleur), Iankkkk (Yani Foughali), Lexinor (Alessandro Alterno)")
        .about("A zip & dezip tool made in Rust")
        .arg(Arg::with_name("action")
            .required(true)
            .value_name("ACTION")
            .multiple(false)
            .number_of_values(1)
            .help("Whether you want to 'zip' or 'unzip'"))
        .arg(Arg::with_name("zip_file_name")
            .required(true)
            .multiple(false)
            .value_name("ZIP_FILE_NAME")
            .help("In case you choose 'zip', it will be the wanted name of your zip file. If you choose 'unzip' just need the zip file path to unzip"))
        .arg(Arg::with_name("FILES")
            .required_if("action","zip")
            .value_name("FILE_LIST")
            .multiple(true)
            .help("Here will be the list of files you want to zip"))
        .get_matches();

    let action = matches.value_of("action").unwrap();
    let zip_file_name = matches.value_of("zip_file_name").unwrap();
    let _files: Vec<&str> = matches.values_of("files").unwrap().collect();

    if action == "zip" {
        zip(action, zip_file_name)
    } else if action == "unzip" {
        unzip();
    } else {
        print!("Please choose 'zip' or 'unzip' - ex : cargo run zip")
    }
}

fn zip(action: &str, zip_file_name: &str) {
    println!("action -> {:?} -- zip_file_name -> {:?}", action, zip_file_name);
    println!("Let's create a zip")
}

fn unzip() {
    println!("Let's unzip a file")
}