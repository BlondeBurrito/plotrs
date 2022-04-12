//! A CLI app for plotting a data set onto a variety of graphs

use clap::Parser;
use font_kit::{
	family_name::FamilyName, handle::Handle, properties::Properties, source::SystemSource,
};
use rusttype::Font;
use std::fs;
use tracing::{debug, error, Level};
use tracing_subscriber;
mod canvas;
mod colours;
mod scatter;

/// Program text
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
	/// Graph type to generate
	#[clap(short, long)]
	graph: String,
	/// Relative path to a .ron config file containing graph metadata
	#[clap(short, long)]
	config: String,
	/// Relative path a directory where your png will be placed
	#[clap(short, long, default_value_t = String::from("."))]
	output: String,
	/// Enables debug logging which will log data about font sizes, graph element positions and more
	#[clap(short, long)]
	verbose: bool,
}

fn main() {
	let args = Args::parse();
	let mut log_level = Level::INFO;
	// enable debug logging
	if args.verbose {
		log_level = Level::DEBUG
	}
	tracing_subscriber::fmt().with_max_level(log_level).init();

	match args.graph.to_lowercase().as_str() {
		"scatter" => scatter::scatter_builder(args.config.as_str(), args.output.as_str()),
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
			debug!("Font path: {:?}", path);
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
