//!

use image::{ImageBuffer, Rgba};
use serde::Deserialize;
use tracing::{trace, warn};

use crate::colours::Colour;

/// The shape a plotted data point should take
#[derive(Debug, Deserialize, Copy, Clone)]
pub enum DataSymbol {
	Cross,
	Circle,
	Triangle,
	Square,
	Point,
}

impl DataSymbol {
	/// Based on the `DataSymbol` type find the pixels that make up its shape
	pub fn find_pixels(self, origin: (u32, u32), thickness: u32, radius: u32) -> Vec<(u32, u32)> {
		let mut pixel_coords: Vec<(u32, u32)> = Vec::new();
		match self {
			DataSymbol::Cross => {
				pixel_coords.push(origin);
				// Ensure even arm lengths
				let length = if (radius + 1) & 1 == 1 {
					radius + 2
				} else {
					radius + 1
				};
				for i in 0..length {
					// right side of cross
					pixel_coords.push((origin.0 + i, origin.1));
					for n in 0..=thickness {
						pixel_coords.push((origin.0 + i, origin.1 + n));
						pixel_coords.push((origin.0 + i, origin.1 - n));
					}
					// left side of cross
					pixel_coords.push((origin.0 - i, origin.1));
					for n in 0..=thickness {
						pixel_coords.push((origin.0 - i, origin.1 + n));
						pixel_coords.push((origin.0 - i, origin.1 - n));
					}
					// top northwards part of cross
					pixel_coords.push((origin.0, origin.1 + i));
					for n in 0..=thickness {
						pixel_coords.push((origin.0 + n, origin.1 + i));
						pixel_coords.push((origin.0 - n, origin.1 + i));
					}
					// bottom southwards part of cross
					pixel_coords.push((origin.0, origin.1 - i));
					for n in 0..=thickness {
						pixel_coords.push((origin.0 + n, origin.1 - i));
						pixel_coords.push((origin.0 - n, origin.1 - i));
					}
				}
			}
			DataSymbol::Circle => {
				pixel_coords.push(origin);
				for n in 0..=thickness {
					// Ensure even radius of circle
					let r = if (radius + 1) & 1 == 1 {
						radius + 2 + n
					} else {
						radius + 1 + n
					};
					let scale_factor: u32 = 1000;
					for angle_deg in 0..(360 * scale_factor) {
						let y =
							(angle_deg as f32 / scale_factor as f32).to_radians().sin() * r as f32;
						let x =
							(angle_deg as f32 / scale_factor as f32).to_radians().cos() * r as f32;
						let delta_x = origin.0 as f32 + x;
						let delta_y = origin.1 as f32 + y;
						pixel_coords.push((delta_x as u32, delta_y as u32));
					}
				}
			}
			DataSymbol::Triangle => {
				pixel_coords.push(origin);
				for n in 0..=thickness {
					let float_n = n as f32;
					// To draw an equilateral triangle with a corner facing northwards we can
					// find the coords of each corner, find equations of a straight line to
					// join them up and iterate over the known `x` coord to place the pixels
					// along each line.
					//
					// Find the 3 corner coord from our origin:
					//
					// Incircle radius of an equilateral triangle is twice the area divided by the perimiter
					// area = (side_length * height)/ 2
					// height = side_length * (3.sqrt() / 2)
					// perimiter = 3 x side_length
					// ir = side_length / (2 * 3.sqrt())
					//
					// Note our side length is the radius argument
					// Ensure side_length is even otherwise the triangle will have an offset
					// Initial (radius + 1) ensures that if a user sets radius to zero something will still be drawn
					let side_length: f32 = if (radius + 1) & 1 == 1 {
						(radius + 2) as f32 + float_n
					} else {
						(radius + 1) as f32 + float_n
					};
					let height: f32 = side_length * (3.0_f32.sqrt() / 2.0);
					let incircle_radius: f32 = side_length / (2.0 * 3.0_f32.sqrt());

					// The 3 corners
					// The northwards facing corner is simply the origin plus the (height - incircle_radius)
					let top: (f32, f32) =
						(origin.0 as f32, origin.1 as f32 + height - incircle_radius);
					// The bottom left corner is simply the origin moved left by half the side length and moved down by incircle
					let left: (f32, f32) = (
						origin.0 as f32 - (side_length / 2.0),
						origin.1 as f32 - incircle_radius,
					);
					// The bottom right corner is simply the origin moved right by half the side length and moved down by incircle
					let right: (f32, f32) = (
						origin.0 as f32 + (side_length / 2.0),
						origin.1 as f32 - incircle_radius,
					);
					// Ensure corners are drawn
					pixel_coords.push((top.0 as u32, top.1 as u32));
					pixel_coords.push((left.0 as u32, left.1 as u32));
					pixel_coords.push((right.0 as u32, right.1 as u32));

					// Joining left and right is easy as they are vertically aligned
					let x_distance_between_left_right = (right.0 - left.0) as u32;
					for i in 0..x_distance_between_left_right {
						pixel_coords.push((left.0 as u32 + i, left.1 as u32));
					}

					// Finding the points to connect left to top we can create an equation of a striahgt line, y = mx + c
					let gradient_l_t = (top.1 - left.1) / (top.0 - left.0);
					let intercept_l_t = left.1 - (gradient_l_t * left.0);
					// Iterate over each X from left to top giving us the Y
					let left_to_top_delta_x = (top.0 - left.0) as u32;
					// We modify the actual range to iterate by a scale factor as we cannot iterate over a float
					// which would allow us to find a contiguous line of points. Without a modifier the f32 conversion
					// to u32 produces a dotted line
					let scale_factor = 10000;
					for i in 0..(left_to_top_delta_x * scale_factor) {
						let x = left.0 + (i as f32 / scale_factor as f32);
						let y = (gradient_l_t * x) + intercept_l_t;
						pixel_coords.push((x as u32, y as u32));
					}

					// Finding the points to connect top to right we can create an equation of a striahgt line, y = mx + c
					let gradient_t_r = (top.1 - right.1) / (top.0 - right.0);
					let intercept_t_r = right.1 - (gradient_t_r * right.0);
					// Iterate over each X from top to right giving us the Y
					let top_to_right_delta_x = (right.0 - top.0) as u32;
					// We modify the actual range to iterate by a scale factor as we cannot iterate over a float
					// which would allow us to find a contiguous line of points. Without a modifier the f32 conversion
					// to u32 produces a dotted line
					let scale_factor = 10000;
					for i in 0..(top_to_right_delta_x * scale_factor) {
						let x = top.0 + (i as f32 / scale_factor as f32);
						let y = (gradient_t_r * x) + intercept_t_r;
						pixel_coords.push((x as u32, y as u32));
					}
				}
			}
			DataSymbol::Square => {
				pixel_coords.push(origin);
				// Always ensure even to prevent shaoe being offset by 1 pixel
				let half_side_length = if (radius + 1) & 1 == 1 {
					radius + 2
				} else {
					radius + 1
				};
				for i in 0..half_side_length {
					for n in 0..=thickness {
						// top right horizontal
						pixel_coords.push((origin.0 + i + n, origin.1 - radius - n));
						// top left horizontal
						pixel_coords.push((origin.0 - i - n, origin.1 - radius - n));
						// bottom right horizontal
						pixel_coords.push((origin.0 + i + n, origin.1 + radius + n));
						// bottom left horizontal
						pixel_coords.push((origin.0 - i - n, origin.1 + radius + n));
						// left upper vertical
						pixel_coords.push((origin.0 - radius - n, origin.1 - i - n));
						// left bottom vertical
						pixel_coords.push((origin.0 - radius - n, origin.1 + i + n));
						// right upper vertical
						pixel_coords.push((origin.0 + radius + n, origin.1 - i - n));
						// right bottom vertical
						pixel_coords.push((origin.0 + radius + n, origin.1 + i + n));
					}
				}
			}
			DataSymbol::Point => {
				// it's just the original pixel
				pixel_coords.push(origin);
			}
		}
		return pixel_coords;
	}
}

