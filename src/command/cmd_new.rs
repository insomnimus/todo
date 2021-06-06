use crate::{
	config,
	note::{self, Note},
};

use clap::{App, Arg, ArgMatches};

use std::error::Error;

#[derive(Debug)]
pub struct NewCommand {
	pub title: String,
	pub body: String,
	pub lvl: Option<u8>,
	pub tags: Option<Vec<String>>,
}

impl NewCommand {
	pub fn app() -> App<'static> {
		let app = App::new("new")
			.about("take a note")
			.visible_alias("n")
			.aliases(&["create", "add"]);

		let title = Arg::new("title")
			.about("note title")
			.takes_value(false)
			.required(true);

		let body = Arg::new("body")
			.about("note body")
			.takes_value(false)
			.required(true);

		let tag = Arg::new("tag")
			.about("comma separated list of tags")
			.short('t')
			.long("tag")
			.takes_value(true)
			.long_about("comma separated list of tags. can be used to filter notes");

		let lvl = Arg::new("lvl")
			.short('l')
			.long("lvl")
			.about("importance level of the note")
			.long_about("importance level of the note (0..255). can be used to filter notes")
			.takes_value(true)
			.validator(|s: &str| -> Result<(), String> {
				s.parse::<u8>()
					.map_err(|_| {
						format!(
							"invalid importance level '{}': must be a number between 0 and 255",
							s
						)
					})
					.map(|_| ())
			});

		app.arg(title).arg(body).arg(tag).arg(lvl)
	}

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
		let p = config::todo_path_checked()?;
		let mut notes = note::get_notes(&p)?;
		let title = &self.title;
		let n = Note {
			title: self.title,
			body: self.body,
			lvl: self.lvl,
			tags: self.tags,
		};
		notes.insert(0, n);
		note::save_notes(&p, &notes)?;
		println!("saved {}", title);
		Ok(())
	}
}
