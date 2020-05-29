use crate::errors::*;
use anyhow::Result;

#[cfg(not(feature = "subcommands"))]
#[derive(structopt::StructOpt, Debug)]
#[structopt(
	setting = structopt::clap::AppSettings::ColoredHelp,
	version = build_info::format!("{.crate_info.version}\n\nBuilt from {.version_control} at {.timestamp} with rustc {.compiler.version} {.compiler.channel} ({.compiler.commit_id}) for {.compiler.target_triple} on {.compiler.host_triple}."),
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
}

#[cfg(feature = "subcommands")]
#[derive(structopt::StructOpt, Debug)]
#[structopt(
	setting = structopt::clap::AppSettings::ColoredHelp,
	version = build_info::format!("{.crate_info.version}\n\nBuilt from {.version_control} at {.timestamp} with rustc {.compiler.version} {.compiler.channel} ({.compiler.commit_id}) for {.compiler.target_triple} on {.compiler.host_triple}."),
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
		.format(if atty::is(atty::Stream::Stderr) {
			flexi_logger::colored_with_thread
		} else {
			flexi_logger::with_thread
		})
		//.log_target(flexi_logger::LogTarget::File)
		//.format_for_files(flexi_logger::with_thread)
		//.duplicate_to_stderr(flexi_logger::Duplicate::Warn)
		.start()
		.map_err(LoggingError::CreationFailure)
}

fn set_ctrlc_handler() -> Result<std::sync::mpsc::Receiver<()>> {
	use std::sync::atomic::{AtomicBool, Ordering};
	use std::sync::Arc;

	let (sender, receiver) = std::sync::mpsc::sync_channel(1);
	let previous_ctrlc = Arc::new(AtomicBool::new(false));

	ctrlc::set_handler(move || {
		if (*previous_ctrlc).swap(true, Ordering::Relaxed) {
			warn!("Received Ctrl+C again: Terminating forcefully!");
			println!("\nReceived Ctrl+C again: Terminating forcefully!");
			std::process::exit(ReturnCode::CtrlC as i32);
		} else {
			warn!("Received Ctrl+C...");
			println!("\nReceived Ctrl+C...");
			sender
				.send(())
				.map_err(|e| error!("Could not notify main program: {}", e))
				.ok();
		}
	})?;

	Ok(receiver)
}

#[cfg(not(feature = "bug"))]
pub fn main(args: Args) -> Result<ReturnCode> {
	let log_handle = create_logger()?;
	let ctrlc = set_ctrlc_handler()?;

	info!("{:?}", args);

	println!("Doing some work... Press ctrl+c to exit...");
	ctrlc.recv().unwrap();

	log_handle.shutdown();
	Ok(ReturnCode::Success)
}

#[cfg(feature = "bug")]
pub fn main(args: Args) -> Result<ReturnCode> {
	let log_handle = create_logger()?;
	set_ctrlc_handler()?;

	info!("{:?}", args);

	use anyhow::{anyhow, Context};

	error!("A bug is about to occur!");
	let error = anyhow!("The bug feature is enabled");
	Err(error).context("Some context for where the error caused problems")?;

	log_handle.shutdown();
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
