use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoggingError {
	#[error("Failed to create logger")]
	CreationFailure(
		#[from] anyhow::Error,
		// backtrace: std::backtrace::Backtrace,
	),
}
