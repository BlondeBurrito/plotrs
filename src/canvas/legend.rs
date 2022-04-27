//!

use image::{ImageBuffer, Rgba};
use tracing::{debug, trace, warn};

use crate::{
	canvas::{
		glyphs::{create_glyphs, draw_glyphs, get_maximum_height_of_glyphs},
		VHConsumedCanvasSpace,
	},
	colours::{Colour, BLACK},
	get_system_font,
};

use super::plot::DataSymbol;
#[derive(Debug)]
pub struct LegendField {
	pub symbol: DataSymbol,
	pub symbol_radius: u32,
	pub symbol_thickness: u32,
	pub colour: Colour,
	pub name: String,
}

pub fn build_legend(
	canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
	origin: (u32, u32),
	fields: Vec<LegendField>,
	font_size: f32,
) -> VHConsumedCanvasSpace {
	debug!("Building legend at {:?}...", origin);
	let font = get_system_font();
	let items = fields.len();
	// As symbols have different radii we want to find the maximum so we can space out the legend elements
	// with the same offset
	let max_radius: u32 = fields
		.iter()
		.max_by(|a, b| a.symbol_radius.cmp(&b.symbol_radius))
		.unwrap()
		.symbol_radius
		+ 2;
	for i in 0..items {
		let field = &fields[i];
		trace!("Legend field {:?}", field);
		let glyphs = create_glyphs(font_size, &field.name, &font);
		// height is used to write legend fields on new rows
		let height = get_maximum_height_of_glyphs(&glyphs);
		let symbol_position = (
			origin.0 + (max_radius + 1),
			origin.1 + (i as u32 * height * 2),
		);
		let pixels =
			field
				.symbol
				.find_pixels(symbol_position, field.symbol_thickness, field.symbol_radius);
		for (px, py) in pixels.iter() {
			match canvas.get_pixel_mut_checked(*px, *py) {
				Some(pixel) => *pixel = Rgba(Colour::get_pixel_colour(field.colour)),
				None => warn!(
					"Cannot plot legend point with symbol pixel position ({}, {})",
					px, py
				),
			}
		}
		let text_position = (
			origin.0 + (max_radius + 1) * 3,
			origin.1 + (i as u32 * height * 2),
		);
		draw_glyphs(canvas, BLACK, glyphs, text_position);
	}
	VHConsumedCanvasSpace {
		v_space_from_top: 0,
		h_space_from_right: canvas.dimensions().0 - origin.0,
		v_space_from_bottom: 0,
		h_space_from_left: 0,
	}
}
