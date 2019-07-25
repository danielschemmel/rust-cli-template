mod logging_error;
pub use logging_error::LoggingError;

pub trait FailExt {
	fn display_chain(&self) -> DisplayChain;
}

impl FailExt for failure::Fail {
	fn display_chain(&self) -> DisplayChain {
		DisplayChain::new(self)
	}
}

impl FailExt for failure::Error {
	fn display_chain(&self) -> DisplayChain {
		DisplayChain::new(self.as_fail())
	}
}

impl<E> FailExt for failure::Context<E>
where
	E: std::fmt::Display + Send + Sync + 'static,
{
	fn display_chain(&self) -> DisplayChain {
		DisplayChain::new(self)
	}
}

pub struct DisplayChain<'a> {
	error: &'a failure::Fail,
}

impl<'a> DisplayChain<'a> {
	fn new(error: &failure::Fail) -> DisplayChain {
		DisplayChain { error }
	}
}

fn backtraces_enabled() -> bool {
	std::env::var("RUST_BACKTRACE").ok().is_some()
}

impl<'a> std::fmt::Display for DisplayChain<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		writeln!(f, "error: {}", self.error)?;
		for e in self.error.iter_causes() {
			writeln!(f, "caused by: {}", e)?;
		}

		writeln!(f)?;
		if backtraces_enabled() {
			if let Some(backtrace) = self.error.backtrace() {
				writeln!(f, "{}", backtrace)?;
			} else {
				writeln!(f, "No backtrace available")?;
			}
		} else {
			writeln!(
				f,
				"Please set the environment variable RUST_BACKTRACE to 1 to enable backtraces."
			)?;
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn display_chain_error_1() {
		let error: failure::Error = format_err!("This is an error");
		let result = format!("{}", error.display_chain());
		println!("{:?}", result);
		assert!(result.starts_with("error: This is an error\n\n"));
	}

	#[test]
	fn display_chain_error_2() {
		let error: failure::Error = format_err!("This is an error").context("In a context").into();
		let result = format!("{}", error.display_chain());
		println!("{:?}", result);
		assert!(result.starts_with("error: In a context\ncaused by: This is an error\n\n"));
	}

	#[test]
	fn display_chain_error_3() {
		let error: failure::Error = format_err!("This is an error").context("In a context").into();
		let error: failure::Error = error.context(format_err!("In another context")).into();
		let result = format!("{}", error.display_chain());
		println!("{:?}", result);
		assert!(result.starts_with("error: In another context\ncaused by: In a context\ncaused by: This is an error\n\n"));
	}

	#[test]
	fn display_chain_context_1() {
		let error: failure::Context<&str> = format_err!("This is an error").context("In a context");
		let result = format!("{}", error.display_chain());
		println!("{:?}", result);
		assert!(result.starts_with("error: In a context\ncaused by: This is an error\n\n"));
	}

	#[test]
	fn display_chain_context_2() {
		let error: failure::Error = format_err!("This is an error").context("In a context").into();
		let error: failure::Context<failure::Error> = error.context(format_err!("In another context"));
		let result = format!("{}", error.display_chain());
		println!("{:?}", result);
		assert!(result.starts_with("error: In another context\ncaused by: In a context\ncaused by: This is an error\n\n"));
	}
}
