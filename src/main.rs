extern crate clap;

use clap::{Arg, App};
use std::fs::File;
use std::io::{BufReader, Read, Error};

mod Parser {
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct ClipParser {
        hm: HashMap<String, HashMap<String, Vec<(String, String)>>>,
    }

    pub fn get_clip_parser() -> ClipParser {
        return ClipParser {
            hm: HashMap::new(),
        };
    }

    impl ClipParser {
        pub fn parse_file_string(&mut self, data: &str) {
            for entry in data.split("==========") {
                if entry.len() < 10 { continue; } // There might be a "\r\n" after the last "=" sequence
                let mut entry_lines = entry.lines().filter(|x| x.len() > 1);
                let title_author : Vec<&str> = entry_lines.next().unwrap().split(&['(', ')'][..]).collect();
                let title = String::from(title_author[0].trim());
                let author = String::from(title_author[1].trim());
                let entry_data = String::from(entry_lines.next().unwrap());
                let quote = String::from(entry_lines.next().unwrap());
                match self.hm.get_mut(&author) {
                    None => {
                        let mut quotes: Vec<(String, String)> = Vec::new();
                        quotes.push((quote, entry_data));
                        let mut books_hm: HashMap<String, Vec<(String, String)>> = HashMap::new();
                        books_hm.insert(title, quotes);
                        self.hm.insert(author, books_hm);
                    }
                    Some(books_hm) => {
                        match books_hm.get_mut(&title) {
                            None => {
                                let mut quotes: Vec<(String, String)> = Vec::new();
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

        pub fn get_authors(&self) -> Vec<&String> {
            return self.hm.keys().collect();
        }
    }
}

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
        .get_matches();

    let mut parser = Parser::get_clip_parser();
    if let Some(file_path) = matches.value_of("file") {
        println!("Input file: {}", file_path);
        match read_input_file(file_path) {
            Ok(buf) => parser.parse_file_string(&buf),
            Err(_) => std::process::exit(1),
        }
    }

    // #### Scratchpad
}