/// Representation of a point to be drawn on a graph
#[derive(Debug, Deserialize, Copy, Clone)]
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
	/// The size of a drawn symbol in (1+ symbol_radius) pixels
	pub symbol_radius: u32,
	/// The thinkness of a drawn symbol in (1 + symbol_thickness) pixels
	pub symbol_thickness: u32,
}
impl DataPoint {
	pub fn draw_point(
		self,
		canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
		x_scale_factor: f32,
		y_scale_factor: f32,
		axes_origin: (u32, u32),
	) {
		trace!("Drawing point {:?}", self);
		let rgba = Colour::get_pixel_colour(self.colour);
		let x_pixel_corrected_pos = if self.x > 0.0 {
			axes_origin.0 + (self.x * x_scale_factor) as u32
		} else {
			axes_origin.0 - (-self.x * x_scale_factor) as u32
		};
		// let x_pixel_corrected_pos = axes_origin.0 + (self.x * x_scale_factor) as u32;
		// note pixel postions on a axes_origin are from top-left corner origin so additionally adjust y position based
		// on canvas height by minusing the offset to centre it at the axis origin and then minus scaled pixels
		let y_pixel_corrected_pos = if self.y > 0.0 {
			axes_origin.1 - (self.y * y_scale_factor) as u32
		} else {
			axes_origin.1 + (-self.y * y_scale_factor) as u32
		};
		trace!(
			"Plotting data point ({}, {}) with pixel position ({}, {})",
			self.x,
			self.y,
			x_pixel_corrected_pos,
			y_pixel_corrected_pos
		);
		// find the pixels that corrpespond to the symbol shape
		let pixels_in_shape = self.symbol.find_pixels(
			(x_pixel_corrected_pos, y_pixel_corrected_pos),
			self.symbol_thickness,
			self.symbol_radius,
		);
		for (px, py) in pixels_in_shape.iter() {
			match canvas.get_pixel_mut_checked(*px, *py) {
				Some(pixel) => *pixel = Rgba(rgba),
				None => warn!(
					"Cannot plot data point ({}, {}) with symbol pixel position ({}, {})",
					self.x, self.y, x_pixel_corrected_pos, y_pixel_corrected_pos
				),
			}
		}
		match self.ux {
			Some(value) => {
				trace!("Drawing x uncertainty with size {}", value);
				// furthest pixel to the right
				let upper_limit_pixel = axes_origin.0 + ((self.x + value) * x_scale_factor) as u32;
				// furthest pixel to the left
				let lower_limit_pixel = axes_origin.0 + ((self.x - value) * x_scale_factor) as u32;
				// draw line from left to right
				for px in lower_limit_pixel..=upper_limit_pixel {
					match canvas.get_pixel_mut_checked(px, y_pixel_corrected_pos) {
						Some(pixel) => *pixel = Rgba(rgba),
						None => warn!(
							"Cannot plot error bar point ({}, {}) with pixel position ({}, {})",
							self.x, self.y, px, y_pixel_corrected_pos
						),
					}
				}
				let error_bar_length = (upper_limit_pixel - lower_limit_pixel) / 4;
				// draws 'wings' either side of the limits
				for py in 0..=error_bar_length {
					// down
					match canvas
						.get_pixel_mut_checked(upper_limit_pixel, y_pixel_corrected_pos + py)
					{
						Some(pixel) => *pixel = Rgba(rgba),
						None => warn!(
							"Cannot plot error bar wing for point ({}, {}) with pixel position ({}, {})",
							self.x, self.y, upper_limit_pixel, y_pixel_corrected_pos + py
						),
					}
					match canvas
						.get_pixel_mut_checked(lower_limit_pixel, y_pixel_corrected_pos + py)
					{
						Some(pixel) => *pixel = Rgba(rgba),
						None => warn!(
							"Cannot plot error bar wing for point ({}, {}) with pixel position ({}, {})",
							self.x, self.y, lower_limit_pixel, y_pixel_corrected_pos - py
						),
					}
					// up
					match canvas
						.get_pixel_mut_checked(upper_limit_pixel, y_pixel_corrected_pos - py)
					{
						Some(pixel) => *pixel = Rgba(rgba),
						None => warn!(
							"Cannot plot error bar wing for point ({}, {}) with pixel position ({}, {})",
							self.x, self.y, upper_limit_pixel, y_pixel_corrected_pos + py
						),
					}
					match canvas
						.get_pixel_mut_checked(lower_limit_pixel, y_pixel_corrected_pos - py)
					{
						Some(pixel) => *pixel = Rgba(rgba),
						None => warn!(
							"Cannot plot error bar wing for point ({}, {}) with pixel position ({}, {})",
							self.x, self.y, lower_limit_pixel, y_pixel_corrected_pos - py
						),
					}
				}
			}
			None => {}
		}
		match self.uy {
			Some(value) => {
				trace!("Drawing y uncertainty with size {}", value);
				// furthest pixel above
				let upper_limit_pixel = axes_origin.1 - ((self.y - value) * y_scale_factor) as u32;
				// furthest pixel below
				let lower_limit_pixel = axes_origin.1 - ((self.y + value) * y_scale_factor) as u32;
				// draw line from above to below
				for py in lower_limit_pixel..=upper_limit_pixel {
					match canvas.get_pixel_mut_checked(x_pixel_corrected_pos, py) {
						Some(pixel) => *pixel = Rgba(rgba),
						None => warn!(
							"Cannot plot error bar point ({}, {}) with pixel position ({}, {})",
							self.x, self.y, x_pixel_corrected_pos, py
						),
					}
				}
				let error_bar_length = (upper_limit_pixel - lower_limit_pixel) / 4;
				// draws 'wings' either side of the limits
				for px in 0..=error_bar_length {
					// to the left
					match canvas
						.get_pixel_mut_checked(x_pixel_corrected_pos - px, upper_limit_pixel)
					{
						Some(pixel) => *pixel = Rgba(rgba),
						None => warn!(
							"Cannot plot error bar wing for point ({}, {}) with pixel position ({}, {})",
							self.x, self.y, x_pixel_corrected_pos - px, upper_limit_pixel
						),
					}
					match canvas
						.get_pixel_mut_checked(x_pixel_corrected_pos - px, lower_limit_pixel)
					{
						Some(pixel) => *pixel = Rgba(rgba),
						None => warn!(
							"Cannot plot error bar wing for point ({}, {}) with pixel position ({}, {})",
							self.x, self.y, x_pixel_corrected_pos - px, lower_limit_pixel
						),
					}
					// to the right
					match canvas
						.get_pixel_mut_checked(x_pixel_corrected_pos + px, upper_limit_pixel)
					{
						Some(pixel) => *pixel = Rgba(rgba),
						None => warn!(
							"Cannot plot error bar wing for point ({}, {}) with pixel position ({}, {})",
							self.x, self.y, x_pixel_corrected_pos + px, upper_limit_pixel
						),
					}
					match canvas
						.get_pixel_mut_checked(x_pixel_corrected_pos + px, lower_limit_pixel)
					{
						Some(pixel) => *pixel = Rgba(rgba),
						None => warn!(
							"Cannot plot error bar wing for point ({}, {}) with pixel position ({}, {})",
							self.x, self.y, x_pixel_corrected_pos + px, lower_limit_pixel
						),
					}
				}
			}
			None => {}
		}
	}
}
