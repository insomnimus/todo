use super::*;
use crate::note::{self, Note};
use index::{Index, MinMax};
use query::Filter;

use clap::{App, Arg, ArgMatches};
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
    pub fn app() -> App<'static> {
        let app = App::new("list")
            .about("display notes")
            .visible_alias("l")
            .aliases(&["show", "display", "ls"]);

        let index = Arg::new("index")
            .short('i')
            .long("index")
            .about("expression to filter the result by index")
            .long_about(
                "an expression to filter the results by index
	syntax: start:end
	or: N
	start or end can be omitted
	last note has the index 0",
            )
            .validator(validate_index)
            .takes_value(true);

        let title = Arg::new("title")
            .about("filter results by their title")
            .long_about(
                "filter results by their title
	glob patterns are allowed and matching is case insensitive",
            );

        let lvl = Arg::new("lvl")
            .short('l')
            .long("level")
            .about("filter results by their importance level")
            .long_about(
                "filter results by their importance level
	syntax: MIN:MAX
	or: LVL
	MIN or MAX can be omitted",
            )
            .validator(validate_minmax)
            .takes_value(true);

        let tag= Arg::new("tag")
	.short('t')
	.long("tag")
	.takes_value(true)
	.about("comma separated list of tags to filter the results with")
	.long_about("comma separated list of tags to filter the results with. tags are case insensitive");

        app.arg(title).arg(index).arg(lvl).arg(tag)
    }

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
        let p = config::todo_path_checked()?;

        let notes: Vec<(usize, Note)> = note::get_notes(&p)?.into_iter().enumerate().collect();
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
