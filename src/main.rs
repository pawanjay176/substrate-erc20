//! Substrate Node Template CLI library.

#![warn(missing_docs)]
#![warn(unused_extern_crates)]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate substrate_network as network;
#[macro_use]
extern crate substrate_executor;
#[macro_use]
extern crate substrate_service;

mod chain_spec;
mod service;
mod cli;

pub use substrate_cli::{VersionInfo, IntoExit, error};

fn run() -> cli::error::Result<()> {
	let version = VersionInfo {
		name: "Substrate ERC20",
		commit: env!("VERGEN_SHA_SHORT"),
		version: env!("CARGO_PKG_VERSION"),
		executable_name: "substrate-erc20",
		author: "gautamdhameja",
		description: "substrate-erc20",
		support_url: "support.anonymous.an",
	};
	cli::run(::std::env::args(), cli::Exit, version)
}

quick_main!(run);
