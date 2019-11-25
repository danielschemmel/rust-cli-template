use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoggingError {
	#[error("Failed to create logger")]
	CreationFailure(
		#[from] flexi_logger::FlexiLoggerError,
		// backtrace: std::backtrace::Backtrace,
	),
}
