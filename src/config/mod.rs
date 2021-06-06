use std::env;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

pub fn todo_path() -> Result<PathBuf, &'static str> {
	if let Ok(s) = env::var("TODO_PATH") {
		let mut p: PathBuf= s.into();
		p.push(".todo.toml");
		Ok(p)
	} else if let Some(mut p) = dirs::home_dir() {
		p.push(".todo.toml");
		Ok(p)
	} else {
		Err("could not locate todo.toml")
	}
}

pub fn todo_path_checked() -> Result<PathBuf, Box<dyn Error>> {
	let p = todo_path()?;
	if !p.is_file() {
		File::create(&p)?;
		Ok(p)
	} else {
		Ok(p)
	}
}
