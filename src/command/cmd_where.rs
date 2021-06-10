use crate::config::{self, Config};
use clap::ArgMatches;
use std::error::Error;

#[derive(Debug)]
pub enum WhereCommand {
    All,
    Config,
    Todos,
}

impl WhereCommand {
    pub fn from_matches(m: &ArgMatches) -> Self {
        if m.is_present("config") {
            Self::Config
        } else if m.is_present("todos") {
            Self::Todos
        } else {
            Self::All
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        match self {
            Self::Config => config::config_path_checked().map(|p| {
                println!("{}", p.display());
            }),
            Self::Todos => Config::get().map(|c| {
                println!("{}", c.todos_file.display());
            }),
            Self::All => {
                let p = config::config_path_checked()?;
                let c = Config::get()?;
                println!("todos: {}\nconfig: {}", c.todos_file.display(), p.display());
                Ok(())
            }
        }
    }
}
