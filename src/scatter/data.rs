//! Reads data sets to identify data ranges, points for plotting and legend fields

use image::{ImageBuffer, Rgba};
use tracing::{debug, error};

use crate::{
	canvas::{legend::LegendField, plot::DataPoint},
	data::load_data,
};

use super::DataSet;

/// Reads the supplied csv files and finds the minimum and maximum x and y values across all sets.
/// This faciliates drawing values on axes and finding the ratio of pixels to a data point for plotting
pub fn get_data_bounds(data_set: &[DataSet], csv_delimiter: &str) -> ((f32, f32), (f32, f32)) {
	let mut min_x = f32::MAX;
	let mut min_ux = Some(f32::MAX); // TODO: unused at present
	let mut min_y = f32::MAX;
	let mut min_uy = Some(f32::MAX); // TODO: unused at present
	let mut max_x = f32::MIN;
	let mut max_ux = Some(f32::MIN); // TODO: unused at present
	let mut max_y = f32::MIN;
	let mut max_uy = Some(f32::MIN); // TODO: unused at present
								 // iterate over each set
	for set in data_set.iter() {
		// read the csv each set corresponds to
		let data = load_data(set.data_path.as_str(), set.has_headers, csv_delimiter);
		// used for error debuging
		let mut row = 1;
		for record in data.iter() {
			// x
			match record.get(set.x_axis_csv_column) {
				Some(string_value) => match string_value.parse::<f32>() {
					// determine the smallest and highest x value
					Ok(value) => {
						if value < min_x {
							min_x = value;
						}
						if value > max_x {
							max_x = value;
						}
					}
					Err(e) => {
						error!(
							"Could not parse data in column {}, row {} for x axis, error: {}",
							set.x_axis_csv_column, row, e
						);
						std::process::exit(1);
					}
				},
				None => {
					error!(
						"Could not extract record in column {} for x axis, row {}",
						set.x_axis_csv_column, row
					);
					std::process::exit(1);
				}
			};
			// ux
			match set.x_axis_error_bar_csv_column {
				Some(column) => match record.get(column) {
					Some(string_value) => match string_value.parse::<f32>() {
						// determine the smallest uncertainty value in x
						Ok(value) => {
							if value < min_ux.unwrap() {
								min_ux = Some(value)
							}
							if value > max_ux.unwrap() {
								max_ux = Some(value)
							}
						}
						Err(e) => {
							error!(
										"Could not parse data in column {}, row{}, to f32 for error bar x, error: {}",
										set.x_axis_csv_column, row, e
									);
							std::process::exit(1);
						}
					},
					None => {
						error!(
							"Could not extract record in column {} for error bar x, row {}",
							set.x_axis_csv_column, row
						);
						std::process::exit(1);
					}
				},
				None => min_ux = None,
			};
			//y
			match record.get(set.y_axis_csv_column) {
				Some(string_value) => match string_value.parse::<f32>() {
					// determine the smallest y value
					Ok(value) => {
						if value < min_y {
							min_y = value
						}
						if value > max_y {
							max_y = value
						}
					}
					Err(e) => {
						error!(
									"Could not parse data in column {}, row {} to f32 for y axis, error: {}",
									set.x_axis_csv_column, row, e
								);
						std::process::exit(1);
					}
				},
				None => {
					error!(
						"Could not extract record in column {} for y axis, row {}",
						set.x_axis_csv_column, row
					);
					std::process::exit(1);
				}
			};
			// uy
			match set.y_axis_error_bar_csv_column {
				Some(column) => match record.get(column) {
					// determine the smallest uncertainty value in y
					Some(string_value) => match string_value.parse::<f32>() {
						Ok(value) => {
							if value < min_uy.unwrap() {
								min_uy = Some(value)
							}
							if value > max_uy.unwrap() {
								max_uy = Some(value)
							}
						}
						Err(e) => {
							error!(
										"Could not parse data in column {}, row {} to f32 for y error bar, error: {}",
										set.x_axis_csv_column, row, e
									);
							std::process::exit(1);
						}
					},
					None => {
						error!(
							"Could not extract record in column {}, row {} for y error bar",
							set.x_axis_csv_column, row
						);
						std::process::exit(1);
					}
				},
				None => min_uy = None,
			};
			row += 1;
		}
	}
	((min_x, min_y), (max_x, max_y))
}

