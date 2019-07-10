error_chain! {
	foreign_links {
		Io(::std::io::Error);
	}
	errors {
		SomeBug(some_cause: String) {
			description("some bug")
			display("some bug occurred due to '{}'", some_cause)
		}
	}
}
