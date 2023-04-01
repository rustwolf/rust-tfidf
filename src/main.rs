use std::collections::HashMap;
use std::default;
use std::fs::{DirEntry, File};
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

        if self.content[0].is_alphabetic() {
            let mut n = 0;

            while n < self.content.len() && self.content[n].is_alphabetic() {
                n += 1;
            }
            let token = &self.content[0..n];
            self.content = &self.content[n..];

            return Some(&token);
        }

        todo!();
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = &'a [char];

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

fn main() {
    let content = read_entire_xml_file("./docs.gl/el3/abs.xhtml");
    let data = content.unwrap().chars().collect::<Vec<_>>();
    let lexer = Lexer::new(&data);

    for token in lexer {
        println!("{token}", token = token.iter().collect::<String>());
    }

    // let _all_documents = HashMap::<String, HashMap<String, i32>>::new();

    // let dir_path = "docs.gl/gl4";
    // let dir = fs::read_dir(&dir_path).unwrap();
    // for file in dir {
    //     let path = file.unwrap().path();

    //     let path = path.display().to_string();

    //     let content = read_entire_xml_file(&path).unwrap();

    //     println!("{content}");
    // }
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
