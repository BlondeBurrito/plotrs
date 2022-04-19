//! Constructs a scatter graph

use ron::de::from_reader;
use serde::Deserialize;
use std::fs::File;
use tracing::{debug, error, info};

mod data;

use crate::{
	canvas::{
		axes::build_x_axis_label,
		axes::build_y_axis_label,
		axes::{
			draw_xy_axes, get_x_axis_pixel_length, get_xy_axis_pixel_max, get_xy_axis_pixel_origin,
			get_y_axis_pixel_length,
		},
		best_fit::BestFit,
		draw_base_canvas,
		glyphs::FontSizes,
		legend::{build_legend},
		plot::DataSymbol,
		save_image,
		title::build_title, quadrants::get_quadrants, quadrants::Quadrants
	},
	colours::*,
	scatter::data::{get_data_bounds, get_legend_fields, build_data_points},
};
/// Specification of a scatter graph
#[derive(Debug, Deserialize)]
struct Scatter {
	/// The title of the graph
	title: String,
	/// Image size in pixels
	canvas_pixel_size: (u32, u32),
	/// X-axis label
	x_axis_label: String,
	/// Number of times the x-axis will be divided to show your data scale. Advised to make it a ratio of your largest x value
	x_axis_resolution: u32,
	/// Y-axis label
	y_axis_label: String,
	/// Number of times the y-axis will be divided to show your data scale. Advised to make it a ratio of your largest y value
	y_axis_resolution: u32,
	/// Should the graph has a light grey background grid
	has_grid: bool,
	/// Should a legend be generated
	has_legend: bool,
	/// Defines where the data is and which parts to use
	data_sets: Vec<DataSet>,
}
/// The source of each data set and how it should be represented
#[derive(Debug, Deserialize)]
pub struct DataSet {
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
	/// Name of the data set, useful when generating a legend to distinguish sets
	name: String,
	/// The colour a data point should be plotted as
	colour: Colour,
	/// The shape used to represent the data point
	symbol: DataSymbol,
	/// The size of a drawn symbol in (1+ symbol_radius) pixels
	symbol_radius: u32,
	/// The thinkness of a drawn symbol in (1 + symbol_thickness) pixels
	symbol_thickness: u32,
	/// Optional, a type of best fit line to draw
	best_fit: Option<BestFit>,
}

