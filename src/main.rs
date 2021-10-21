// By default, forbid unsafe code - your project may need to change this setting
#![forbid(unsafe_code)]

mod cli;

use anyhow::{Context, Result};
use cli::{Args, ReturnCode};

async fn parse_arguments() -> Result<ReturnCode> {
	use structopt::clap::ErrorKind;
	use structopt::StructOpt;

	match Args::from_iter_safe(std::env::args_os()) {
		Ok(args) => cli::main(args).await,
		Err(e) => match e.kind {
			ErrorKind::VersionDisplayed => {
				println!("{}", e.message);
				Ok(ReturnCode::Success)
			}
			ErrorKind::HelpDisplayed => {
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

#[tokio::main]
async fn main() -> Result<()> {
	parse_arguments()
		.await
		.map(|return_code| std::process::exit(return_code as i32))
		.context("Uncaught error in main")
}
