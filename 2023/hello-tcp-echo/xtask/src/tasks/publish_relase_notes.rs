use clap::Parser;

use crate::helpers::Runner;

#[derive(Parser, Debug)]
pub(crate) struct PublishReleaseNotes;

impl Runner for PublishReleaseNotes {
    fn run(&self, sh: &xshell::Shell) -> crate::helpers::XtaskResult<()> {
        Ok(())
    }
}
