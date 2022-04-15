//!

use image::{DynamicImage, GenericImage, ImageBuffer, Rgba};
use rusttype::{PositionedGlyph};
use tracing::{debug, error, trace};

use crate::{colours::*, get_system_font};

use super::glyphs::{create_glyphs, draw_glyphs, TEXT_PIXEL_BUFFER};

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
/// Find the pixel pair which poinpoints the origin of the x-y axes.
/// 
/// X is small as it's drawn from the left hand border based on canvas origin in top right (0,0).
/// 
/// Y is large as it's the pixel most towrds the bottom left canvas corner, which is far away from the canvas (0, 0) in the top left corner
pub fn get_xy_axis_pixel_origin(x:u32, y: u32) -> (u32, u32) {
	// TODO: there must be a better way than hardcoding scaling factors below
	(
		x + TEXT_PIXEL_BUFFER as u32 * 5,
		y - TEXT_PIXEL_BUFFER as u32 * 2,
	)
}
/// Find the pixel pair which pinpoints the maxmium length and height of the axes.
/// 
/// X is large as it's to the rightmost side of the canvas based on canvas origin in top right (0,0).
/// 
/// Y is small as it's near to the (0,0) canvas origin in the top left
pub fn get_xy_axis_pixel_max(canvas_width: u32, x_min: u32, y:u32, legend_scale_factor: u32) -> (u32, u32) {
	// TODO: there must be a better way than hardcoding scaling factors below
	(
		canvas_width - (x_min * legend_scale_factor),
		(y + TEXT_PIXEL_BUFFER as u32) * 2,
	)
}
/// Get the pixel length of the x-axis
pub fn get_x_axis_pixel_length(min_pixel: u32, max_pixel: u32) -> u32{
	max_pixel - min_pixel
}
/// Get the pixel length of the y-axis
pub fn get_y_axis_pixel_length(min_pixel: u32, max_pixel: u32) -> u32{
	max_pixel - min_pixel
}

/// Within the acceptable pixel space for the axes draw them, note the top left corner of the canvas is the origin `(0, 0)` with bottom right `(canvas.dimensions().0, canvas.dimensions().1)`
pub fn draw_xy_axes(
	canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
	origin_pixel: (u32, u32),
	max_pixel: (u32, u32),
	x_data_min_max_limits: (u32, u32),
	font_size: f32,
	has_grid: bool,
	x_axis_resolution: u32,
	y_axis_resolution: u32,
) {
	// x-axis
	debug!("Drawing x-axis");
	for px in origin_pixel.0..max_pixel.0 {
		canvas.put_pixel(px, origin_pixel.1, Rgba(BLACK));
	}
	let font = get_system_font();
	// x-axis data labels
	debug!("Drawing x-axis data labels");
	// Subdivide the x-axis length into a number of points we can draw labels at.
	// First we need the axis length in pixels
	let x_length = max_pixel.0 - origin_pixel.0;
	// The number of pixels along the x-axis between each data label
	let x_subdivision_length = x_length / x_axis_resolution;
	// The pixel length of each data label
	let data_label_length = 5;
	// For writing a value for each data label we need to know the overall data size that corrpesonds to the axis
	let x_value_range = x_data_min_max_limits.1 as f32 - x_data_min_max_limits.0 as f32;
	// Find how much a suddivsion is in terms of data value
	let x_value_per_subdivision = x_value_range / x_axis_resolution as f32;
	// If required draw the x part of a background grid as grey vertical lines
	if has_grid {
		for i in 1..(x_axis_resolution + 1) {
			for py in max_pixel.1..origin_pixel.1 {
				canvas.put_pixel(origin_pixel.0 + (i * x_subdivision_length), py, Rgba(GREY));
			}
		}
	}
	// Draw a line of pixels down from the axis as each subdivisions
	for i in 0..(x_axis_resolution + 1) {
		// Draw each even section slightly longer
		let label_length_scale = if i & 1 == 1 {
			1
		} else {
			2
		};
		for n in 0..(data_label_length * label_length_scale) {
			canvas.put_pixel(origin_pixel.0 + (i * x_subdivision_length), origin_pixel.1 + n, Rgba(BLACK));
		}
		// Draw the data label text
		let text = (x_value_per_subdivision * i as f32).to_string();
		let glyphs = create_glyphs(font_size, &text, &font);
		let origin_x = origin_pixel.0 + (i * x_subdivision_length);
		let origin_y = origin_pixel.1 + (data_label_length * label_length_scale);
		let offset = get_x_axis_data_label_offset(&glyphs, origin_x, origin_y, font_size, x_subdivision_length);
		debug!("Drawing {} at {:?}", text, offset);
		draw_glyphs(canvas, BLACK, glyphs, offset);
	}

	// y-axis
	// max_pixel.1 is the smaller value located top left, we work drawing "down" to meet the axis origin
	debug!("Drawing y-axis");
	for py in max_pixel.1..origin_pixel.1 {
		canvas.put_pixel(origin_pixel.0, py, Rgba(BLACK));
	}
}

/// Using glyph sizes calculate by how much the axis data label should be offset from an origin point
fn get_x_axis_data_label_offset(
	glyphs: &Vec<PositionedGlyph>,
	origin_x: u32,
	origin_y: u32,
	font_size: f32,
	subdivision_length: u32,
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
	// debug!("X-axis label pixel width: {}", text_width);
	// debug!("X-axis label max pixel height: {}", max_text_height);
	//TODO: there must be a better way than using a scale factor of 2?
	let horizontal_position = origin_x - (subdivision_length /(text_width as u32/2));
	// debug!("X-axis horizontal offset: {}", horizontal_position);
	let vertical_postion = origin_y - (max_text_height as u32);
	// debug!("X-axis vertical offset: {}", vertical_postion);
	return (horizontal_position, vertical_postion);
}
