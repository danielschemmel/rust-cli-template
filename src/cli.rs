use crate::errors::*;
use anyhow::Result;

#[cfg(not(feature = "subcommands"))]
#[derive(structopt::StructOpt, Debug)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
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
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
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

pub enum ReturnCode {
	Success = 0,
	ArgumentParsing = 1,
}

fn create_logger() -> Result<flexi_logger::ReconfigurationHandle, LoggingError> {
	flexi_logger::Logger::with_env_or_str("warn, application=debug")
		.format(flexi_logger::colored_with_thread)
		.start()
		.map_err(LoggingError::CreationFailure)
}

#[cfg(not(feature = "bug"))]
pub fn main(args: Args) -> Result<ReturnCode> {
	let _log_handle = create_logger()?;

	println!("{:?}", args);

	Ok(ReturnCode::Success)
}

#[cfg(feature = "bug")]
pub fn main(args: Args) -> Result<ReturnCode> {
	let _log_handle = create_logger()?;

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
	pub fn test_pretty_assertions_dummy() {
		assert_eq!(Some(1), Some(1));
		assert_ne!(Some(1), Some(2));
		#[cfg(feature = "bug")]
		assert_eq!(1000000, 1_000_000);
	}

	#[test]
	pub fn test_create_logger() {
		let log_handle = create_logger();
		assert!(log_handle.is_ok());
	}
}
