use index::{Index, MinMax};
use crate::note::{self, Note};
use super::*;

use clap::{App, Arg, ArgMatches};
use glob::{Pattern, MatchOptions};

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
			.visible_alias("ls")
			.aliases(&["show", "display"]);

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
			
		let tags = m.value_of("tag").map(|s| s.split(',').map(String::from).collect());
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
	
	pub fn run(&self) -> Result<(), Box<dyn Error>> {
		let p= config:: todo_path_checked()?;
		let notes: Vec<(usize, Note)>= note::get_notes(&p)?.into_iter().enumerate().collect();
		let mut filtered= self.index.slice(&notes).unwrap_or_default().into_iter();
		if let Some(lvl) = self.lvl {
			filtered= filtered.filter(|(_, n)| lvl.in_range(n.lvl.unwrap_or_default()));
		}
		if self.index.is_reversed() {
			filtered= filtered.rev();
		}
		if let Some(t) = self.title{
			const OPT: MatchOptions= MatchOptions{
				case_sensitive: false,
				require_literal_separator: false,
				require_literal_leading_dot: false,
			};
			let pattern= Pattern::new(&t)?;
			filtered= filtered.filter(|(_, n)| pattern.matches_with(&n.title, OPT));
		}
	
	if let Some(tags) = self.tags {
		filtered= filtered.filter(|(_, n)| {
			if let Some(note_tags)= n.tags{
				note_tags.iter().any(|&tag| tags.iter().any(|&s| s==tag))
			}else{
				false
			}
		});
	}
	
	note::print_notes_enumerated(filtered);
	Ok(())
	}
}
