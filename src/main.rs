// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate failure;

#[allow(unused_imports)] // the macro is used - but clippy fails to notice
#[macro_use]
extern crate log;

mod cli;
use cli::{Args, ReturnCode};

mod logging_error;
pub use logging_error::LoggingError;

fn parse_arguments() -> Result<ReturnCode, failure::Error> {
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

fn backtraces_enabled() -> bool {
	std::env::var("RUST_BACKTRACE")
		.ok()
		.and_then(|value| if value == "1" { Some(()) } else { None })
		.is_some()
}

fn report_error(e: &failure::Fail) {
	eprintln!();
	eprintln!("Oops!");
	eprintln!("An unexpected error occurred. Please provide the error message below and any way to cause this error to the maintainers of this program.");

	eprintln!();
	eprintln!("error: {}", e);
	for e in e.iter_causes() {
		eprintln!("caused by: {}", e);
	}

	if backtraces_enabled() {
		if let Some(backtrace) = e.backtrace() {
			eprintln!();
			eprintln!("{}", backtrace);
		}
	} else {
		eprintln!();
		eprintln!("Please set the environment variable RUST_BACKTRACE to 1 to enable backtraces.");
	}

	std::process::exit(ReturnCode::UnhandledFailure as i32);
}

use failure::ResultExt;

fn main() {
	parse_arguments()
		.map(|return_code| std::process::exit(return_code as i32))
		.context("Uncaught error in main")
		.map_err(|error| report_error(&error))
		.ok();
}
