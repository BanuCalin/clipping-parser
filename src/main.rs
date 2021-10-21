extern crate clap;

use clap::{Arg, App};
use std::fs::File;
// use std::fs::OpenOptions;
use std::io::{BufReader, Read};
// use regex::Regex;
use std::collections::HashMap;

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
        .get_matches();

    let mut buf = String::new();
    let mut authors_hm : HashMap<&str, HashMap<&str, Vec<(&str, &str)>>> = HashMap::new();

    if let Some(file_path) = matches.value_of("file") {
        println!("Input file: {}", file_path);
        let file = match File::open(file_path) {
            Ok(file) => file,
            Err(err) => {
                println!("Error opening input file: {}", err);
                std::process::exit(1);
            }
        };
        match BufReader::new(file).read_to_string(&mut buf) {
            Ok(_) => (),
            Err(err) => {
                println!("Error reading input file: {}", err);
                std::process::exit(1);
            }
        };

        for entry in buf.split("==========") {
            if entry.len() < 10 { continue; } // There might be a "\r\n" after the last "=" sequence
            let mut entry_lines = entry.lines().filter(|x| x.len() > 1);
            let title_author : Vec<&str> = entry_lines.next().unwrap().split(&['(', ')'][..]).collect();
            let title = title_author[0].trim();
            let author = title_author[1].trim();
            let entry_data = entry_lines.next().unwrap();
            let quote = entry_lines.next().unwrap();
            // authors_hm.entry(author).or_insert();
            match authors_hm.get_mut(author) {
                None => {
                    let mut quotes: Vec<(&str, &str)> = Vec::new();
                    quotes.push((quote, entry_data));
                    let mut books_hm: HashMap<&str, Vec<(&str, &str)>> = HashMap::new();
                    books_hm.insert(title, quotes);
                    authors_hm.insert(author, books_hm);
                }
                Some(books_hm) => {
                    match books_hm.get_mut(title) {
                        None => {
                            let mut quotes: Vec<(&str, &str)> = Vec::new();
                            quotes.push((quote, entry_data));
                            books_hm.insert(title, quotes);
                        }
                        Some(quotes) => {
                            quotes.push((quote, entry_data));
                        }
                    }
                }
            }
        }
    }
}
