use serde_derive::{Deserialize, Serialize};

use std::{
    env,
    error::Error,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

pub mod hook;
use hook::Hooks;

fn todo_path_default() -> Option<PathBuf> {
    match dirs::home_dir() {
        Some(mut p) => {
            p.push(".todos.toml");
            Some(p)
        }
        None => None,
    }
}

fn todo_path_env() -> Option<PathBuf> {
    env::var("TODOS_FILE_PATH").map(PathBuf::from).ok()
}

fn config_dir() -> Result<PathBuf, &'static str> {
    if let Ok(s) = env::var("TODO_CONFIG_DIR") {
        Ok(s.into())
    } else if let Some(mut p) = dirs::config_dir() {
        p.push("todo.toml");
        Ok(p)
    } else {
        Err("could not determine the todo config dir, consider setting the $TODO_CONFIG_DIR env variable")
    }
}

pub fn config_path_checked() -> Result<PathBuf, Box<dyn Error>> {
    let mut p = config_dir()?;
    if !p.is_dir() {
        // first make sure the parent dirs exist
        fs::create_dir_all(&p)?;
    }
    p.push("todo.toml");
    if !p.is_file() {
        let def = Config::default();
        let data = toml::to_string_pretty(&def)?;
        let mut file = File::create(&p)?;
        file.write_all(data.as_bytes())?;
        file.sync_all()?;
    }
    Ok(p)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub todos_file: PathBuf,
    pub abort_on_hook_error: Option<bool>,
    pub hooks: Hooks,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            todos_file: todo_path_env().unwrap_or_else(PathBuf::new),
            abort_on_hook_error: Some(true),
            hooks: Hooks::default(),
        }
    }
}

impl Config {
    pub fn get() -> Result<Self, Box<dyn Error>> {
        let p = config_path_checked()?;
        let data = fs::read_to_string(&p)?;
        let mut conf: Self = toml::from_str(&data)?;
        if let Some(tp) = todo_path_env() {
            conf.todos_file = tp;
        } else if conf.todos_file.as_os_str().is_empty() {
            conf.todos_file= todo_path_default().ok_or("could not determine todos_file path; consider setting the TODOS_FILE_PATH env variable or editing the config file")?;
        }

        // create todos file if it doesn't exist
        if !conf.todos_file.is_file() {
            File::create(&conf.todos_file)?;
        }
        Ok(conf)
    }
}
