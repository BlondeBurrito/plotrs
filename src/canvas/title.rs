//! Draws a title onto a canvas

use image::{ImageBuffer, Rgba};
use rusttype::PositionedGlyph;
use tracing::debug;

use crate::{
	canvas::{
		glyphs::{get_maximum_height_of_glyphs, get_width_of_glyphs},
		VHConsumedCanvasSpace, CANVAS_BORDER_PIXELS,
	},
	colours::*,
	get_system_font,
};

use super::glyphs::{create_glyphs, draw_glyphs};

/// Draws the title of the graph onto the canvas, returns the amount of vertical pixel space occupied
/// from the top of the canvas with an additional buffer of `CANVAS_BORDER_PIXELS`
pub fn build_title(
	canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
	title: &str,
	font_size: f32,
) -> VHConsumedCanvasSpace {
	let font = get_system_font();
	let glyphs: Vec<PositionedGlyph> = create_glyphs(font_size, title, &font);
	let height = get_maximum_height_of_glyphs(&glyphs);
	let width = get_width_of_glyphs(&glyphs);
	// position the title in the middle
	let position: (u32, u32) = (
		(canvas.dimensions().0 / 2) - (width as u32 / 2),
		CANVAS_BORDER_PIXELS,
	);
	debug!("Title position {:?}", position);
	draw_glyphs(canvas, BLACK, glyphs, position);
	VHConsumedCanvasSpace {
		v_space_from_top: position.1 + height + CANVAS_BORDER_PIXELS,
		h_space_from_left: 0,
		v_space_from_bottom: 0,
		h_space_from_right: 0,
	}
}
