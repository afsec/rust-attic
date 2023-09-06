use std::path::PathBuf;

use clap::Parser;

use crate::helpers::Runner;

#[derive(Parser, Debug)]
pub(crate) struct Dist {
    /// Path to the dist folder
    pub(crate) path: PathBuf,
}

impl Runner for Dist {
    fn run(&self, sh: &xshell::Shell) -> crate::helpers::XtaskResult<()> {
        Ok(())
    }
}
