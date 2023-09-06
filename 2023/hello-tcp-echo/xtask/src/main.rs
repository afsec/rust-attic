#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::dbg_macro,
    clippy::todo,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::mem_forget,
    clippy::unused_self,
    clippy::filter_map_next,
    clippy::needless_continue,
    clippy::needless_borrow,
    clippy::match_wildcard_for_single_variants,
    clippy::if_let_mutex,
    clippy::mismatched_target_os,
    clippy::await_holding_lock,
    clippy::match_on_vec_items,
    clippy::imprecise_flops,
    clippy::suboptimal_flops,
    clippy::lossy_float_literal,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::fn_params_excessive_bools,
    clippy::exit,
    clippy::inefficient_to_string,
    clippy::linkedlist,
    clippy::macro_use_imports,
    clippy::option_option,
    clippy::verbose_file_reads,
    clippy::unnested_or_patterns,
    clippy::str_to_string,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
    missing_debug_implementations,
    // missing_docs
)]
#![deny(unreachable_pub, private_in_public)]
#![allow(elided_lifetimes_in_paths, clippy::type_complexity)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(test, allow(clippy::float_cmp))]

mod cli;
mod helpers;
mod tasks;

use cli::Command as XtaskCommand;
use xshell::Shell;

use crate::helpers::{project_root, Runner, XtaskResult};

fn main() -> XtaskResult<()> {
    use clap::Parser;

    let cli = cli::Args::parse();

    let sh = &Shell::new()?;

    sh.change_dir(project_root()?);
    use humantime::format_duration;
    use std::time::{Duration, Instant};

    print!("\n{{ cargo xtask ");
    let now = Instant::now();

    cli.command.run(sh)?;

    println!(
        "\n}}: Finished in {}",
        format_duration(Duration::from_secs(now.elapsed().as_secs()))
    );

    Ok(())
}
