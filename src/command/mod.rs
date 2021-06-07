mod app;
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

use clap::App;

pub struct Command;

impl Command {
    pub fn app() -> App<'static> {
        app::app()
    }

    pub fn run() -> Result<(), Box<dyn Error>> {
        let matches = Self::app().get_matches();
        if let Some(cmd) = matches.subcommand_name() {
            let m = matches
                .subcommand_matches(cmd)
                .expect("internal error: arg matches is empty");
            match cmd {
                "list" => ListCommand::from_matches(m).run(),
                "remove" => RemoveCommand::from_matches(m).run(),
                "where" => show_todo_path(),
                "new" => NewCommand::from_matches(m).run(),
                _ => panic!("internal error: unknown command {}", cmd),
            }
        } else {
            ListCommand::default().run()
        }
    }
}

fn show_todo_path() -> Result<(), Box<dyn Error>> {
    config::todo_path_checked().map(|p| {
        println!("{}", p.display());
    })
}
