use serde_derive::{Deserialize, Serialize};

use std::error::Error;
use std::fs::{self, File};
use std::path::Path;
use std::io::Write;

#[derive(Debug, Deserialize, Serialize)]
pub struct Note {
	pub title: String,
	pub body: String,
	pub tags: Option<Vec<String>>,
	pub lvl: Option<u8>,
}

impl Note {
	pub fn new(title: impl AsRef<str>, body: impl AsRef<str>) -> Self {
		Self {
			title: title.as_ref().to_owned(),
			body: body.as_ref().to_owned(),
			tags: None,
			lvl: None,
		}
	}
}

pub fn save_notes(p: impl AsRef<Path>, notes: &[Note]) -> Result<(), Box<dyn Error>> {
	let data = toml::to_string(notes)?;
	let mut f = File::create(p.as_ref())?;
	f.write_all(data.as_bytes())?;
	f.sync_all()?;
	Ok(())
}

pub fn get_notes(p: impl AsRef<Path>) -> Result<Vec<Note>, Box<dyn Error>> {
	let data = fs::read_to_string(p.as_ref())?;
	let notes: Vec<Note> = toml::from_str(&data)?;
	Ok(notes)
}

pub fn print_notes(notes: &[Note]) {
	let max_title = notes.iter().map(|n| n.title.len()).max().unwrap_or(4);
	for (i, n) in notes.iter().enumerate() {
		println!(
			"#{index:2}  {title:width$}  |  {body}",
			index = i,
			width = max_title,
			title = n.title,
			body = n.body
		);
	}
}

pub fn print_notes_enumerated(notes: &[&(usize, Note)]) {
	let max_title = notes.iter().map(|(_, n)| n.title.len()).max().unwrap_or(4);
	for (i, n) in notes {
		println!(
			"#{index:2}  {title:width$}  |  {body}",
			index = i,
			width = max_title,
			title = n.title,
			body = n.body
		);
	}
}
