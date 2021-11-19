mod Parser;

extern crate clap;

use clap::{Arg, App};
use std::fs::File;
use std::io::{BufReader, Read, Error, Write, BufWriter};
use std::ops::Add;

fn read_input_file(file_path: &str) -> Result<String, Error> {
    let mut buf = String::new();
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Error opening input file: {}", err);
            return Err(err);
        }
    };
    match BufReader::new(file).read_to_string(&mut buf) {
        Ok(_) => (),
        Err(err) => {
            println!("Error reading input file: {}", err);
            return Err(err)
        }
    };
    return Ok(buf);
}

fn main() {
    let matches = App::new("Kindle Clipping Parser")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .value_name("FILE PATH")
            .help("Set input file")
            .takes_value(true))
        .arg(Arg::with_name("authors")
            .short("a")
            .long("authors")
            .help("Prints all authors names")
            .requires("file"))
        .arg(Arg::with_name("author")
            .long("author")
            .help("Prints all titles for the given author")
            .requires("file")
            .takes_value(true))
        .arg(Arg::with_name("output")
            .short("o")
            .help("Generates output files for each author and title in the given path")
            .requires("file")
            .default_value("."))
        .get_matches();

    if let Some(file_path) = matches.value_of("file") {
        let mut parser = Parser::get_clip_parser();
        println!("Input file: {}", file_path);
        match read_input_file(file_path) {
            Ok(buf) => parser.parse_file_string(&buf),
            Err(_) => std::process::exit(1),
        }

        if matches.is_present("authors") {
            for author in parser.get_all_authors() {
                println!("{}", author);
            }
        }

        if let Some(author_search) = matches.value_of("author") {
            for author in parser.get_all_authors() {
                if author.to_uppercase().contains(&author_search.to_uppercase()) {
                    println!("{}", author);
                    for title in parser.get_all_titles(author) {
                        println!("\t{}", title);
                    }
                }
            }
        }

        if matches.is_present("output") {
            for author in parser.get_all_authors() {
                for title in parser.get_all_titles(author) {
                    let mut file_name = String::new();
                    let directory = matches.value_of("output").unwrap();
                    file_name = format!("{}/{} - {}", directory, author, title);
                    println!("{}", file_name);
                    let mut file = match File::create(file_name) {
                        Ok(file) => file,
                        Err(err) => {
                            println!("Error opening output file: {}", err);
                            std::process::exit(1);
                        }
                    };
                    let mut buf_writer = BufWriter::new(file);
                    for quote in parser.get_all_quotes(author, title) {
                        buf_writer.write(format!("{}\n\n", quote).as_bytes());
                    }
                }
            }

        }
    }
}
