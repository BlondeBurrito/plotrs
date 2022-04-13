//! Constructs a scatter graph

use ron::de::from_reader;
use serde::Deserialize;
use std::fs::File;
use tracing::{debug, error, info};

use crate::{
	canvas::{
		build_title, build_x_axis_label, build_y_axis_label, draw_base_canvas, save_image,
		DataSymbol, FontSizes,
	},
	colours::*,
	data::load_data,
};
/// Specification of a scatter chart
#[derive(Debug, Deserialize)]
struct Scatter {
	/// The title of the graph
	title: String,
	/// Image size in pixels
	canvas_pixel_size: (u32, u32),
	/// X-axis label
	x_axis_label: String,
	/// Y-axis label
	y_axis_label: String,
	/// Defines where the data is and which parts to use
	data_sets: Vec<DataSet>,
	/// Should a legend be generated
	has_legend: bool,
}
/// The source of each data point and how it should be represented
#[derive(Debug, Deserialize)]
struct DataSet {
	/// Path to csv data
	data_path: String,
	/// Does the csv contain headers
	has_headers: bool,
	/// Which column in the csv contains the x-axis data
	x_axis_csv_column: usize,
	/// Optional, the column which contains an uncertainty measure
	x_axis_error_bar_csv_column: Option<usize>,
	/// Which column in the csv contains the y-axis data
	y_axis_csv_column: usize,
	/// Optional, the csv column which contains an uncertainty measure
	y_axis_error_bar_csv_column: Option<usize>,
	/// Name of the x-axis data, useful when generating a legend to distinguish sets
	name: String,
	/// The colour a data point should be plotted as
	colour: Colour,
	/// The shape used to represent the data point
	symbol: DataSymbol,
}

/// Creates a canvas and draws the scatter graph over it
pub fn scatter_builder(path: &str, output: &str, csv_delimiter: &str) {
	info!("Building scatter chart...");
	let scatter: Scatter = Scatter::deserialise(path);
	let mut canvas = draw_base_canvas(scatter.canvas_pixel_size);
	// calcualte font sizes
	let font_sizes = FontSizes::new(&scatter.canvas_pixel_size);
	// always create the y-axis label first as it's more complicated and will overwrite existing data
	build_y_axis_label(&mut canvas, scatter.y_axis_label, font_sizes.axis_font_size);
	build_x_axis_label(&mut canvas, scatter.x_axis_label, font_sizes.axis_font_size);
	build_title(&mut canvas, &scatter.title, font_sizes.title_font_size);

	// with the text drawn we need to know how much space is available for the axes

	// get the csv data content
	build_data_points(&scatter.data_sets, csv_delimiter);
	// let mut data = load_data(scatter.data_path, scatter.has_headers, csv_delimiter);

	// save the resulting image
	save_image(canvas, output, scatter.title);
}

impl Scatter {
	/// Based on a path deserialise a `.ron` file into a graph data structure
	fn deserialise(path: &str) -> Scatter {
		// attempt to open the .ron file
		let f = match File::open(path) {
			Ok(file) => file,
			Err(e) => {
				error!("Failed to open .ron file at {}, error: {:?}", path, e);
				std::process::exit(1)
			}
		};
		// attempt to deserialise the config data
		let scatter: Scatter = match from_reader(f) {
			Ok(x) => x,
			Err(e) => {
				error!(
					"Failed to load config, maybe you're missing a comma? Error: {}",
					e
				);
				std::process::exit(1);
			}
		};
		debug!("Ron config {:?}", &scatter);
		return scatter;
	}
}
/// Iterate through the data sets extracing the values from the csv and plot them
fn build_data_points(data_set: &Vec<DataSet>, csv_delimiter: &str) {
	// iterate over each set
	for set in data_set.iter() {
		// read the csv each set corresponds to
		let mut data = load_data(set.data_path.as_str(), set.has_headers, csv_delimiter);
		for record in data.records() {
			// extract the x-y values from each record
			match record {
				Ok(d) => {
					let x = match d.get(set.x_axis_csv_column) {
						Some(string_value) => match string_value.parse::<f32>() {
							Ok(value) => value,
							Err(e) => {
								error!(
									"Could not parse data in column {}, error: {}",
									set.x_axis_csv_column, e
								);
								std::process::exit(1);
							}
						},
						None => {
							error!(
								"Could not extract record in column {}",
								set.x_axis_csv_column
							);
							std::process::exit(1);
						}
					};
					let ux = match set.x_axis_error_bar_csv_column {
						Some(column) => match d.get(column) {
							Some(string_value) => match string_value.parse::<f32>() {
								Ok(value) => Some(value),
								Err(e) => {
									error!(
										"Could not parse data in column {} to f32, error: {}",
										set.x_axis_csv_column, e
									);
									std::process::exit(1);
								}
							},
							None => {
								error!(
									"Could not extract record in column {}",
									set.x_axis_csv_column
								);
								std::process::exit(1);
							}
						},
						None => None,
					};
					let y = match d.get(set.y_axis_csv_column) {
						Some(string_value) => match string_value.parse::<f32>() {
							Ok(value) => value,
							Err(e) => {
								error!(
									"Could not parse data in column {} to f32, error: {}",
									set.x_axis_csv_column, e
								);
								std::process::exit(1);
							}
						},
						None => {
							error!(
								"Could not extract record in column {}",
								set.x_axis_csv_column
							);
							std::process::exit(1);
						}
					};
					let uy = match set.y_axis_error_bar_csv_column {
						Some(column) => match d.get(column) {
							Some(string_value) => match string_value.parse::<f32>() {
								Ok(value) => Some(value),
								Err(e) => {
									error!(
										"Could not parse data in column {} to f32, error: {}",
										set.x_axis_csv_column, e
									);
									std::process::exit(1);
								}
							},
							None => {
								error!(
									"Could not extract record in column {}",
									set.x_axis_csv_column
								);
								std::process::exit(1);
							}
						},
						None => None,
					};
					// plot the value
				}
				Err(_) => todo!(),
			};
		}
	}
}

/// Representation of a point of data point on a Scatter graph
#[derive(Debug, Deserialize)]
struct DataPoint {
	x: f32,
	y: f32,
}
