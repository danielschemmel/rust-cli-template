use crate::errors::*;
use anyhow::Result;

#[cfg(not(feature = "subcommands"))]
#[derive(structopt::StructOpt, Debug)]
#[structopt(
	setting = structopt::clap::AppSettings::ColoredHelp,
	version = concat!(env!("VERGEN_SEMVER"), "\n\nBuilt from commit ", env!("VERGEN_SHA"), " at ", env!("VERGEN_BUILD_TIMESTAMP"), " for ", env!("VERGEN_TARGET_TRIPLE"), "."),
)]
pub struct Args {
	/// Something, something, path
	#[structopt(name = "FILE")]
	file: std::path::PathBuf,
	/// Optional blubber command
	#[structopt(name = "BLUBBER")]
	blubber: Option<String>,
	/// Port to listen on.
	#[structopt(short = "p", long = "port", env = "PORT", default_value = "8080")]
	port: u16,
	/// Address to listen on.
	#[structopt(short = "a", long = "address", default_value = "127.0.0.1")]
	address: String,
	/// Perform "work" by sleeping the specified amount of milliseconds
	#[structopt(short = "s", long = "sleep", default_value = "0")]
	sleep: u64,
}

#[cfg(feature = "subcommands")]
#[derive(structopt::StructOpt, Debug)]
#[structopt(
	setting = structopt::clap::AppSettings::ColoredHelp,
	version = concat!(env!("VERGEN_SEMVER"), "\n\nBuilt from commit ", env!("VERGEN_SHA"), " at ", env!("VERGEN_BUILD_TIMESTAMP"), " for ", env!("VERGEN_TARGET_TRIPLE"), "."),
)]
pub enum Args {
	#[structopt(name = "file-based")]
	FileBased {
		/// Something, something, path
		#[structopt(name = "FILE")]
		file: std::path::PathBuf,
		/// Optional blubber command
		#[structopt(name = "BLUBBER")]
		blubber: Option<String>,
	},
	#[structopt(name = "network-based")]
	NetworkBased {
		/// Port to listen on.
		#[structopt(short = "p", long = "port", env = "PORT", default_value = "8080")]
		port: u16,
		/// Address to listen on.
		#[structopt(short = "a", long = "address", default_value = "127.0.0.1")]
		address: String,
	},
}

#[derive(Copy, Clone)]
pub enum ReturnCode {
	Success = 0,
	CtrlC = -1,
	ArgumentParsing = 1,
}

fn create_logger() -> Result<flexi_logger::ReconfigurationHandle, LoggingError> {
	flexi_logger::Logger::with_env_or_str(concat!("warn, ", env!("CARGO_PKG_NAME"), "=debug"))
		.format(flexi_logger::colored_with_thread)
		.start()
		.map_err(LoggingError::CreationFailure)
}

fn set_ctrlc_handler() -> Result<()> {
	use std::sync::atomic::{AtomicBool, Ordering};
	use std::sync::Arc;

	let previous_ctrlc = Arc::new(AtomicBool::new(false));
	ctrlc::set_handler(move || {
		if (*previous_ctrlc).swap(true, Ordering::Relaxed) {
			error!("Received Ctrl+C again: Terminating forcefully!");
			println!("\nReceived Ctrl+C again: Terminating forcefully!");
			std::process::exit(ReturnCode::CtrlC as i32);
		} else {
			warn!("Received Ctrl+C...");
			println!("\nReceived Ctrl+C...");
		}
	})?;

	Ok(())
}

#[cfg(not(feature = "bug"))]
pub fn main(args: Args) -> Result<ReturnCode> {
	let _log_handle = create_logger()?;
	set_ctrlc_handler()?;

	println!("{:?}", args);

	std::thread::sleep(std::time::Duration::from_millis(args.sleep));

	Ok(ReturnCode::Success)
}

#[cfg(feature = "bug")]
pub fn main(args: Args) -> Result<ReturnCode> {
	let _log_handle = create_logger()?;
	set_ctrlc_handler()?;

	println!("{:?}", args);

	use anyhow::{anyhow, Context};

	error!("A bug is about to occur!");
	let error = anyhow!("The bug feature is enabled");
	Err(error).context("Some context for where the error caused problems")?;

	Ok(ReturnCode::Success)
}

#[cfg(test)]
mod test {
	use super::*;
	use pretty_assertions::{assert_eq, assert_ne};

	#[test]
	pub fn test_return_code_success() {
		assert_eq!(ReturnCode::Success as u8, 0);
	}

	#[test]
	pub fn test_pretty_assertions_dummy() {
		assert_eq!(Some(1), Some(1));
		assert_ne!(Some(1), Some(2));
	}

	#[test]
	pub fn test_create_logger() {
		let log_handle = create_logger();
		assert!(log_handle.is_ok());
	}
}
