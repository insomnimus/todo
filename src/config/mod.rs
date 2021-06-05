use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::error::Error;

pub fn todo_path() -> Result<PathBuf, &'static str> {
	if let Ok(s) = env::var("TODO_PATH") {
		Ok(s.into())
	} else if let Some(mut p) = dirs::home_dir() {
		p.push(".todo.toml");
		Ok(p)
	} else {
		Err("could not locate todo.toml")
	}
}

pub fn todo_path_checked() -> Result<PathBuf, Box<dyn Error>> {
	let p= todo_path()?;
	if !p.is_file() {
		File::create(&p)?;
		Ok(p)
	}else{
		Ok(p)
	}
}
