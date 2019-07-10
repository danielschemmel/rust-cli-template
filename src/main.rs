// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

mod cli;
use cli::{Args, ReturnCode};

mod errors;
use errors::*;

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

fn main() {
	match parse_arguments() {
		Ok(return_code) => {
			std::process::exit(return_code as i32);
		}
		Err(e) => {
			use std::io::Write;
			let stderr = &mut ::std::io::stderr();
			let stderr_error_message = "Error writing to stderr";

			writeln!(stderr, "Oops!").expect(stderr_error_message);
			writeln!(stderr, "An unexpected error occurred. Please provide the error message below and any way to cause this error to the maintainers of this program.").expect(stderr_error_message);
			writeln!(stderr).expect(stderr_error_message);

			writeln!(stderr, "error: {}", e).expect(stderr_error_message);

			for e in e.iter().skip(1) {
				writeln!(stderr, "caused by: {}", e).expect(stderr_error_message);
			}

			if let Some(backtrace) = e.backtrace() {
				writeln!(stderr, "backtrace: {:?}", backtrace).expect(stderr_error_message);
			} else {
				writeln!(
					stderr,
					"run with the environment variable RUST_BACKTRACE=1 to get a backtrace..."
				)
				.expect(stderr_error_message);
			}

			std::process::exit(ReturnCode::UnhandledFailure as i32);
		}
	}
}
