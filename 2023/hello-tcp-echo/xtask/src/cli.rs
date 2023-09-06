use clap::Parser;

use crate::{
    helpers::Runner,
    tasks::{Build, Dist, FuzzTests, Metrics, Prepare, PublishReleaseNotes},
};

/// cargo xtask
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Parser, Debug)]
pub(crate) enum Command {
    /// Perform cargo check, build , build --release
    Prep(Prepare),
    /// Compiles release version
    Build(Build),
    /// Run fuzzing tests
    FuzzTests(FuzzTests),
    /// Compiles distribute version
    Dist(Dist),
    /// Prepare release notes to publish
    PublishReleaseNotes(PublishReleaseNotes),
    /// Generate metrics
    Metrics(Metrics),
}
impl Runner for Command {
    fn run(&self, sh: &xshell::Shell) -> crate::helpers::XtaskResult<()> {
        match &self {
            Self::Prep(cmd) => cmd.run(sh)?,
            Self::Build(cmd) => cmd.run(sh)?,
            Self::FuzzTests(cmd) => cmd.run(sh)?,
            Self::Dist(cmd) => cmd.run(sh)?,
            Self::PublishReleaseNotes(cmd) => cmd.run(sh)?,
            Self::Metrics(cmd) => cmd.run(sh)?,
        };
        Ok(())
    }
}
