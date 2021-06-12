use std::path::PathBuf;
use structopt::clap::AppSettings;
use structopt::StructOpt;

/// Command-line arguments to parse.
#[derive(Debug, StructOpt)]
#[structopt(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    global_settings(&[
        AppSettings::ColorAuto,
        AppSettings::ColoredHelp,
        AppSettings::DeriveDisplayOrder,
    ]),
    rename_all_env = "screaming-snake"
)]
pub struct Opt {
	/// Activates the debug mode
	#[structopt(short, long)]
	pub debug:      bool,
	/// Sets the configuration file.
	#[structopt(
		short,
		long,
		env,
		value_name = "PATH",
		default_value = "cliff.toml"
	)]
	pub config:     String,
	/// Sets the repository to parse commits from.
	#[structopt(short, long, env, value_name = "PATH")]
	pub repository: Option<PathBuf>,
	/// Prepends entries to the given changelog file.
	#[structopt(short = "p", long, env, value_name = "PATH")]
	pub changelog:  Option<PathBuf>,
	/// Sets the tag for the latest version.
	#[structopt(short, long, env, value_name = "TAG", allow_hyphen_values = true)]
	pub tag:        Option<String>,
	/// Processes the commits starting from the latest tag.
	#[structopt(short, long)]
	pub latest:     bool,
	/// Processes the commits that do not belong to a tag.
	#[structopt(short, long)]
	pub unreleased: bool,
	/// Strips the given parts from the changelog.
	#[structopt(short, long, possible_values = &["header", "footer", "all"])]
	pub strip:      Option<String>,
	/// Sets the commit range to process.
	pub range:      Option<String>,
}