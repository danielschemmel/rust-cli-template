// By default, forbid unsafe code - your project may need to change this setting
#![forbid(unsafe_code)]

#[allow(unused_imports)] // the macros are used by the example only when the "bug" feature is on
#[macro_use]
extern crate anyhow;

#[allow(unused_imports)] // the macros are used by the example only when the "bug" feature is on
#[macro_use]
extern crate log;

mod cli;
mod errors;

use anyhow::{Context, Result};
use cli::{Args, ReturnCode};

fn parse_arguments() -> Result<ReturnCode> {
	use structopt::StructOpt;
	match Args::from_iter_safe(std::env::args_os()) {
		Ok(args) => cli::main(args),
		Err(e) => match e.kind {
			structopt::clap::ErrorKind::VersionDisplayed => {
				println!("{}", e.message);
				Ok(ReturnCode::Success)
			}
			structopt::clap::ErrorKind::HelpDisplayed => {
				println!("{}", e.message);
				Ok(ReturnCode::Success)
			}
			_ => {
				println!("{}", e.message);
				Ok(ReturnCode::ArgumentParsing)
			}
		},
	}
}

fn main() -> Result<()> {
	parse_arguments()
		.map(|return_code| std::process::exit(return_code as i32))
		.context("Uncaught error in main")
}
