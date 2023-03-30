use std::collections::HashMap;
use std::default;
use std::fs::{DirEntry, File};
use std::io::Result;
use std::path::Path;
use std::{fs, process::exit};
use xml::reader::EventReader;

fn main() {
    let _all_documents = HashMap::<String, HashMap<String, i32>>::new();

    let dir_path = "docs.gl/gl4";
    let dir = fs::read_dir(&dir_path).unwrap();
    for file in dir {
        let path = file.unwrap().path();

        let path = path.display().to_string();

        let content = read_entire_xml_file(&path).unwrap();

        println!("{content}");
    }
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
            }
            _ => {}
        }
    }

    Ok(content)
}
