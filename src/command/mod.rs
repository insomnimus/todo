mod cmd_list;
mod cmd_new;
mod cmd_remove;
mod index;
mod query;

use std::error::Error;

use crate::config;

use cmd_list::ListCommand;
use cmd_new::NewCommand;
use cmd_remove::RemoveCommand;

use clap::{crate_version, App, AppSettings};

fn validate_index(s: &str) -> Result<(), String> {
	let vals: Vec<_> = s.split(':').collect();
	match vals.len() {
		0 => {
			Err("the syntax is START:END or just N, where START, END and N are integers".to_owned())
		}
		1 => s
			.parse::<isize>()
			.map(|_| ())
			.map_err(|_| format!("{} is not a valid number/ range pattern", s)),
		2 => {
			let (x, y) = (vals[0], vals[1]);
			if x.is_empty() && y.is_empty() {
				Ok(())
			} else if x.is_empty() {
				y.parse::<isize>()
					.map_err(|_| format!("{}: not a number", y))
					.map(|_| ())
			} else if y.is_empty() {
				x.parse::<isize>()
					.map_err(|_| format!("{}: not a valid number", x))
					.map(|_| ())
			} else if let (Ok(_), Ok(_)) = (x.parse::<isize>(), y.parse::<isize>()) {
				Ok(())
			} else {
				Err(format!("{}: invalid range expression", s))
			}
		}
		_ => Err("only 1 ':' is allowed in a range expression".to_owned()),
	}
}

fn validate_minmax(s: &str) -> Result<(), String> {
	let vals: Vec<_> = s.split(':').collect();
	match vals.len() {
		0=> Err("the syntax is MIN:MAX or just LVL, where MIN, MAX and LVL are integers between 0 and 255".to_owned()),
		1 => s.parse::<u8>().map(|_| ()).map_err(|_| format!("{} is not a valid number/ range pattern", s)),
		2=> {
			let (x, y)= (vals[0], vals[1]);
			if x.is_empty() && y.is_empty() {
				Ok(())
			}else if x.is_empty() {
				y.parse::<u8>().map(|_| ()).map_err(|_| format!("{}: not a valid number", y))
			}else if y.is_empty() {
				x.parse::<u8>().map(|_| ()).map_err(|_| format!("{}: not a valid number", x))
			}else if let (Ok(_), Ok(_)) = (x.parse::<u8>(), y.parse::<u8>()) {
				Ok(())
			}else{
				Err(format!("{}: invalid range expression", s))
			}
		}
		_=> Err("only 1 ':' is allowed in a range expression".to_owned())
	}
}

#[derive(Debug)]
pub enum Command {
	Bare,
	Where,
	List(ListCommand),
	New(NewCommand),
	Remove(RemoveCommand),
}

impl Command {
	pub fn app() -> App<'static> {
		let where_command = App::new("where")
			.short_flag('w')
			.long_flag("wh")
			.about("show the path of the todoes file");

		App::new("todo")
			.about("simple note tracker")
			.global_setting(AppSettings::UnifiedHelpMessage)
			.global_setting(AppSettings::VersionlessSubcommands)
			.global_setting(AppSettings::AllowNegativeNumbers)
			.global_setting(AppSettings::DeriveDisplayOrder)
			.version(crate_version!())
			.subcommand(ListCommand::app())
			.subcommand(NewCommand::app())
			.subcommand(RemoveCommand::app())
			.subcommand(where_command)
	}

	pub fn run() {
		let matches = Self::app().get_matches();
		if let Some(cmd) = matches.subcommand_name() {
			let m = matches.subcommand_matches(cmd).expect("internal error: arg matches is empty");
			match cmd {
				"list" => ListCommand::from_matches(m).run(),
				"remove" => RemoveCommand::from_matches(m).run(),
				"where" => show_todo_path(),
				"new" => NewCommand::from_matches(m).run(),
				_ => panic!("internal error: unknown command {}", cmd),
			};
		} else {
			ListCommand::default().run();
		}
	}
}

fn show_todo_path() -> Result<(), Box<dyn Error>> {
	config::todo_path_checked().map(|p| {
		println!("{}", p.display());
	})
}
