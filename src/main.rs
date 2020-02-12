#[macro_use]
extern crate clap;
extern crate zip;

use regex::Regex;
use clap::{App, Arg};

use std::io::{Seek, Write, Read};
use zip::result::ZipResult;
use zip::read::ZipFile;
use zip::read::ZipArchive;
use zip::write::{FileOptions, ZipWriter};

use std::fs::File;

// To fill my the files added to the zip
static FILE_CONTENTS: &'static [u8] = include_bytes!("../README.md");

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

    if action == "zip" {
        if matches.is_present("files") {
            let files: Vec<&str> = matches.values_of("files").unwrap().collect();
            zip(zip_file_name, files)
        }
        else{
            println!("Please enter a list of file")
        }        
    } else if action == "unzip" {
        unzip(zip_file_name)
    } else {
        println!("Please choose 'zip' or 'unzip' - ex : cargo run zip")
    }
}

fn zip(zip_file_name: &str, files: Vec<&str>) {
    let re = Regex::new(r"\.(zip|gz|tar|rar|7z)$").unwrap();
    if re.is_match(zip_file_name) {
        println!("You've choose to create a compressed file called : {}", zip_file_name);
        println!("number of files {}", files.len());
        
        let mut file = File::create(zip_file_name).expect("Couldn't create file");
        create_zip_archive(&mut file, files).expect("Couldn't create archive");
    }
    else {        
        println!("your file name needs to have one of these extensions");
        println!("zip|gz|tar|rar|7z, you've choose {}", zip_file_name)
    }
}

fn create_zip_archive<T: Seek + Write>(buf: &mut T, _files: Vec<&str>) -> ZipResult<()> {
    let mut writer = ZipWriter::new(buf);

    for file in _files {
        writer.start_file(file, FileOptions::default())?;
        writer.write(FILE_CONTENTS)?;        
    }
    writer.finish()?;
    Ok(())
}

fn unzip(zip_file_name: &str) {
    /* DEBUG PRINT
    println!("UNZIP");
    println!("zip_file_name -> {:?}", zip_file_name);
    */

    let re = Regex::new(r"\.(zip|gz|tar|rar|7z)$").unwrap();

    if re.is_match(zip_file_name) {
        println!("You've choose to decompress a file called : {}", zip_file_name);
        
        let mut file = File::open(zip_file_name).expect("Couldn't open file");
        let files = browse_zip_archive(&mut file, |f| {
            //let v: Vec<&str> = zip_file_name.split_terminator(".").collect();
            //let newString = ".".
            //fs::create_dir(v[1])?;
            //File::create("./test/{}",f.name())?;

            /* To Write a file
            let mut file = File::create(f.sanitized_name())?;
            file.write_all(mut buf: &[u8])
            */
            Ok(format!("Your zip file '{}' contains {}", zip_file_name,f.name()))
        });
        println!("{:?}", files);
    }
    else {        
        println!("Your file name needs to use one of these extensions");
        println!("zip|gz|tar|rar|7z, you've choose {}", zip_file_name)
    }
}

fn browse_zip_archive<T, F, U>(buf: &mut T, browse_func: F) -> ZipResult<Vec<U>> where T: Read + Seek, F: Fn(&ZipFile) -> ZipResult<U>
{
    let mut archive = ZipArchive::new(buf)?;
    (0..archive.len())
        .map(|i| archive.by_index(i).and_then(|file| browse_func(&file)))
        .collect()
}


