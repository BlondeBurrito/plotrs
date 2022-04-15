//!

use image::{ImageBuffer, Rgba};
use rusttype::PositionedGlyph;
use tracing::debug;

use crate::{colours::*, get_system_font};

use super::glyphs::{create_glyphs, draw_glyphs};

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
	let width = {
        let min_x = glyphs
            .first()
            .map(|g| g.pixel_bounding_box().unwrap().min.x)
            .unwrap();
        let max_x = glyphs
            .last()
            .map(|g| g.pixel_bounding_box().unwrap().max.x)
            .unwrap();
        (max_x - min_x) as u32
    };
	let height = {
        let min_y = glyphs
            .first()
            .map(|g| g.pixel_bounding_box().unwrap().min.y)
            .unwrap();
        let max_y = glyphs
            .last()
            .map(|g| g.pixel_bounding_box().unwrap().max.y)
            .unwrap();
        (max_y - min_y) as u32
    };
	debug!("Title width: {}", width);
	debug!("Title height: {}", height);
	let horizontal_position = (canvas_width / 2) - (width as u32 / 2);
	debug!("Title horizontal offset: {}", horizontal_position);
	let vertical_postion = height as u32;
	debug!("Title vertical offset: {}", vertical_postion);
	return (horizontal_position, vertical_postion);
}