/// Creates a canvas and draws the scatter graph over it
pub fn scatter_builder(path: &str, output: &str, csv_delimiter: &str) {
	info!("Building scatter chart...");
	let scatter: Scatter = Scatter::deserialise(path);
	info!("Drawing canvas...");
	let mut canvas = draw_base_canvas(scatter.canvas_pixel_size);
	// Calcualte font sizes
	info!("Calculating font sizes...");
	let font_sizes = FontSizes::new(&scatter.canvas_pixel_size);
	// Place the title at the top of the canvas and tell us how many v-pixels have been used
	info!("Building title...");
	let vertical_pixels_used_from_top =
		build_title(&mut canvas, &scatter.title, font_sizes.title_font_size);
	// Find the size of the data - this tells us whether any axis requires a negative range.
	// Of the form `(min_x, min_y), (max_x, max_y)`
	info!("Finding min and max range of data...");
	let (min_xy, max_xy): ((f32, f32), (f32, f32)) = get_data_bounds(&scatter.data_sets, csv_delimiter);
	// We want to create buffer space around our bounds so data points are not plotted directly on an axis, if
	// large symbols are used for plotting they may obscure data labels on an axis. We scale the bounds by 10%
	// taking care to shrink and expand bounds based on their sign
	let min_x_scaled = if min_xy.0.is_sign_positive() {min_xy.0 / 1.1} else {min_xy.0 * 1.1};
	let min_y_scaled = if min_xy.1.is_sign_positive() {min_xy.1 / 1.1} else {min_xy.1 * 1.1};
	let max_x_scaled = if max_xy.0.is_sign_positive() {max_xy.0 * 1.1} else {max_xy.0 / 1.1};
	let max_y_scaled = if max_xy.1.is_sign_positive() {max_xy.1 * 1.1} else {max_xy.1 / 1.1};
	let min_xy_scaled = (min_x_scaled as i32, min_y_scaled as i32);
	debug!("Minimum x-y with buffer space {:?}", min_xy_scaled);
	let max_xy_scaled = (max_x_scaled as i32, max_y_scaled as i32);
	debug!("Maximum x-y with buffer space {:?}", max_xy_scaled);
	// With the scaled values we find which quadrants to draw are valid
	let quadrants = get_quadrants(min_xy_scaled, max_xy_scaled);
	info!("Quadrants to draw based on data set {:?}", quadrants);
	info!("Building y-axis label...");
	// Draws the y-axis label and returns the amount of total pixel space used up by all graph elements/compoents
	let horizontal_pixels_used =
		build_y_axis_label(&mut canvas, scatter.y_axis_label, font_sizes.axis_font_size, &quadrants, vertical_pixels_used_from_top);
	info!("Building x-axis label...");
	// Draws the x-axis label and returns TODO
	let vertical_pixels_used_from_bottom =
		build_x_axis_label(&mut canvas, scatter.x_axis_label, font_sizes.axis_font_size, &quadrants);
	// legend_scale_factor decides how much horizontal space should be reserved for a legend
	//TODO: there must be a nicer way reserve some legend space, for true 2 is way too big
	let legend_scale_factor = if scatter.has_legend { 1 } else { 1 };
	// With the text drawn we can calculate the rectangular space for the axes, represrnted as two tuples
	// pinpointing the bottom left origin of the graph and the top right corner.
	// Pixel position of axes origin
	let axis_min: (u32, u32) = get_xy_axis_pixel_origin(
		horizontal_pixels_used,
		vertical_pixels_used_from_bottom,
		canvas.dimensions(),
	);
	// Pixel position showing the maximum extents of the axes
	let axis_max: (u32, u32) = get_xy_axis_pixel_max(
		axis_min,
		vertical_pixels_used_from_top,
		legend_scale_factor,
		canvas.dimensions(),
		scatter.x_axis_resolution,
		scatter.y_axis_resolution,
	);
	debug!("Origin axis placement {:?}", axis_min);
	debug!("Maximun axis placement {:?}", axis_max);
	// We need to know how the csv data scales to the length of axes for plotting,
	// ie. we need a scale factor of how many units of data there is to one pixel
	// First we need the axis length
	let x_axis_length = get_x_axis_pixel_length(axis_min.0, axis_max.0);
	// Y-axis max is in fact a smaller number due to canvas image origin
	let y_axis_length = get_y_axis_pixel_length(axis_max.1, axis_min.1);
	debug!("X-axis length {}", x_axis_length);
	debug!("Y-axis length {}", y_axis_length);
	// next we need to know the 'size' of our data, min x-y and max x-y
	let bounds: ((f32, f32), (f32, f32)) = get_data_bounds(&scatter.data_sets, csv_delimiter);
	debug!("Min and max data points: {:?}", bounds);
	// We want to create buffer space around our bounds so data points are not plotted directly on an axis, if
	// large symbols are used for plotting they may obscure data labels on an axis
	let bounds_with_buffer: ((u32, u32), (u32, u32)) = (
		((bounds.0 .0 / 1.1) as u32, (bounds.0 .1 / 1.1) as u32),
		((bounds.1 .0 * 1.1) as u32, (bounds.1 .1 * 1.1) as u32),
	);
	debug!("Axes bounds {:?}", bounds_with_buffer);
	let x_data_min_max_limits: (u32, u32) = (bounds_with_buffer.0 .0, bounds_with_buffer.1 .0);
	let y_data_min_max_limits: (u32, u32) = (bounds_with_buffer.0 .1, bounds_with_buffer.1 .1);
	// Now we can find the number of axis units per x and y
	//TODO: single row data set causes divide by zero
	let x_axis_data_scale_factor: f32 =
		x_axis_length as f32 / (bounds_with_buffer.1 .0 as f32 - bounds_with_buffer.0 .0 as f32);
	let y_axis_data_scale_factor: f32 =
		y_axis_length as f32 / (bounds_with_buffer.1 .1 as f32 - bounds_with_buffer.0 .1 as f32);
	debug!("X-axis scale factor {}", x_axis_data_scale_factor);
	debug!("Y-axis scale factor {}", y_axis_data_scale_factor);

	draw_xy_axes(
		&mut canvas,
		axis_min,
		axis_max,
		x_axis_length,
		y_axis_length,
		x_data_min_max_limits,
		y_data_min_max_limits,
		font_sizes.axis_unit_font_size,
		scatter.has_grid,
		scatter.x_axis_resolution,
		scatter.y_axis_resolution,
	);
	// optionally build the legend
	if scatter.has_legend {
		let legend_fields = get_legend_fields(&scatter.data_sets);
		let legend_origin: (u32, u32) = (axis_max.0, axis_max.1 * 2);
		build_legend(
			&mut canvas,
			legend_origin,
			legend_fields,
			font_sizes.legend_font_size,
		);
	}
	// if a line of best fit has been specified then draw it
	for set in &scatter.data_sets {
		match &set.best_fit {
			Some(curve) => {
				info!("Plotting best fit...");
				let points = curve.find_coordinates(
					x_data_min_max_limits.0,
					x_data_min_max_limits.1,
					y_data_min_max_limits.0,
					y_data_min_max_limits.1,
					scatter.canvas_pixel_size.0,
				);
				let origin_offset = (axis_min.0, axis_min.1);
				for p in points.iter() {
					p.draw_point(
						&mut canvas,
						x_axis_data_scale_factor,
						y_axis_data_scale_factor,
						origin_offset,
					);
				}
			}
			None => {}
		}
	}
	// get the csv data content and plot it
	build_data_points(
		&scatter.data_sets,
		csv_delimiter,
		&mut canvas,
		x_axis_data_scale_factor,
		y_axis_data_scale_factor,
		(axis_min.0, axis_min.1),
	);

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
