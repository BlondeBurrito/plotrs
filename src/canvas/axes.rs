//!

use image::{DynamicImage, GenericImage, ImageBuffer, Rgba};
use rusttype::PositionedGlyph;
use tracing::{debug, error, trace};

use crate::{colours::*, get_system_font};

use super::glyphs::{create_glyphs, draw_glyphs};

/// Draws the y-axis label onto the canvas, returns how much horizontal space has been occupied from the left-hand side of the canvas edge
pub fn build_y_axis_label(
	canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
	label: String,
	font_size: f32,
) -> u32 {
	let font = get_system_font();
	let axis_glyphs: Vec<PositionedGlyph> = create_glyphs(font_size, label.as_str(), &font);
	// as the glphs are drawn horizontally we draw them onto a new canvas where its width matches the main canvas' height
	// we can then rotate this new canvas and copy it onto the main canvas so that the y-axis label appears vertical and aligned to the left
	let mut rotated_canvas =
		DynamicImage::new_rgba8(canvas.dimensions().1, canvas.dimensions().0).to_rgba8();
	let offset = get_y_axis_label_offset(&axis_glyphs, rotated_canvas.dimensions().0, font_size);
	draw_glyphs(&mut rotated_canvas, BLACK, axis_glyphs, offset);
	// rotate the canvas so its dimensions are aligned to the main canvas
	let aligned_canvas = DynamicImage::ImageRgba8(rotated_canvas).rotate270();
	// copy the canvas containing the text onto the main canvas
	match canvas.copy_from(&aligned_canvas, 0, 0) {
		Ok(_) => (),
		Err(e) => {
			error!("Unable to draw y-axis label: {}", e);
			std::process::exit(1);
		}
	}
	// return offset height as the rotated width offset
	return offset.1;
}
/// Using glyph sizes calculate by how much the axis label should be offset from the origin
fn get_y_axis_label_offset(
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
	debug!("Y-axis label pixel width: {}", text_width);
	debug!("Y-axis label max pixel height: {}", max_text_height);
	let horizontal_position = (canvas_width / 2) - (text_width as u32);
	debug!("Y-axis horizontal offset: {}", horizontal_position);
	let vertical_postion = max_text_height as u32 * 2;
	debug!("Y-axis vertical offset: {}", vertical_postion);
	return (horizontal_position, vertical_postion);
}

/// Draws the x-axis label onto the canvas, returns the amount of vertical pixel space occupied from the bottom border
pub fn build_x_axis_label(
	canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
	label: String,
	font_size: f32,
) -> u32 {
	let font = get_system_font();
	let axis_glyphs: Vec<PositionedGlyph> = create_glyphs(font_size, label.as_str(), &font);
	let offset = get_x_axis_label_offset(
		&axis_glyphs,
		canvas.dimensions().0,
		canvas.dimensions().1,
		font_size,
	);
	draw_glyphs(canvas, BLACK, axis_glyphs, offset);
	return offset.1;
}

/// Using glyph sizes calculate by how much the axis label should be offset from the origin
fn get_x_axis_label_offset(
	glyphs: &Vec<PositionedGlyph>,
	canvas_width: u32,
	canvas_height: u32,
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
	debug!("X-axis label pixel width: {}", text_width);
	debug!("X-axis label max pixel height: {}", max_text_height);
	let horizontal_position = (canvas_width / 2) - (text_width as u32);
	debug!("X-axis horizontal offset: {}", horizontal_position);
	//TODO: there must be a better way than using a scale factor of 6?
	let vertical_postion = canvas_height - (max_text_height as u32 * 6);
	debug!("X-axis vertical offset: {}", vertical_postion);
	return (horizontal_position, vertical_postion);
}

/// Within the acceptable pixel space for the axes draw them, note the top left corner of the canvas is the origin `(0, 0)` with bottom right `(canvas.dimensions().0, canvas.dimensions().1)`
pub fn draw_xy_axes(
	canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
	origin_pixel: (u32, u32),
	max_pixel: (u32, u32),
) {
	// x-axis
	debug!("Drawing x-axis");
	for px in origin_pixel.0..max_pixel.0 {
		canvas.put_pixel(px, origin_pixel.1, Rgba(BLACK));
	}
	// y-axis
	// max_pixel.1 is the smaller value located top left, we work drawing "down" to meet the axis origin
	debug!("Drawing y-axis");
	for py in max_pixel.1..origin_pixel.1 {
		canvas.put_pixel(origin_pixel.0, py, Rgba(BLACK));
	}
}
