//! A CLI app for plotting csv data sets onto a graph. It works by reading a graph definition from a `.ron` file, then extracts data from one or more csv files and produces a `.png` image.
//!
//! ## Features
//!
//! * Overlay best fit curves onto your graph
//! * Graph element/component positions and sizes are dynamically calculated based on the size of the image you want
//! * Multiple colours and symbols can be used to plot data sets
//! * Data can be sourced from one or more csv files - you're simply targeting certain columns in a given file for extraction
//! * Error bars - plot uncertainty in `x` and `y` singly or jointly
//!
//! ## Install
//!
//! `cargo install plotrs`
//!
//! ## How To Use
//!
//! Create a `.ron` file containing the configuration of your desired chart and generate a `png` with:
//!
//! ```bash
//! plotrs -g <graph_type> -c <path_to_config_ron_file> -o <dir_for_output_png>
//! ```
//!
//! E.g
//!
//! ```bash
//! plotrs -g scatter -c scatter_config.ron -o here/please
//! ```

use clap::Parser;
use font_kit::{
	family_name::FamilyName, handle::Handle, properties::Properties, source::SystemSource,
};
use rusttype::Font;
use std::fs;
use tracing::error;
use tracing::{self, trace};
mod canvas;
mod colours;
mod data;
mod scatter;

/// Programme arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
	/// Graph type to generate, accepted values: "scatter"
	#[clap(short, long)]
	graph: String,
	/// Relative path to a .ron config file containing graph metadata
	#[clap(short, long)]
	config: String,
	/// Relative path to a directory where your png will be placed. Png names are based on our config graph title
	#[clap(short, long, default_value_t = String::from("."))]
	output: String,
	/// Override the default csv delimiter "," with your own, e.g ";"
	#[clap(long, default_value_t = String::from(","))]
	csv_delimiter: String,
	/// Set the verbosity level with a series of `v`'s, e.g `-v` or `-vv`
	#[clap(flatten)]
	verbose: clap_verbosity_flag::Verbosity<clap_verbosity_flag::InfoLevel>,
}
/// Process Cli arguments and call appropriate methods for graph creation
fn main() {
	let args = Args::parse();
	// set various logging levels
	let log_level = match args.verbose.log_level() {
		Some(v) => match v {
			log::Level::Error => tracing::Level::ERROR,
			log::Level::Warn => tracing::Level::WARN,
			log::Level::Info => tracing::Level::INFO,
			log::Level::Debug => tracing::Level::DEBUG,
			log::Level::Trace => tracing::Level::TRACE,
		},
		None => tracing::Level::INFO,
	};
	tracing_subscriber::fmt().with_max_level(log_level).init();

	if args.csv_delimiter.len() != 1 {
		error!("Csv delimiter must be a single character");
		std::process::exit(1);
	}

	match args.graph.to_lowercase().as_str() {
		"scatter" => scatter::scatter_builder(
			args.config.as_str(),
			args.output.as_str(),
			args.csv_delimiter.as_str(),
		),
		_ => {
			error!("Invalid graph type selected. Valid graphs are 'scatter'.");
			std::process::exit(1);
		}
	}
}

/// Retrieves a system font
pub fn get_system_font() -> Font<'static> {
	let font_path: Handle = SystemSource::new()
		.select_best_match(&[FamilyName::SansSerif], &Properties::new())
		.unwrap();
	match font_path {
		Handle::Path {
			path,
			font_index: _,
		} => {
			trace!("Font path: {:?}", path);
			let bytes = fs::read(path.as_path()).unwrap();
			match Font::try_from_vec(bytes) {
				Some(x) => x,
				None => {
					error!("Could not construct/find a suitable font");
					std::process::exit(1);
				}
			}
		}
		Handle::Memory {
			bytes,
			font_index: _,
		} => match Font::try_from_vec(bytes.to_vec()) {
			Some(x) => x,
			None => {
				error!("Could not construct/find a suitable font");
				std::process::exit(1);
			}
		},
	}
}
