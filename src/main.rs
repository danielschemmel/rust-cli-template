// By default, forbid unsafe code - your project may need to change this setting
#![forbid(unsafe_code)]

mod cli;

use anyhow::{Context, Result};
use cli::{Args, ReturnCode};

impl std::process::Termination for ReturnCode {
	fn report(self) -> std::process::ExitCode {
		std::process::ExitCode::from(self as u8)
	}
}

#[tokio::main]
async fn main() -> Result<ReturnCode> {
	use clap::Parser;
	use clap::error::ErrorKind;

	match Args::try_parse() {
		Ok(args) => cli::main(args).await.context("Uncaught error in main"),
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
