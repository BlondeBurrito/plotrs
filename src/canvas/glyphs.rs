//!

use image::{ImageBuffer, Rgba};
use rusttype::{point, Font, PositionedGlyph, Scale};
use tracing::{debug, warn};

use crate::colours::*;

/// Font sizes for the different elements of a graph
pub struct FontSizes {
	pub title_font_size: f32,
	pub axis_font_size: f32,
	pub axis_unit_font_size: f32,
	pub legend_font_size: f32,
}

impl FontSizes {
	/// Based on the golden ratio and canvas width generate appropriate font sizes
	/// inspired from [pearsonified](https://pearsonified.com/golden-ratio-typography-intro/)
	pub fn new(canvas_pixel_size: &(u32, u32)) -> FontSizes {
		// using the golden ratio and canvas width calculate the title font size
		let gr = (1.0 + 5.0_f32.sqrt()) / 2.0;
		// line height is the root of canvas wdith
		let line_height = (canvas_pixel_size.0 as f32).sqrt();
		// font size is line height divided by the ratio
		// we include a scale based on testing of 1.5
		let title_font_size = 1.5 * line_height / gr;
		debug!("Calculated title font size to be {}", title_font_size);
		// axis font size is based on a reduction of title size
		let axis_font_size = title_font_size / 2.0;
		debug!("Calculated x-axis font size to be {}", axis_font_size);
		//TODO: is there a better wa of scaling axis unit size?
		let axis_unit_font_size = axis_font_size * 1.0;
		//TODO: is there a better way to calc legend font size?
		let legend_font_size = axis_font_size;
		FontSizes {
			title_font_size: title_font_size,
			axis_font_size: axis_font_size,
			axis_unit_font_size: axis_unit_font_size,
			legend_font_size: legend_font_size,
		}
	}
}

/// Creates a vector of gyphs running left to right
pub fn create_glyphs<'a>(
	font_size: f32,
	text: &'a str,
	font: &'a Font,
) -> Vec<PositionedGlyph<'a>> {
	let scale = Scale::uniform(font_size);
	let v_metrics = font.v_metrics(scale);

	// layout the glyphs in a line with TEXT_PIXEL_BUFFER pixels padding
	font.layout(text, scale, point(0.0, 0.0 + v_metrics.ascent))
		.collect()
}
/// Draws glyphs onto the canvas at a given position
pub fn draw_glyphs(
	canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
	colour: [u8; 4],
	glyphs: Vec<PositionedGlyph>,
	position: (u32, u32),
) {
	for glyph in glyphs {
		if let Some(bounding_box) = glyph.pixel_bounding_box() {
			// Draw the glyph into the image per-pixel by using the draw closure
			glyph.draw(|x, y, v| {
				let r = colour[0];
				let g = colour[1];
				let b = colour[2];
				let a = (v * 255.0) as u8;
				// select pixels with an offset
				let px = x + position.0 + bounding_box.min.x as u32;
				let py = y + position.1 + bounding_box.min.y as u32;
				match canvas.get_pixel_mut_checked(px, py) {
					Some(pixel) => *pixel = Rgba([r, g, b, a]),
					None => warn!("Cannot draw text outside of canvas at ({}, {}), shorter title/labels required or increase the canvas size", px, py),
				}
			});
		}
	}
	// drawing glyphs creates a pixel with an alpha channel of 0 surrounding its edges,
	// these indicate the empty space around a character and we fill them in with white background pixels
	for pixel in canvas.pixels_mut() {
		if pixel.0[3] == 0 {
			*pixel = Rgba(WHITE);
		}
	}
}
