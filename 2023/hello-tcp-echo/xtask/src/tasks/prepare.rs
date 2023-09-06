use std::{cell::RefCell, fmt::Display};

use clap::Parser;
use xshell::{cmd, Shell};

use crate::helpers::{Runner, XtaskResult};

#[derive(Parser, Debug)]
pub(crate) struct Prepare {
    #[arg(long, short)]
    /// Show cargo ouput
    verbose: bool,
}

impl Prepare {
    pub(crate) fn verbose(&self) -> bool {
        self.verbose
    }
}

impl Runner for Prepare {
    fn run(&self, sh: &Shell) -> XtaskResult<()> {
        println!("{self}:\n");

        {
            let cmd = cmd!(sh, "cargo check");
            if self.verbose() {
                cmd.run()?;
            } else {
                cmd.ignore_stdout().ignore_stderr().run()?;
            }
        }
        {
            let cmd = cmd!(sh, "cargo build");
            if self.verbose() {
                cmd.run()?;
            } else {
                cmd.ignore_stdout().ignore_stderr().run()?;
            }
        }
        {
            let cmd = cmd!(sh, "cargo build --release");
            if self.verbose() {
                cmd.run()?;
            } else {
                cmd.ignore_stdout().ignore_stderr().run()?;
            }
        }
        Ok(())
    }
}

impl Display for Prepare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "prepare")
    }
}
