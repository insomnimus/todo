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

        let index = index.map(|idx| idx.calibrated(notes.len()));

        let (remaining, deleted): (Vec<_>, Vec<_>) =
            notes.into_iter().enumerate().partition(|(i, n)| {
                let not_in_range = if let Some(idx) = &index {
                    !idx.in_range(*i as isize)
                } else {
                    true
                };
                not_in_range && (filter.is_empty() || !filter.is_match(&n))
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
