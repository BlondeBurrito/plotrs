//!

use image::{ImageBuffer, Rgba};
use serde::Deserialize;
use tracing::trace;

use crate::colours::Colour;

use super::DataSymbol;

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
}
impl DataPoint {
	pub fn draw_point(self, canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
		trace!("Drawing point {:?}", self);
		let rgba = Colour::get_pixel_colour(self.colour);
	}
}
