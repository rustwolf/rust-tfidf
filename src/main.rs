use std::collections::HashMap;
use std::default;
use std::fs::{DirEntry, File};
use std::hash::Hash;
use std::io::Result;
use std::path::Path;
use std::{fs, process::exit};
use xml::reader::EventReader;

#[derive(Debug)]
struct Lexer<'a> {
    content: &'a [char],
}
impl<'a> Lexer<'a> {
    fn new(content: &'a [char]) -> Self {
        Self { content: content }
    }

    fn trim_left(&mut self) {
        while self.content.len() > 0 && self.content[0].is_whitespace() {
            self.content = &self.content[1..];
        }
    }

    fn next_token(&mut self) -> Option<&'a [char]> {
        self.trim_left();

        if self.content.len() == 0 {
            return None;
        }

        if self.content[0].is_numeric() {
            let mut n = 0;
            while n < self.content.len() && self.content[n].is_numeric() {
                n += 1;
            }

            let token = &self.content[0..n];
            self.content = &self.content[n..];
            return Some(token);
        }

        if self.content[0].is_alphabetic() {
            let mut n = 0;

            while n < self.content.len() && self.content[n].is_alphabetic() {
                n += 1;
            }
            let token = &self.content[0..n];
            self.content = &self.content[n..];

            return Some(&token);
        }

        let token = &self.content[0..1];
        self.content = &self.content[1..];

        return Some(token);
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = &'a [char];

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

fn main() {
    let mut tf_by_file = HashMap::<String, HashMap<String, i32>>::new();

    let dir_path = "docs.gl/gl4";
    let dir = fs::read_dir(&dir_path).unwrap();
    for file in dir {
        let path = file.unwrap().path();

        let path = path.display().to_string();

        // let content = read_entire_xml_file(&path).unwrap();

        let (file, result) = get_tf_by_document(String::from(&path));

        tf_by_file.insert(file, result);
    }

    let json_file = File::create("result.json").unwrap();
    serde_json::to_writer(json_file, &tf_by_file).expect("We have a error");

    // println!("{tf_by_file:?}");
}

fn get_tf_by_document(file_path: String) -> (String, HashMap<String, i32>) {
    let content = read_entire_xml_file(&file_path);
    let data = content.unwrap().chars().collect::<Vec<_>>();
    let lexer = Lexer::new(&data);

    let mut tf = HashMap::<String, i32>::new();

    for token in lexer {
        let term = token
            .iter()
            .map(|t| t.to_ascii_uppercase())
            .collect::<String>();

        match tf.get(&term) {
            Some(count) => {
                tf.insert(term, count + 1);
            }
            None => {
                tf.insert(term, 1);
            }
        }
    }

    let mut stats = tf.iter().collect::<Vec<_>>();

    stats.sort_by_key(|(t, f)| {
        return *f;
    });

    stats.reverse();

    // println!("{stats:?}");

    let mut to_return = HashMap::<String, i32>::new();

    for (k, i) in stats.iter().take(10) {
        to_return.insert(k.to_string(), **i);
    }

    return (file_path, to_return);
}

fn index_document(document_content: &str) -> HashMap<String, i32> {
    todo!("Not implemented yet");
}

fn read_entire_xml_file(path: &str) -> Result<String> {
    let file = File::open(path).unwrap_or_else(|err| {
        println!("{}", err);
        exit(1)
    });

    let er = EventReader::new(file);

    let mut content = String::new();

    for event in er.into_iter() {
        let event = event.unwrap_or_else(|err| {
            println!("We have a problem reading this event");
            exit(1)
        });

        match event {
            xml::reader::XmlEvent::Characters(text) => {
                content.push_str(&text);
                content.push_str(" ");
            }
            _ => {}
        }
    }

    Ok(content)
}
