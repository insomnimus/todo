use super::*;
use crate::note::{self, Notes};
use index::{Index, MinMax};
use query::Filter;

use clap::{App, AppSettings, Arg, ArgMatches};
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
    pub fn app() -> App<'static> {
        let app = App::new("remove")
            .visible_alias("rm")
            .aliases(&["del", "delete"])
            .about("remove notes")
            .setting(AppSettings::ArgRequiredElseHelp);

        let title = Arg::new("title")
            .about("a glob pattern matching the note title")
            .long_about("a glob pattern matching the note title. matching is case insensitive");

        let index = Arg::new("index")
            .short('i')
            .long("index")
            .about("index of the note to remove")
            .long_about(
                "index of the note to remove
	syntax: START:END
	or N
	START or END can be omitted
	negative numbers are allowed (will count from the end of the list)
	the newest note will be index 0",
            )
            .takes_value(true)
            .validator(validate_index);

        let lvl = Arg::new("lvl")
            .short('l')
            .long("level")
            .takes_value(true)
            .about("remove notes matching the importance level")
            .long_about(
                "remove notes matching the importance level
	syntax: MIN:MAX or LVL
	MIN or MAX can be omitted",
            )
            .validator(validate_minmax);

        let tag= Arg::new("tag")
	.short('t')
	.long("tag")
	.takes_value(true)
	.about("space separated list of tags to remove notes having any of them")
	.about("space separated list of tags to remove notes having any of them. tags are case insensitive");

        app.arg(title).arg(index).arg(lvl).arg(tag)
    }

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
                    false
                }
            })
            .map(|(_, n)| n)
            .filter(|n| !filter.is_match(&n))
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
