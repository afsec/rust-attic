use clap::Parser;
use xshell::{cmd, Shell};

use crate::helpers::{Runner, XtaskResult};

#[derive(Parser, Debug)]
pub(crate) struct FuzzTests;

impl Runner for FuzzTests {
    fn run(&self, sh: &Shell) -> XtaskResult<()> {
        run_fuzzer(sh)?;
        Ok(())
    }
}

fn run_fuzzer(sh: &Shell) -> XtaskResult<()> {
    use anyhow::bail;
    let _d = sh.push_dir("./fuzzer");
    let _e = sh.push_env("RUSTUP_TOOLCHAIN", "nightly");
    if cmd!(sh, "cargo fuzz --help").read().is_err() {
        cmd!(sh, "cargo install cargo-fuzz").run()?;
    };

    // Expecting nightly rustc
    let out = cmd!(sh, "rustc --version").read()?;
    if !out.contains("nightly") {
        bail!("fuzz tests require nightly rustc")
    }

    cmd!(sh, "cargo fuzz run parser").run()?;
    Ok(())
}
