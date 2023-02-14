use std::process::ExitCode;

use clap::Parser;
use cli::Clargs;
use fs::erdtree::{self, tree::Tree};
use ignore::WalkParallel;

/// CLI rules and definitions.
mod cli;

/// Filesystem operations.
mod fs;

/// Common utilities.
mod utils;

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    erdtree::tree::ui::init();
    let clargs = Clargs::parse();
    let walker = WalkParallel::try_from(&clargs)?;
    let tree = Tree::new(walker, clargs.sort(), clargs.level())?;

    println!("{tree}");

    Ok(())
}
