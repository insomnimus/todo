use super::index::MinMax;
use crate::note::Note;

use glob::{MatchOptions, Pattern};

#[derive(Debug)]
pub struct Filter {
	pub title: Option<Pattern>,
	pub lvl: Option<MinMax>,
	pub tags: Option<Vec<String>>,
}

impl Filter {
	pub fn is_match(&self, n: &Note) -> bool {
		const OPT: MatchOptions = MatchOptions {
			case_sensitive: false,
			require_literal_separator: false,
			require_literal_leading_dot: false,
		};

		if let Some(title) = &self.title {
			if !title.matches_with(&n.title[..], OPT) {
				return false;
			}
		}
		if let Some(lvl) = &self.lvl {
			if !lvl.in_range(n.lvl.unwrap_or_default()) {
				return false;
			}
		}

		if let Some(tags) = &self.tags {
			if let Some(note_tags) = &n.tags {
				if !tags.iter().any(|s| note_tags.iter().any(|t| s == t)) {
					return false;
				}
			} else {
				return false;
			}
		}

		true
	}
}
