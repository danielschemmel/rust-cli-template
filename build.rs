fn main() {
	let mut flags = vergen::ConstantsFlags::all();
	// Removing the `SEMVER` flag will cause `env!("VERGEN_SEMVER")` to be generated from cargo (via `env::var("CARGO_PKG_VERSION")`) instead of git
	flags.remove(vergen::ConstantsFlags::SEMVER);
	vergen::generate_cargo_keys(flags).expect("Unable to generate the cargo keys!");
}
