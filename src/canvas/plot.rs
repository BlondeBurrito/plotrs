//!

use image::{ImageBuffer, Rgba};
use serde::Deserialize;
use tracing::{debug, trace, warn};

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
		pixel_coords.push(origin);
		match self {
			DataSymbol::Cross => {
				let length: u32 = (radius + 1) * (thickness + 1);

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
			DataSymbol::Circle => todo!(),
			DataSymbol::Triangle => todo!(),
			DataSymbol::Square => todo!(),
			DataSymbol::Point => todo!(),
		}
		return pixel_coords;
	}
}

/// Representation of a point to be drawn on a graph
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
		let x_pixel_corrected_pos = axes_origin.0 + (self.x * x_scale_factor) as u32;
		// note pixel postions on a axes_origin are from top-left corner origin so additionally adjust y position based
		// on canvas height by minusing the offset to centre it at the axis origin and then minus scaled pixels
		let y_pixel_corrected_pos =
			axes_origin.1 - (self.y * y_scale_factor) as u32;
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
	}
}
