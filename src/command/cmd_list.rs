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
    pub filter: Filter,
}

impl Default for ListCommand {
    fn default() -> Self {
        Self {
            index: Index::Between((0, 5)),
            filter: Filter::default(),
        }
    }
}

impl ListCommand {
    pub fn from_matches(m: &ArgMatches) -> Self {
        let titles = m.values_of("title").map(|i| {
            i.map(|s| {
                Pattern::new(s).unwrap_or_else(|e| {
                    panic!("invalid glob pattern {}: {:?}", s, e);
                })
            })
            .collect::<Vec<_>>()
        });

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
            filter: Filter { titles, lvl, tags },
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let c = Config::get()?;
        let notes: Vec<_> = note::get_notes(&c.todos_file)?
            .into_iter()
            .enumerate()
            .collect();
        // self.index.calibrate(notes.len());

        let mut filtered: Vec<_> = self
            .index
            .slice(&notes)
            .unwrap_or_default()
            .iter()
            .filter(|(_, n)| self.filter.is_match(n))
            .collect();

        if self.index.is_reversed() {
            filtered = filtered.into_iter().rev().collect();
        }

        note::print_notes_enumerated(&filtered[..]);
        Ok(())
    }
}
