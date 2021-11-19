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
    fn insert_quote(&mut self, author: &str, title: &str, quote: &str, entry_data: &str) {
        match self.hm.get_mut(author) {
            None => {
                let mut quotes: Vec<(String, String)> = Vec::new();
                quotes.push((quote.to_string(), entry_data.to_string()));
                let mut books_hm: HashMap<String, Vec<(String, String)>> = HashMap::new();
                books_hm.insert(title.to_string(), quotes);
                self.hm.insert(author.to_string(), books_hm);
            }
            Some(books_hm) => {
                match books_hm.get_mut(title) {
                    None => {
                        let mut quotes: Vec<(String, String)> = Vec::new();
                        quotes.push((quote.to_string(),entry_data.to_string()));
                        books_hm.insert(title.to_string(), quotes);
                    }
                    Some(quotes) => {
                        quotes.push((quote.to_string(), entry_data.to_string()));
                    }
                }
            }
        }
    }

    pub fn parse_file_string(&mut self, data: &str) {
        for entry in data.split("==========") {
            if entry.len() < 10 { continue; } // There might be a "\r\n" after the last "=" sequence
            let mut entry_lines = entry.lines().filter(|x| x.len() > 1);
            let title_author : Vec<&str> = entry_lines.next().unwrap().split(&['(', ')'][..]).collect();
            let title = title_author[0].trim();
            let author = title_author[1].trim();
            let entry_data = entry_lines.next().unwrap();
            let quote = entry_lines.next().unwrap();
            self.insert_quote( author, title, quote, entry_data);
        }
    }

    pub fn get_all_authors(&self) -> Vec<&str> {
        return self.hm.keys().map(|s| s.as_str()).collect();
    }

    pub fn get_all_titles(&self, author: &str) -> Vec<&str> {
        match self.hm.get(author) {
            None => Vec::new(),
            Some(books_hm) => books_hm.keys().map(|s| s.as_str()).collect(),
        }
    }

    pub fn get_all_quotes(&self, author: &str, title: &str) -> Vec<&str> {
        match self.hm.get(author) {
            None => Vec::new(),
            Some(books_hm) => {
                match books_hm.get(title) {
                    None => Vec::new(),
                    Some(quotes_vec) => quotes_vec.iter().map(|x| x.0.as_str()).collect(),
                }
            },
        }
    }
}

#[cfg(test)]
mod parser_test {
    use super::*;
    static TEST_INPUT: &str = "title0 (author0)\ndata0\n\nquote0\n==========\n\
                               title1 (author1)\ndata1\n\nquote1\n==========\n\
                               title2 (author1)\ndata2\n\nquote2\n==========\n\
                               title3 (author2)\ndata3\n\nquote3\n==========\n\
                               title0 (author0)\ndata3\n\nquote4\n==========\n\
                               title9 (author0)\ndata0\n\nquote5\n==========\n";

    #[test]
    fn test_get_all_authors() {
        let mut parser = get_clip_parser();
        assert_eq!(parser.get_all_authors().len(), 0);
        parser.parse_file_string(TEST_INPUT);
        let author_list = parser.get_all_authors();
        assert_eq!(author_list.len(), 3);
        assert_eq!(author_list.contains(&"author0"), true);
        assert_eq!(author_list.contains(&"author1"), true);
        assert_eq!(author_list.contains(&"author3"), false);
    }

    #[test]
    fn test_get_all_titles() {
        let mut parser = get_clip_parser();
        assert_eq!(parser.get_all_titles("author0").len(), 0);
        parser.parse_file_string(TEST_INPUT);
        let titles_list = parser.get_all_titles("author0");
        assert_eq!(titles_list.len(), 2);
        assert_eq!(titles_list.contains(&"title0"), true);
        assert_eq!(titles_list.contains(&"title9"), true);
        let titles_list = parser.get_all_titles("author1");
        assert_eq!(titles_list.len(), 2);
        assert_eq!(titles_list.contains(&"title1"), true);
        assert_eq!(titles_list.contains(&"title2"), true);
        let titles_list = parser.get_all_titles("author2");
        assert_eq!(titles_list.len(), 1);
        assert_eq!(titles_list.contains(&"title3"), true);
        let titles_list = parser.get_all_titles("author3");
        assert_eq!(titles_list.len(), 0);
    }

    #[test]
    fn test_get_all_quotes() {
        let mut parser = get_clip_parser();
        assert_eq!(parser.get_all_quotes("author0", "title0").len(), 0);
        parser.parse_file_string(TEST_INPUT);
        let quotes_list = parser.get_all_quotes("author0", "title0");
        assert_eq!(quotes_list.len(), 2);
        assert_eq!(quotes_list.contains(&"quote0"), true);
        assert_eq!(quotes_list.contains(&"quote4"), true);
        let quotes_list = parser.get_all_quotes("author0", "title9");
        assert_eq!(quotes_list.len(), 1);
        assert_eq!(quotes_list.contains(&"quote5"), true);
    }
}