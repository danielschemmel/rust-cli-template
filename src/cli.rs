#[cfg(not(feature = "camino"))]
use std::path::PathBuf;

use anyhow::Result;
#[cfg(feature = "camino")]
use camino::Utf8PathBuf as PathBuf;
use tracing::{error, info};

#[cfg(not(feature = "subcommands"))]
#[allow(dead_code)] // this is just a sample application
#[derive(clap::Parser, Debug)]
#[clap(
	version = build_info::format!(
		"{} {}\nBuilt from {} at {} with {} for {} on {}. Enabled features: {}.",
		$.crate_info.version,
		$.profile,
		$.version_control,
		$.timestamp,
		$.compiler,
		$.compiler.target_triple,
		$.compiler.host_triple,
		$.crate_info.enabled_features),
	about,
	long_about = None,
)]
pub struct Args {
	/// Something, something, path
	#[clap(name = "FILE")]
	file: PathBuf,
	/// Optional blubber command
	#[clap(name = "BLUBBER")]
	blubber: Option<String>,
	/// Port to listen on.
	#[clap(short = 'p', long = "port", env = "PORT", default_value = "8080")]
	port: u16,
	/// Address to listen on.
	#[clap(short = 'a', long = "address", default_value = "127.0.0.1")]
	address: String,
}

#[cfg(feature = "subcommands")]
#[allow(dead_code)] // this is just a sample application
#[derive(clap::Parser, Debug)]
#[clap(
	about,
	long_about = None,
	version = build_info::format!(
		"{} {}\nBuilt from {} at {} with {} for {} on {}. Enabled features: {}.",
		$.crate_info.version,
		$.profile,
		$.version_control,
		$.timestamp,
		$.compiler,
		$.compiler.target_triple,
		$.compiler.host_triple,
		$.crate_info.enabled_features),
	about,
	long_about = None,
)]
pub enum Args {
	#[clap(name = "file-based")]
	FileBased {
		/// Something, something, path
		#[clap(name = "FILE")]
		file: PathBuf,
		/// Optional blubber command
		#[clap(name = "BLUBBER")]
		blubber: Option<String>,
	},
	#[clap(name = "network-based")]
	NetworkBased {
		/// Port to listen on.
		#[clap(short = 'p', long = "port", env = "PORT", default_value = "8080")]
		port: u16,
		/// Address to listen on.
		#[clap(short = 'a', long = "address", default_value = "127.0.0.1")]
		address: String,
	},
}

#[derive(Copy, Clone)]
pub enum ReturnCode {
	Success = 0,
	CtrlC = -1,
	ArgumentParsing = 1,
}

fn create_logger() -> anyhow::Result<()> {
	use tracing_subscriber::EnvFilter;

	let env_filter = if std::env::var_os(EnvFilter::DEFAULT_ENV).is_some() {
		EnvFilter::try_from_default_env()?
	} else {
		EnvFilter::try_new(concat!("warn,", env!("CARGO_PKG_NAME"), "=debug"))?
	};

	let subscriber = tracing_subscriber::fmt()
		.with_target(true)
		.with_timer(tracing_subscriber::fmt::time::uptime())
		.with_level(true)
		.with_thread_ids(true)
		.with_thread_names(true)
		.with_env_filter(env_filter);

	#[cfg(not(test))]
	let subscriber = subscriber
		.with_ansi(atty::is(atty::Stream::Stdout))
		.with_writer(std::io::stdout);

	#[cfg(test)]
	// `cargo test` does not actually support stdout. With `with_test_writer`, logging is delivered as intended.
	let subscriber = subscriber.with_test_writer();

	// Set the subscriber as the default and also installs a mechanic for transmuting `log` events into `tracing` events.
	subscriber.init();

	Ok(())
}

/// Returns a receiver that is signalled when `SIGINT` is received, e.g., when the user hits Ctrl+C. If the receiver
/// has been dropped or not serviced quickly enough, the program is terminated automatically upon receiving a signal.
fn set_ctrlc_handler() -> Result<tokio::sync::mpsc::Receiver<()>> {
	let (sender, receiver) = tokio::sync::mpsc::channel(1);

	ctrlc::set_handler(move || match sender.try_send(()) {
		Ok(()) => {
			eprintln!("\nReceived Ctrl+C...");
			info!("Received Ctrl+C");
		}
		Err(tokio::sync::mpsc::error::TrySendError::Full(())) => {
			eprintln!("\nReceived Ctrl+C again: Terminating forcefully!");
			error!("Received Ctrl+C again: Terminating forcefully!");
			std::process::exit(ReturnCode::CtrlC as i32);
		}
		Err(tokio::sync::mpsc::error::TrySendError::Closed(())) => {
			eprintln!("\nReceived Ctrl+C. Terminating now.");
			error!("Received Ctrl+C again. Terminating now.");
			std::process::exit(ReturnCode::CtrlC as i32);
		}
	})?;

	Ok(receiver)
}

#[cfg(not(feature = "bug"))]
pub async fn main(args: Args) -> Result<ReturnCode> {
	create_logger()?;
	let mut ctrlc = set_ctrlc_handler()?;

	info!("{:?}", args);

	println!("Doing some work... Press ctrl+c to exit...");
	ctrlc.recv().await;

	Ok(ReturnCode::Success)
}

#[cfg(feature = "bug")]
pub async fn main(args: Args) -> Result<ReturnCode> {
	create_logger()?;
	set_ctrlc_handler()?;

	info!("{:?}", args);

	use anyhow::{anyhow, Context};

	error!("A bug is about to occur!");
	let error = anyhow!("The bug feature is enabled");
	Err(error).context("Some context for where the error caused problems")?;

	Ok(ReturnCode::Success)
}

#[cfg(test)]
mod test {
	use pretty_assertions::{assert_eq, assert_ne};

	use super::*;

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
	pub fn log_error() -> anyhow::Result<()> {
		create_logger()?;

		error!("Logging an \"error\"");

		Ok(())
	}
}
