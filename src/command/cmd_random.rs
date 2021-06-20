use crate::{config::Config, note};
use clap::ArgMatches;
use rand::seq::SliceRandom;
use std::error::Error;

#[derive(Debug)]
pub struct RandomCommand {
    n: usize,
    tags: Option<Vec<String>>,
}

impl RandomCommand {
    pub fn from_matches(m: &ArgMatches) -> Self {
        let n = m.value_of("n").unwrap().parse::<usize>().unwrap();

        let tags = m
            .values_of("tag")
            .map(|i| i.map(String::from).collect::<Vec<_>>());

        Self { n, tags }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let c = Config::get()?;

        if let Some(tags) = self.tags.as_mut() {
            for t in tags.iter_mut() {
                *t = t.to_lowercase();
            }
        }

        let notes: Vec<_> = note::get_notes(&c.todos_file)?
            .into_iter()
            .filter(|n| match self.tags.as_ref() {
                None => true,
                Some(tags) => match n.tags.as_ref() {
                    None => false,
                    Some(t) => is_match(tags, t),
                },
            })
            .enumerate()
            .collect();

        if notes.is_empty() {
            if self.tags.is_none() {
                println!("you have no notes");
            } else {
                println!("given tags did not match any note");
            }
            return Ok(());
        }

        let mut rng = &mut rand::thread_rng();
        let notes: Vec<_> = notes.choose_multiple(&mut rng, self.n).collect();

        note::print_notes_enumerated(&notes);
        Ok(())
    }
}

fn is_match(lower: &[String], tags: &[String]) -> bool {
    tags.iter()
        .any(|s| lower.iter().any(|x| s.to_lowercase() == *x))
}
