//! Substrate Node Template CLI library.
#![warn(missing_docs)]

mod chain_spec;
#[macro_use]
mod service;
mod cli;
mod command;

pub use sc_cli::VersionInfo;

fn main() -> sc_cli::Result<()> {
	let version = VersionInfo {
		name: "paracon",
		commit: env!("VERGEN_SHA_SHORT"),
		version: env!("CARGO_PKG_VERSION"),
		executable_name: "paracon",
		author: "Anonymous",
		description: "Template Node",
		support_url: "support.anonymous.an",
		copyright_start_year: 2020,
	};

	command::run(version)
}
