use super::index::{Index, MinMax};
use super::query::Filter;
use crate::config;
use crate::note::{self, Notes};

use clap::ArgMatches;
use glob::Pattern;

use std::error::Error;

#[derive(Debug)]
pub struct RemoveCommand {
    pub index: Option<Index>,
    pub lvl: Option<MinMax>,
    pub title: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl RemoveCommand {
    pub fn from_matches(m: &ArgMatches) -> Self {
        let title = m.value_of("title").map(String::from);
        let lvl = m
            .value_of("lvl")
            .map(|s| MinMax::parse(s).expect("internal error: MinMax::parse returned none"));
        let index = m
            .value_of("index")
            .map(|s| Index::parse(s).expect("internal error: Index::parse returned none"));
        let tags = m
            .value_of("tag")
            .map(|s| s.split(',').map(String::from).collect::<Vec<_>>());
        Self {
            index,
            lvl,
            title,
            tags,
        }
    }

    pub fn run(self) -> Result<(), Box<dyn Error>> {
        let Self {
            index,
            lvl,
            title,
            tags,
        } = self;

        let title = match title {
            Some(t) => Some(Pattern::new(&t)?),
            _ => None,
        };
        let filter = Filter { title, lvl, tags };
        let p = config::todo_path_checked()?;
        let notes = note::get_notes(&p)?;
        let total = notes.len();
        if total == 0 {
            println!("you have no todos yet");
            return Ok(());
        }
        let index = index.map(|idx| idx.calibrated(total));
        let notes: Vec<_> = notes
            .into_iter()
            .enumerate()
            .filter(|(i, _)| {
                if let Some(idx) = &index {
                    !idx.in_range(*i as isize)
                } else {
                    true
                }
            })
            .map(|(_, n)| n)
            .filter(|n| filter.is_empty() || !filter.is_match(&n))
            .collect();

        let remaining = notes.len();
        if remaining == total {
            println!("no match, nothing to do");
            Ok(())
        } else {
            let notes = Notes::new(notes);
            //note::save_notes(&p, &notes).map(|_| {
            notes.save_to(&p).map(|_| {
                if remaining + 1 == total {
                    println!("deleted 1 note");
                } else {
                    println!("deleted {} notes", total - remaining);
                }
            })
        }
    }
}
