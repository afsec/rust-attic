use clap::Parser;

use crate::helpers::Runner;

#[derive(Parser, Debug)]
pub(crate) struct Build {
    #[arg(long, short)]
    /// Enable release
    release: bool,
}

impl Runner for Build {
    fn run(&self, sh: &xshell::Shell) -> crate::helpers::XtaskResult<()> {
        Ok(())
    }
}
