#[derive(Fail, Debug)]
pub enum LoggingError {
	#[fail(display = "Failed to create logger")]
	CreationFailure(#[fail(cause)] flexi_logger::FlexiLoggerError),
}
