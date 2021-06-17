use super::{
    index::{Index, MinMax},
    query::Filter,
};

use crate::{
    config::Config,
    note::{self, Notes},
};

use clap::ArgMatches;
use glob::Pattern;

use std::error::Error;

#[derive(Debug)]
pub struct RemoveCommand {
    pub index: Option<Index>,
    pub filter: Filter,
}

impl RemoveCommand {
    pub fn from_matches(m: &ArgMatches) -> Self {
        let titles = m.values_of("title").map(|i| {
            i.map(|s| {
                Pattern::new(s).unwrap_or_else(|e| {
                    panic!("invalid glob pattern {}: {:?}", s, e);
                })
            })
            .collect::<Vec<_>>()
        });

        let lvl = m
            .value_of("lvl")
            .map(|s| MinMax::parse(s).expect("internal error: MinMax::parse returned none"));
        let index = m
            .value_of("index")
            .map(|s| Index::parse(s).expect("internal error: Index::parse returned none"));
        let tags = m
            .values_of("tag")
            .map(|i| i.map(String::from).collect::<Vec<_>>());
        Self {
            index,
            filter: Filter { titles, lvl, tags },
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let c = Config::get()?;

        if let Err(e) = c.hooks.run_pre_remove() {
            match c.abort_on_hook_error {
                Some(true) | None => return Err(Box::new(e)),
                Some(false) => println!("pre-remove hook error: {:?}", e),
            };
        }

        let notes = note::get_notes(&c.todos_file)?;

        if notes.is_empty() {
            println!("you have no todos");
            return Ok(());
        }
        if let Some(i) = self.index.as_mut() {
            i.calibrate(notes.len());
        }

        let (remaining, deleted): (Vec<_>, Vec<_>) =
            notes.into_iter().enumerate().partition(|(i, n)| {
                let not_in_range = if let Some(idx) = &self.index {
                    !idx.in_range(*i as isize)
                } else {
                    true
                };
                not_in_range && (self.filter.is_empty() || !self.filter.is_match(n))
            });

        if deleted.is_empty() {
            println!("no match, nothing to do");
            return Ok(());
        }
        let notes = Notes::new(remaining.into_iter().map(|(_, n)| n).collect());

        notes.save_to(&c.todos_file)?;

        if deleted.len() == 1 {
            println!("deleted 1 note:");
        } else {
            println!("deleted {} notes:", deleted.len());
        }
        for (_, n) in &deleted {
            println!("-  {}", n.title);
        }

        c.hooks.run_post_remove().map_err(Box::new)?;
        Ok(())
    }
}
