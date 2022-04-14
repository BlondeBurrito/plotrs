//! Constructs a scatter graph

use ron::de::from_reader;
use serde::Deserialize;
use std::fs::File;
use tracing::{debug, error, info};

use crate::{
	canvas::{
		build_title, build_x_axis_label, build_y_axis_label, draw_base_canvas, draw_point,
		draw_xy_axes, save_image, DataSymbol, FontSizes, TEXT_PIXEL_BUFFER,
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
	let horizontal_pixels_used =
		build_y_axis_label(&mut canvas, scatter.y_axis_label, font_sizes.axis_font_size);
	let vertical_pixels_used_from_bottom =
		build_x_axis_label(&mut canvas, scatter.x_axis_label, font_sizes.axis_font_size);
	let vertical_pixels_used_from_top =
		build_title(&mut canvas, &scatter.title, font_sizes.title_font_size);
	// with the text drawn we need to know how much space is available for the axes
	// note origin is top-left corner of canvas, so minimum y value is greater than maximum y value
	// TODO: there must be a better way than hardcoding scaling factors below
	let legend_scale_factor = if scatter.has_legend { 3 } else { 1 };
	let axis_min = (
		horizontal_pixels_used + TEXT_PIXEL_BUFFER as u32 * 5,
		vertical_pixels_used_from_bottom - TEXT_PIXEL_BUFFER as u32 * 2,
	);
	let axis_max = (
		canvas.dimensions().0 - (axis_min.0 * legend_scale_factor),
		(vertical_pixels_used_from_top + TEXT_PIXEL_BUFFER as u32) * 2,
	);
	debug!("Origin axis placement {:?}", axis_min);
	debug!("Maximun axis placement {:?}", axis_max);
	draw_xy_axes(&mut canvas, axis_min, axis_max);
	// we need to know how the data scales to the length of axes for plotting,
	// ie. we need a scale factor of how many units of data there is to one pixel
	let x_axis_length = axis_max.0 - axis_min.0;
	let y_axis_length = axis_min.1 - axis_max.1;
	debug!("X-axis length {}", x_axis_length);
	debug!("Y-axis length {}", y_axis_length);
	// min x-y and max x-y
	let bounds: ((u32, u32), (u32, u32)) = get_data_bounds(&scatter.data_sets, csv_delimiter);

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
// Reads the supplied csv files and finds the minimum and maximum x and y values across all sets.
// This faciliates drawing values on axes and finding the ratio of pixels to a data point for plotting
fn get_data_bounds(data_set: &Vec<DataSet>, csv_delimiter: &str) -> ((u32, u32), (u32, u32)) {
	let mut min_x = 0.0;
	let mut min_ux = Some(0.0);
	let mut min_y = 0.0;
	let mut min_uy = Some(0.0);
	let mut max_x = 0.0;
	let mut max_ux = Some(0.0);
	let mut max_y = 0.0;
	let mut max_uy = Some(0.0);
	// iterate over each set
	for set in data_set.iter() {
		// read the csv each set corresponds to
		let mut data = load_data(set.data_path.as_str(), set.has_headers, csv_delimiter);
		// used for error debuging
		let mut row = 1;
		for record in data.records() {
			match record {
				Ok(d) => {
					// x
					match d.get(set.x_axis_csv_column) {
						Some(string_value) => match string_value.parse::<f32>() {
							// determine the smallest x value
							Ok(value) => {
								if value < min_x {min_x = value}
								if value > max_x {max_x = value}
							},
							Err(e) => {
								error!(
									"Could not parse data in column {}, row {}, error: {}",
									set.x_axis_csv_column, row, e
								);
								std::process::exit(1);
							}
						},
						None => {
							error!(
								"Could not extract record in column {}, row {}",
								set.x_axis_csv_column, row
							);
							std::process::exit(1);
						}
					};
					// ux
					match set.x_axis_error_bar_csv_column {
						Some(column) => match d.get(column) {
							Some(string_value) => match string_value.parse::<f32>() {
								// determine the smallest uncertainty value in x
								Ok(value) => {
									if value < min_ux.unwrap() { min_ux = Some(value)}
									if value > max_ux.unwrap() { max_ux = Some(value)}
								},
								Err(e) => {
									error!(
										"Could not parse data in column {}, row{}, to f32, error: {}",
										set.x_axis_csv_column, row, e
									);
									std::process::exit(1);
								}
							},
							None => {
								error!(
									"Could not extract record in column {}, row {}",
									set.x_axis_csv_column, row
								);
								std::process::exit(1);
							}
						},
						None => min_ux = None,
					};
					//y
					match d.get(set.y_axis_csv_column) {
						Some(string_value) => match string_value.parse::<f32>() {
							// determine the smallest y value
							Ok(value) => {
								if value < min_y {min_y = value}
								if value > max_y {max_y = value}
							},
							Err(e) => {
								error!(
									"Could not parse data in column {}, row {} to f32, error: {}",
									set.x_axis_csv_column, row, e
								);
								std::process::exit(1);
							}
						},
						None => {
							error!(
								"Could not extract record in column {}, row {}",
								set.x_axis_csv_column, row
							);
							std::process::exit(1);
						}
					};
					// uy
					match set.y_axis_error_bar_csv_column {
						Some(column) => match d.get(column) {
							// determine the smallest uncertainty value in y
							Some(string_value) => match string_value.parse::<f32>() {
								Ok(value) => {
									if value < min_uy.unwrap() { min_uy = Some(value)}
									if value > max_uy.unwrap() { max_uy = Some(value)}
								},
								Err(e) => {
									error!(
										"Could not parse data in column {}, row {} to f32, error: {}",
										set.x_axis_csv_column, row, e
									);
									std::process::exit(1);
								}
							},
							None => {
								error!(
									"Could not extract record in column {}, row {}",
									set.x_axis_csv_column, row
								);
								std::process::exit(1);
							}
						},
						None => min_uy = None,
					};
				}
				Err(e) => {
					error!("Cannot read csv record: {}", e);
					std::process::exit(1);
				},
			}
		row +=1;
		}
	}
	return ((min_x as u32, min_y as u32),(max_x as u32, max_y as u32))
}
/// Iterate through the data sets extracing the values from the csv and plot them
fn build_data_points(data_set: &Vec<DataSet>, csv_delimiter: &str) {
	// iterate over each set
	for set in data_set.iter() {
		// read the csv each set corresponds to
		let mut data = load_data(set.data_path.as_str(), set.has_headers, csv_delimiter);
		// used for error debuging
		let mut row = 1;
		for record in data.records() {
			// extract the x-y values from each record
			match record {
				Ok(d) => {
					let x = match d.get(set.x_axis_csv_column) {
						Some(string_value) => match string_value.parse::<f32>() {
							Ok(value) => value,
							Err(e) => {
								error!(
									"Could not parse data in column {}, row {}, error: {}",
									set.x_axis_csv_column, row, e
								);
								std::process::exit(1);
							}
						},
						None => {
							error!(
								"Could not extract record in column {}, row {}",
								set.x_axis_csv_column, row
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
										"Could not parse data in column {}, row{}, to f32, error: {}",
										set.x_axis_csv_column, row, e
									);
									std::process::exit(1);
								}
							},
							None => {
								error!(
									"Could not extract record in column {}, row {}",
									set.x_axis_csv_column, row
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
									"Could not parse data in column {}, row {} to f32, error: {}",
									set.x_axis_csv_column, row, e
								);
								std::process::exit(1);
							}
						},
						None => {
							error!(
								"Could not extract record in column {}, row {}",
								set.x_axis_csv_column, row
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
										"Could not parse data in column {}, row {} to f32, error: {}",
										set.x_axis_csv_column, row, e
									);
									std::process::exit(1);
								}
							},
							None => {
								error!(
									"Could not extract record in column {}, row {}",
									set.x_axis_csv_column, row
								);
								std::process::exit(1);
							}
						},
						None => None,
					};
					// plot the value
					let point: DataPoint = DataPoint {
						x,
						ux,
						y,
						uy,
						colour: set.colour,
						symbol: set.symbol,
					};
					draw_point(point);
				}
				Err(e) => {
					error!("Cannot read csv record: {}", e);
					std::process::exit(1);
				},
			};
			row +=1;
		}
	}
}

/// Representation of a point of data point on a Scatter graph
#[derive(Debug, Deserialize)]
pub struct DataPoint {
	/// An x data point
	pub x: f32,
	/// Uncertainty in x
	pub ux: Option<f32>,
	/// A  data point
	pub y: f32,
	/// Uncertainty in y
	pub uy: Option<f32>,
	/// The colour of the point
	pub colour: Colour,
	/// Symbol to represent point
	pub symbol: DataSymbol,
}
