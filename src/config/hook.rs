use serde_derive::{Deserialize, Serialize};
use std::{io, path::PathBuf, process::Command};

use HookType::*;

enum HookType {
    PreNew,
    PostNew,
    PreRemove,
    PostRemove,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hooks {
    pub pre_new: Option<Vec<Hook>>,
    pub post_new: Option<Vec<Hook>>,
    pub pre_remove: Option<Vec<Hook>>,
    pub post_remove: Option<Vec<Hook>>,
}

impl Default for Hooks {
    fn default() -> Self {
        Self {
            pre_new: None,
            post_new: None,
            pre_remove: None,
            post_remove: None,
        }
    }
}

impl Hooks {
    fn run(&self, hook: HookType) -> io::Result<()> {
        let hooks = match hook {
            PreNew => self.pre_new.as_ref(),
            PostNew => self.post_new.as_ref(),
            PreRemove => self.pre_remove.as_ref(),
            PostRemove => self.post_remove.as_ref(),
        };
        if let Some(hks) = hooks {
            let mut last_err: io::Result<()> = Ok(());
            for h in hks {
                if let Err(e) = h.run() {
                    last_err = match h.abort_on_error {
                        Some(true) | None => return Err(e),
                        Some(false) => Err(e),
                    };
                }
            }
            last_err
        } else {
            Ok(())
        }
    }

    pub fn run_pre_new(&self) -> io::Result<()> {
        self.run(PreNew)
    }

    pub fn run_post_new(&self) -> io::Result<()> {
        self.run(PostNew)
    }
    pub fn run_pre_remove(&self) -> io::Result<()> {
        self.run(PreRemove)
    }

    pub fn run_post_remove(&self) -> io::Result<()> {
        self.run(PostRemove)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hook {
    cmd: Vec<String>,
    silent: Option<bool>,
    working_dir: Option<PathBuf>,
    abort_on_error: Option<bool>,
}

impl Hook {
    fn run(&self) -> io::Result<()> {
        if self.cmd.is_empty() {
            return Ok(());
        }
        let mut cmd = Command::new(&self.cmd[0]);
        for arg in &self.cmd[1..] {
            cmd.arg(arg);
        }
        if let Some(wd) = &self.working_dir {
            cmd.current_dir(wd);
        }
        if let Some(true) = self.silent {
            cmd.output().map(|_| ())
        } else {
            cmd.spawn().map(|_| ())
        }
    }
}
