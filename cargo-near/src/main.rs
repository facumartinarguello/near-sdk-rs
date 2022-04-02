use anyhow::Result;
use clap::{AppSettings, Args, Parser, Subcommand};
use colored::Colorize;
use crate_metadata::CrateMetadata;
use env_logger;
use std::{convert::TryFrom, path::PathBuf};
use workspace::ManifestPath;

mod crate_metadata;
mod metadata;
mod util;
mod workspace;

#[derive(Debug, Parser)]
#[clap(bin_name = "cargo")]
#[clap(version = env!("CARGO_NEAR_CLI_IMPL_VERSION"))]
pub(crate) enum Opts {
    /// Utilities to develop Wasm smart contracts.
    #[clap(name = "near")]
    #[clap(version = env!("CARGO_NEAR_CLI_IMPL_VERSION"))]
    #[clap(setting = AppSettings::DeriveDisplayOrder)]
    Near(NearArgs),
}

#[derive(Debug, Args)]
pub(crate) struct NearArgs {
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Generates metadata for the contract
    #[clap(name = "metadata")]
    Metadata(MetadataCommand),
}

#[derive(Debug, clap::Args)]
#[clap(name = "metadata")]
pub struct MetadataCommand {
    /// Path to the `Cargo.toml` of the contract to build
    #[clap(long, parse(from_os_str))]
    manifest_path: Option<PathBuf>,
}

fn main() {
    env_logger::init();

    let Opts::Near(args) = Opts::parse();
    match exec(args.cmd) {
        Ok(()) => {}
        Err(err) => {
            eprintln!("{} {}", "ERROR:".bright_red().bold(), format!("{:?}", err).bright_red());
            std::process::exit(1);
        }
    }
}

fn exec(cmd: Command) -> Result<()> {
    match &cmd {
        Command::Metadata(metadata) => {
            let manifest_path = ManifestPath::try_from(metadata.manifest_path.as_ref())?;
            let crate_metadata = CrateMetadata::collect(&manifest_path)?;

            let _ = metadata::execute(&crate_metadata)?;
            Ok(())
        }
    }
}
