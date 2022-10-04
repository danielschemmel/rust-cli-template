// By default, forbid unsafe code - your project may need to change this setting
#![forbid(unsafe_code)]

mod cli;

use anyhow::{Context, Result};
use cli::{Args, ReturnCode};

async fn parse_arguments() -> Result<ReturnCode> {
	use clap::error::ErrorKind;
	use clap::Parser;

	match Args::try_parse() {
		Ok(args) => cli::main(args).await,
		Err(e) => match e.kind() {
			ErrorKind::DisplayVersion => {
				e.print().expect("Could not print version");
				Ok(ReturnCode::Success)
			}
			ErrorKind::DisplayHelp => {
				e.print().expect("Could not print help");
				Ok(ReturnCode::Success)
			}
			_ => {
				e.print().expect("Could not print error");
				Ok(ReturnCode::ArgumentParsing)
			}
		},
	}
}

#[tokio::main]
async fn main() -> Result<()> {
	parse_arguments()
		.await
		.map(|return_code| std::process::exit(return_code as i32))
		.context("Uncaught error in main")
}