/// Iterate through the data sets extracting the values from the csv and plot them
pub fn build_data_points(
	data_set: &[DataSet],
	csv_delimiter: &str,
	canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
	x_scale_factor: f32,
	y_scale_factor: f32,
	origin_offset: (u32, u32),
) {
	debug!("Building data points...");
	// iterate over each set
	for set in data_set.iter() {
		// read the csv each set corresponds to
		let data = load_data(set.data_path.as_str(), set.has_headers, csv_delimiter);
		// used for error debuging
		let mut row = 1;
		for record in data.iter() {
			// extract the x-y values from each record
			let x = match record.get(set.x_axis_csv_column) {
				Some(string_value) => match string_value.parse::<f32>() {
					Ok(value) => value,
					Err(e) => {
						error!(
							"Could not parse data in column {}, row {} for x axis, error: {}",
							set.x_axis_csv_column, row, e
						);
						std::process::exit(1);
					}
				},
				None => {
					error!(
						"Could not extract record in column {}, row {} for x axis",
						set.x_axis_csv_column, row
					);
					std::process::exit(1);
				}
			};
			let ux = match set.x_axis_error_bar_csv_column {
				Some(column) => match record.get(column) {
					Some(string_value) => match string_value.parse::<f32>() {
						Ok(value) => Some(value),
						Err(e) => {
							error!(
										"Could not parse data in column {}, row{}, to f32 for error bar x, error: {}",
										set.x_axis_csv_column, row, e
									);
							std::process::exit(1);
						}
					},
					None => {
						error!(
							"Could not extract record in column {}, row {} for error bar x",
							set.x_axis_csv_column, row
						);
						std::process::exit(1);
					}
				},
				None => None,
			};
			let y = match record.get(set.y_axis_csv_column) {
				Some(string_value) => match string_value.parse::<f32>() {
					Ok(value) => value,
					Err(e) => {
						error!(
									"Could not parse data in column {}, row {} to f32 for y axis, error: {}",
									set.x_axis_csv_column, row, e
								);
						std::process::exit(1);
					}
				},
				None => {
					error!(
						"Could not extract record in column {}, row {} for y axis",
						set.x_axis_csv_column, row
					);
					std::process::exit(1);
				}
			};
			let uy = match set.y_axis_error_bar_csv_column {
				Some(column) => match record.get(column) {
					Some(string_value) => match string_value.parse::<f32>() {
						Ok(value) => Some(value),
						Err(e) => {
							error!(
										"Could not parse data in column {}, row {} to f32 for y error bar, error: {}",
										set.x_axis_csv_column, row, e
									);
							std::process::exit(1);
						}
					},
					None => {
						error!(
							"Could not extract record in column {}, row {} for y error bar",
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
				symbol_radius: set.symbol_radius,
				symbol_thickness: set.symbol_thickness,
			};
			point.draw_point(canvas, x_scale_factor, y_scale_factor, origin_offset);
			row += 1;
		}
	}
}
/// Extracts the colour, symbol and data set names for use in building a legend
pub fn get_legend_fields(data_set: &[DataSet]) -> Vec<LegendField> {
	let mut legend_fields: Vec<LegendField> = Vec::new();
	for set in data_set.iter() {
		legend_fields.push(LegendField {
			symbol: set.symbol,
			symbol_radius: set.symbol_radius,
			symbol_thickness: set.symbol_thickness,
			colour: set.colour,
			name: set.name.to_owned(),
		});
	}
	legend_fields
}
