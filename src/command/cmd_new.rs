use crate::{
    config::Config,
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
        let c = Config::get()?;
        let mut notes = note::get_notes(&c.todos_file)?;

        if let Err(e) = c.hooks.run_pre_new() {
            match c.abort_on_hook_error {
                Some(true) | None => return Err(Box::new(e)),
                Some(false) => println!("pre-new hook error: {:?}", e),
            };
        }

        let title = self.title.clone();
        let n = Note {
            title: self.title,
            body: self.body,
            lvl: self.lvl,
            tags: self.tags,
        };
        notes.insert(0, n);
        let notes = Notes::new(notes);
        notes.save_to(&c.todos_file)?;
        println!("saved {}", title);

        c.hooks.run_post_new()?;
        Ok(())
    }
}
