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
            .possible_value("unzip")
            .possible_value("zip")
            .help("Whether you want to 'zip' or 'unzip'"))
        .arg(Arg::with_name("zip_file_name")
            .required(true)
            .value_name("ZIP_FILE_NAME")
            .help("In case you choose 'zip', it will be the wanted name of your zip file. If you choose 'unzip' just need the zip file path to unzip"))
        .arg(Arg::with_name("files")
            .requires_if("action","zip")
            .value_name("FILE_LIST")
            .multiple(true)
            .help("Here will be the list of files you want to zip"))
        .get_matches();

    let action = matches.value_of("action").unwrap();
    let zip_file_name = matches.value_of("zip_file_name").unwrap();
    
    if let Some(files) = matches.values_of("files") {
        for file in files {
            println!("My file name is {:?}", file);
        }
    } 
       

    if action == "zip" {
        zip(zip_file_name)
    } else if action == "unzip" {
        unzip(zip_file_name)
    } else {
        print!("Please choose 'zip' or 'unzip' - ex : cargo run zip")
    }
}

fn zip(zip_file_name: &str) {
    println!("ZIP");
    println!("zip_file_name -> {:?}", zip_file_name);

    
}

fn unzip(zip_file_name: &str) {
    println!("UNZIP");
    println!("zip_file_name -> {:?}", zip_file_name);

    if zip_file_name.contains(".zip") {
        println!("Your file has a .zip extension");
        println!("UNZIPPING YOUR FILE");
    }
    else{
        println!("Your file doesn't have a .zip extension");
    }
}