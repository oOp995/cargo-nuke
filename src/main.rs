//! `cargo-nuke` **is not** a library.
//! > `cargo-nuke` is CLI tool helps you search and clean old cargo artifacts that has
//! been built and forgotten occupying space.
//! operation done in path finds all crates that matches search criteria
//! and gives you option to clean or not .

use crate::result::Conflicts;
use std::error::Error;

//modules

///`cargos` module contains the cargo search and classify algorithms
/// for more details refer to github repo 
pub mod cargos;

///`cli` module is the part of tool which takes CLI arguments and determine the
/// flow of control that `main` and `exec` module will follow.
pub mod cli;

///`exec` module contains the possible execution path's to follow. 
pub mod exec;

///`result` module contains the errors that is possible to occur.

pub mod result;

///`utils` module contains the common utils used in tool .
pub mod utils;

use colored::*;

///starting point to take CLI arguments
/// evaluate and execute
pub fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::NukeCli::parse();

    exec::is_conflict(&args)?;
    exec::execute(&args)?;

    eprintln!("{}","cargo-nuke terminated succesfully".green());
    Ok(())
}
