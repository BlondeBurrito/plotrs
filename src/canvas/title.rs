//!

use image::{ImageBuffer, Rgba};
use rusttype::PositionedGlyph;
use tracing::debug;

use crate::{get_system_font, colours::*};

use super::glyphs::{draw_glyphs, create_glyphs};

/// Draws the title of the graph onto the canvas, returns the amount of vertical pixel space occupied from the top border
pub fn build_title(
	canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
	title: &String,
	font_size: f32,
) -> u32 {
	let font = get_system_font();
	let title_glyphs: Vec<PositionedGlyph> = create_glyphs(font_size, title.as_str(), &font);
	let offset = get_title_offset(&title_glyphs, canvas.dimensions().0, font_size);
	draw_glyphs(canvas, BLACK, title_glyphs, offset);
	return offset.1;
}

/// Using glyph sizes calculate by how much the axis label should be offset from the origin
fn get_title_offset(
	glyphs: &Vec<PositionedGlyph>,
	canvas_width: u32,
	font_size: f32,
) -> (u32, u32) {
	let mut text_width = 0;
	let mut max_text_height = 0;
	for g in glyphs {
		match g.pixel_bounding_box() {
			Some(x) => {
				text_width += x.width();
				if x.height() > max_text_height {
					max_text_height = x.height()
				}
			}
			None => {
				// None indicates whitespace, assume whitespace width is same as font size
				text_width += font_size as i32;
			}
		};
	}
	debug!("Title pixel width: {}", text_width);
	debug!("Title max pixel height: {}", max_text_height);
	let horizontal_position = (canvas_width / 2) - (text_width as u32 / 2);
	debug!("Title horizontal offset: {}", horizontal_position);
	let vertical_postion = max_text_height as u32 * 2;
	debug!("Title vertical offset: {}", vertical_postion);
	return (horizontal_position, vertical_postion);
}