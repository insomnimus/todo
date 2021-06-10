mod app;
mod cmd_list;
mod cmd_new;
mod cmd_remove;
mod cmd_where;
mod index;
mod query;

use std::error::Error;

use cmd_list::ListCommand;
use cmd_new::NewCommand;
use cmd_remove::RemoveCommand;
use cmd_where::WhereCommand;

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
                "where" => WhereCommand::from_matches(m).run(),
                "new" => NewCommand::from_matches(m).run(),
                _ => panic!("internal error: unknown command {}", cmd),
            }
        } else {
            ListCommand::default().run()
        }
    }
}
