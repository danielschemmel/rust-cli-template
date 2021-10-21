use anyhow::Result;
use tracing::{error, info};

#[cfg(not(feature = "subcommands"))]
#[derive(structopt::StructOpt, Debug)]
#[structopt(
	setting = structopt::clap::AppSettings::ColoredHelp,
	version = build_info::format!("{} {}\n\nBuilt from {} at {} with {} for {} on {}.", $.crate_info.version, $.profile, $.version_control, $.timestamp, $.compiler, $.compiler.target_triple, $.compiler.host_triple),
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
	version = build_info::format!("{} {}\n\nBuilt from {} at {} with {} for {} on {}.", $.crate_info.version, $.profile, $.version_control, $.timestamp, $.compiler, $.compiler.target_triple, $.compiler.host_triple),
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

fn create_logger() -> anyhow::Result<()> {
	use tracing_subscriber::EnvFilter;

	let env_filter = if std::env::var_os(EnvFilter::DEFAULT_ENV).is_some() {
		EnvFilter::try_from_default_env()?
	} else {
		EnvFilter::try_new(concat!("warn,", env!("CARGO_PKG_NAME"), "=debug"))?
	};

	tracing_subscriber::fmt()
		.with_target(true)
		.with_timer(tracing_subscriber::fmt::time::uptime())
		.with_level(true)
		.with_thread_ids(true)
		.with_thread_names(true)
		.with_env_filter(env_filter)
		.with_ansi(atty::is(atty::Stream::Stdout))
		.with_writer(std::io::stdout)
		// `cargo test` does not actually support stdout. With `with_test_writer`, logging is delivered as intended.
		.with_test_writer()
		// Set the subscriber as the default and also installs a mechanic for transmuting `log` events into `tracing` events.
		.init();

	Ok(())
}

/// Returns a receiver that is signalled when `SIGINT` is received, e.g., when the user hits Ctrl+C. If the receiver
/// is dropped or not serviced quickly enough, the program is terminated automatically.
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
	pub fn log_error() -> anyhow::Result<()> {
		create_logger()?;
		
		error!("Logging an \"error\"");

		Ok(())
	}
}
