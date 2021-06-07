use crate::{
    config,
    note::{self, Note, Notes},
};

use clap::ArgMatches;

use std::error::Error;

#[derive(Debug)]
pub struct NewCommand {
    pub title: String,
    pub body: String,
    pub lvl: Option<u8>,
    pub tags: Option<Vec<String>>,
}

impl NewCommand {
    pub fn from_matches(m: &ArgMatches) -> Self {
        let title = m.value_of("title").map(String::from).unwrap();
        let body = m.value_of("body").map(String::from).unwrap();
        let tags = m
            .value_of("tag")
            .map(|s| s.split(',').map(String::from).collect::<Vec<_>>());

        let lvl = m.value_of("lvl").map(|s| s.parse::<u8>().unwrap());

        Self {
            title,
            body,
            lvl,
            tags,
        }
    }

    pub fn run(self) -> Result<(), Box<dyn Error>> {
        let p = config::todo_path_checked()?;
        let mut notes = note::get_notes(&p)?;
        let title = self.title.clone();
        let n = Note {
            title: self.title,
            body: self.body,
            lvl: self.lvl,
            tags: self.tags,
        };
        notes.insert(0, n);
        let notes = Notes::new(notes);
        notes.save_to(&p)?;
        //note::save_notes(&p, &notes)?;
        println!("saved {}", title);
        Ok(())
    }
}
