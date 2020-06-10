use anyhow::{anyhow, Context, Result};
use tracing::{error, info, instrument};
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::FmtSubscriber;

use crate::errors::*;

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

fn create_logger() -> Result<(), LoggingError> {
	/// used if `RUST_LOG` is not set
	const DEFAULT_FILTER: &str = build_info::format!("warn,{}=debug", $.crate_info.name);

	/// used if the filter provided in `RUST_LOG` cannot be parsed
	const BACKUP_FILTER: &str = build_info::format!("warn,{}=debug", $.crate_info.name);

	let timer = tracing_subscriber::fmt::time::Uptime::default();
	// let timer = tracing_subscriber::fmt::time::Uptime::default();
	// let timer = tracing_subscriber::fmt::time::Uptime::default();

	let mut delayed_errors: Vec<anyhow::Error> = Vec::new();

	use std::env::VarError;
	let env_filter = match std::env::var(EnvFilter::DEFAULT_ENV) {
		Ok(value) => EnvFilter::try_new(&value)
			.with_context(|| {
				anyhow!(
					"Could not parse value of the environment variable `{}` ({:?}) as a logging filter. Using {:?} instead.",
					EnvFilter::DEFAULT_ENV,
					&value,
					BACKUP_FILTER
				)
			})
			.unwrap_or_else(|parse_error| {
				delayed_errors.push(parse_error);
				EnvFilter::new(BACKUP_FILTER)
			}),
		Err(err) => match &err {
			VarError::NotUnicode(_) => {
				delayed_errors.push(
					Result::<(), VarError>::Err(err)
						.with_context(|| {
							anyhow!(
								"Could not parse value of the environment variable `{}` as a logging filter. Using {:?} instead.",
								EnvFilter::DEFAULT_ENV,
								BACKUP_FILTER
							)
						})
						.unwrap_err(),
				);
				EnvFilter::new(BACKUP_FILTER)
			}
			VarError::NotPresent => EnvFilter::new(DEFAULT_FILTER),
		},
	};

	let builder = FmtSubscriber::builder()
		.with_env_filter(env_filter)
		.with_timer(timer)
		.with_writer(std::io::stderr)
		.with_ansi(atty::is(atty::Stream::Stderr));
	builder.try_init().map_err(|e| anyhow!(e))?;

	for err in &delayed_errors {
		error!("{}", err);
	}

	if delayed_errors.is_empty() {
		Ok(())
	} else {
		Err(delayed_errors.into_iter().next().unwrap().into())
	}
}

/// Returns a receiver that is signalled when `SIGINT` is received, e.g., when the user hits Ctrl+C. If the receiver
/// is dropped or not serviced quickly enough, the program is terminated automatically.
fn set_ctrlc_handler() -> Result<std::sync::mpsc::Receiver<()>> {
	let (sender, receiver) = std::sync::mpsc::sync_channel(1);

	ctrlc::set_handler(move || match sender.try_send(()) {
		Ok(()) => {
			eprintln!("\nReceived Ctrl+C...");
			info!("Received Ctrl+C");
		}
		Err(std::sync::mpsc::TrySendError::Full(())) => {
			eprintln!("\nReceived Ctrl+C again: Terminating forcefully!");
			error!("Received Ctrl+C again: Terminating forcefully!");
			std::process::exit(ReturnCode::CtrlC as i32);
		}
		Err(std::sync::mpsc::TrySendError::Disconnected(())) => {
			eprintln!("\nReceived Ctrl+C. Terminating now.");
			error!("Received Ctrl+C again. Terminating now.");
			std::process::exit(ReturnCode::CtrlC as i32);
		}
	})?;

	Ok(receiver)
}

#[cfg(not(feature = "bug"))]
#[instrument]
pub fn main(args: Args) -> Result<ReturnCode> {
	create_logger()?;
	let ctrlc = set_ctrlc_handler()?;

	info!("{:?}", args);

	println!("Doing some work... Press ctrl+c to exit...");
	ctrlc.recv().unwrap();

	Ok(ReturnCode::Success)
}

#[cfg(feature = "bug")]
#[instrument]
pub fn main(args: Args) -> Result<ReturnCode> {
	create_logger()?;
	set_ctrlc_handler()?;

	info!("{:?}", args);

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
