use super::{
    index::{Index, MinMax},
    query::Filter,
};

use crate::{config::Config, note};

use clap::ArgMatches;
use glob::Pattern;

use std::error::Error;

#[derive(Debug)]
pub struct ListCommand {
    pub index: Index,
    pub title: Option<String>,
    pub lvl: Option<MinMax>,
    pub tags: Option<Vec<String>>,
}

impl Default for ListCommand {
    fn default() -> Self {
        Self {
            index: Index::Between((0, 5)),
            lvl: None,
            title: None,
            tags: None,
        }
    }
}

impl ListCommand {
    pub fn from_matches(m: &ArgMatches) -> Self {
        let title = m.value_of("title").map(String::from);
        let index = m
            .value_of("index")
            .map(|s| Index::parse(s).expect("internal error: Index::parse returned None"))
            .unwrap_or_default();

        let tags = m
            .value_of("tag")
            .map(|s| s.split(',').map(String::from).collect());
        let lvl = m
            .value_of("lvl")
            .map(|s| MinMax::parse(s).expect("internal error: MinMax::parse returned None"));

        Self {
            index,
            title,
            lvl,
            tags,
        }
    }

    pub fn run(self) -> Result<(), Box<dyn Error>> {
        let Self {
            mut index,
            title,
            lvl,
            tags,
        } = self;
        let title = match title {
            Some(t) => Some(Pattern::new(&t)?),
            _ => None,
        };
        let filter = Filter { title, lvl, tags };

        let c = Config::get()?;
        let notes: Vec<_> = note::get_notes(&c.todos_file)?
            .into_iter()
            .enumerate()
            .collect();
        index.calibrate(notes.len());
        let mut filtered: Vec<_> = index
            .slice(&notes)
            .unwrap_or_default()
            .iter()
            .filter(|(_, n)| filter.is_match(n))
            .collect();

        if index.is_reversed() {
            filtered = filtered.into_iter().rev().collect();
        }

        note::print_notes_enumerated(&filtered[..]);
        Ok(())
    }
}
