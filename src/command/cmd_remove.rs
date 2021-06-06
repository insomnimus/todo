use super::*;
use crate::note::{self, Note};
use index::{Index, MinMax};

use clap::{App, AppSettings, Arg, ArgMatches};
use glob::{MatchOptions, Pattern};

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

	pub fn run(&self) -> Result<(), Box<dyn Error>> {
		let p = config::todo_path_checked()?;
		let mut notes = note::get_notes(&p)?;
		let total = notes.len();
		if total == 0 {
			println!("you have no todos yet");
			return Ok(());
		}

		if let Some(&idx) = self.index {
			idx.exclude(&mut notes);
		}
		let mut filtered = notes.into_iter();

		if let Some(&lvl) = self.lvl {
			filtered = filtered.filter(|&n| !lvl.in_range(n.lvl.or_default()));
		}

		if let Some(t) = self.title {
			const OPT: MatchOptions = MatchOptions {
				case_sensitive: false,
				require_literal_separator: false,
				require_literal_leading_dot: false,
			};
			let pattern = Pattern::new(&t)?;
			filtered = filtered.filter(|(_, n)| !pattern.matches_with(&n.title, OPT));
		}

		if let Some(tags) = self.tags {
			filtered = filtered.filter(|(_, n)| !{
				if let Some(note_tags) = n.tags {
					note_tags.iter().any(|&tag| tags.iter().any(|&s| s == tag))
				} else {
					false
				}
			});
		}

		let notes: Vec<Note> = filtered.collect();
		let remaining = notes.len();
		if remaining == total {
			println!("no match, nothing to do");
			Ok(())
		} else {
			note::save_notes(&p, &notes).and_then(|_| {
				if remaining + 1 == total {
					println!("deleted 1 note");
				} else {
					println!("deleted {} notes", total - remaining);
				}
			})
		}
	}
}
