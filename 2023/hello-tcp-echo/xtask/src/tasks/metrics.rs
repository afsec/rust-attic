use clap::Parser;

use crate::helpers::Runner;

#[derive(Parser, Debug)]
pub(crate) struct Metrics;

impl Runner for Metrics {
    fn run(&self, sh: &xshell::Shell) -> crate::helpers::XtaskResult<()> {
        Ok(())
    }
}
