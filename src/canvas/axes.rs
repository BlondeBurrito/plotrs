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
	let offset = get_y_axis_label_offset(&axis_glyphs, rotated_canvas.dimensions().0);
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
fn get_y_axis_label_offset(glyphs: &Vec<PositionedGlyph>, canvas_width: u32) -> (u32, u32) {
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
	debug!("Y-axis label width: {}", width);
	debug!("Y-axis label height: {}", height);
	let horizontal_position = (canvas_width / 2) - (width / 2);
	debug!("Y-axis horizontal offset: {}", horizontal_position);
	let vertical_postion = height as u32;
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
	let offset =
		get_x_axis_label_offset(&axis_glyphs, canvas.dimensions().0, canvas.dimensions().1);
	draw_glyphs(canvas, BLACK, axis_glyphs, offset);
	return offset.1;
}

/// Using glyph sizes calculate by how much the axis label should be offset from the origin
fn get_x_axis_label_offset(
	glyphs: &Vec<PositionedGlyph>,
	canvas_width: u32,
	canvas_height: u32,
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
	debug!("X-axis label width: {}", width);
	debug!("X-axis label height: {}", height);
	let horizontal_position = (canvas_width / 2) - (width / 2);
	debug!("X-axis horizontal offset: {}", horizontal_position);
	// scale by 2 so the bottom has some whitespace
	let vertical_postion = canvas_height - (height * 2);
	debug!("X-axis vertical offset: {}", vertical_postion);
	return (horizontal_position, vertical_postion);
}
/// Find the pixel pair which pinpoints the origin of the x-y axes.
///
/// X is small as it's drawn from the left hand border based on canvas origin in top right (0,0).
///
/// Y is large as it's the pixel most towrds the bottom left canvas corner, which is far away from the canvas (0, 0) in the top left corner
pub fn get_xy_axis_pixel_origin(x: u32, vertical_pixels_from_bottom: u32, canvas_size: (u32, u32)) -> (u32, u32) {
	// ensures at least 10% of the left and bottom canvas is free
	(x + (canvas_size.0 / 10), vertical_pixels_from_bottom - (canvas_size.1 / 10))
}
/// Find the pixel pair which pinpoints the maxmium length and height of the axes. Resolutions are
/// used to ensure that the length of each axis is a natural scale factor of the resolution. This
/// allows for accurately plotting data points
///
/// X is large as it's to the rightmost side of the canvas based on canvas origin in top right (0,0).
///
/// Y is small as it's near to the (0,0) canvas origin in the top left
pub fn get_xy_axis_pixel_max(
	axis_origin: (u32, u32),
	vertical_pixels_from_top: u32,
	legend_scale_factor: u32,
	canvas_size: (u32, u32),
	x_axis_resolution: u32,
	y_axis_resolution: u32,
) -> (u32, u32) {
	// ensures the whitespace to the left and right are the same when a legend is not specified
	let maximum_possible_x = canvas_size.0 - (axis_origin.0 * legend_scale_factor);
	// The true length of the axis must be a factor of the resolution so that axis scale markings
	// accurately line up with plotted points
	let mut x = maximum_possible_x.clone();
	'outer_x: for i in 0..maximum_possible_x {
		if get_x_axis_pixel_length(axis_origin.0, x - i) % x_axis_resolution == 0 {
			x = x - i;
			break 'outer_x;
		}
	}
	// ensures at least 5% of the top canvas is free
	let minimum_possible_y = vertical_pixels_from_top + (canvas_size.1 / 20);
	// The true length of the axis must be a factor of the resolution so that axis scale markings
	// accurately line up with plotted points
	let mut y = minimum_possible_y.clone();
	'outer_y: for i in 0..minimum_possible_y {
		if get_y_axis_pixel_length(y + i, axis_origin.1) % y_axis_resolution == 0 {
			y = y + i;
			break 'outer_y;
		}
	}
	return (x, y)
}
/// Get the pixel length of the x-axis
pub fn get_x_axis_pixel_length(min_pixel: u32, max_pixel: u32) -> u32 {
	max_pixel - min_pixel
}
/// Get the pixel length of the y-axis
pub fn get_y_axis_pixel_length(min_pixel: u32, max_pixel: u32) -> u32 {
	max_pixel - min_pixel
}

/// Within the acceptable pixel space for the axes draw them, note the top left corner of the canvas is the origin `(0, 0)` with bottom right `(canvas.dimensions().0, canvas.dimensions().1)`
pub fn draw_xy_axes(
	canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
	origin_pixel: (u32, u32),
	max_pixel: (u32, u32),
	x_axis_length: u32,
	y_axis_length: u32,
	x_data_min_max_limits: (u32, u32),
	y_data_min_max_limits: (u32, u32),
	font_size: f32,
	has_grid: bool,
	x_axis_resolution: u32,
	y_axis_resolution: u32,
) {
	// x-axis
	draw_x_axis(canvas, origin_pixel, x_axis_length);
	// x-axis data labels
	draw_x_axis_scale_markings(
		canvas,
		origin_pixel,
		max_pixel,
		x_axis_length,
		x_data_min_max_limits,
		font_size,
		has_grid,
		x_axis_resolution,
	);
	// y-axis
	draw_y_axis(canvas, origin_pixel, max_pixel, y_axis_length);
	// y-axis data labels
	draw_y_axis_scale_markings(
		canvas,
		origin_pixel,
		max_pixel,
		y_axis_length,
		y_data_min_max_limits,
		font_size,
		has_grid,
		y_axis_resolution,
	);
}
/// Draws the x-axis where the static `y` poistion is defined in the `max_pixel` tuple. This is a result
/// of the image origin being based in the top-left corner while the graph origin is in the bottom left
fn draw_x_axis(
	canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
	origin_pixel: (u32, u32),
	x_axis_length: u32,
) {
	debug!("Drawing x-axis");
	for px in origin_pixel.0..(x_axis_length + origin_pixel.0) {
		canvas.put_pixel(px, origin_pixel.1, Rgba(BLACK));
	}
}
/// Draws the scale markings along the x-axis
fn draw_x_axis_scale_markings(
	canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
	origin_pixel: (u32, u32),
	max_pixel: (u32, u32),
	x_axis_length: u32,
	x_data_min_max_limits: (u32, u32),
	font_size: f32,
	has_grid: bool,
	x_axis_resolution: u32,
) {
	let font = get_system_font();
	debug!("Drawing x-axis data labels");
	// Subdivide the x-axis length into a number of points we can draw labels at.
	// The number of pixels along the x-axis between each data label
	let x_subdivision_length = x_axis_length / x_axis_resolution;
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
		let label_length_scale = if i & 1 == 1 { 1 } else { 2 };
		for n in 0..(data_label_length * label_length_scale) {
			canvas.put_pixel(
				origin_pixel.0 + (i * x_subdivision_length),
				origin_pixel.1 + n,
				Rgba(BLACK),
			);
		}
		// Draw the data label text
		let text = (x_value_per_subdivision * i as f32).to_string();
		let glyphs = create_glyphs(font_size, &text, &font);
		let origin_x = origin_pixel.0 + (i * x_subdivision_length);
		let origin_y = origin_pixel.1 + (data_label_length * label_length_scale);
		let offset = get_x_axis_scale_label_offset(&glyphs, origin_x, origin_y);
		trace!("Drawing x-axis label {} at {:?}", text, offset);
		draw_glyphs(canvas, BLACK, glyphs, offset);
	}
}

/// Using glyph sizes calculate by how much the axis data label should be offset from an origin point
fn get_x_axis_scale_label_offset(
	glyphs: &Vec<PositionedGlyph>,
	origin_x: u32,
	origin_y: u32,
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
	trace!("X-axis data label width: {}", width);
	trace!("X-axis data label height: {}", height);
	//TODO: there must be a better way than using a scale factor of 2?
	let horizontal_position = origin_x - (width / 2);
	trace!(
		"X-axis data label horizontal offset: {}",
		horizontal_position
	);
	let vertical_postion = origin_y + height;
	trace!("X-axis data label vertical offset: {}", vertical_postion);
	return (horizontal_position, vertical_postion);
}

/// Draws the y-axis from the top left corner of the image down to the bottom left
fn draw_y_axis(
	canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
	origin_pixel: (u32, u32),
	max_pixel: (u32, u32),
	y_axis_length: u32,
) {
	debug!("Drawing y-axis");
	for py in max_pixel.1..(max_pixel.1 + y_axis_length) {
		canvas.put_pixel(origin_pixel.0, py, Rgba(BLACK));
	}
}

/// Draws the scale markings along the x-axis
fn draw_y_axis_scale_markings(
	canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
	origin_pixel: (u32, u32),
	max_pixel: (u32, u32),
	y_axis_length: u32,
	y_data_min_max_limits: (u32, u32),
	font_size: f32,
	has_grid: bool,
	y_axis_resolution: u32,
) {
	let font = get_system_font();
	debug!("Drawing y-axis data labels");
	// Subdivide the y-axis length into a number of points we can draw labels at.
	// The number of pixels along the x-axis between each data label
	let subdivision_length = y_axis_length / y_axis_resolution;
	// The pixel length of each data label
	let data_label_length = 5;
	// For writing a value for each data label we need to know the overall data size that corrpesonds to the axis
	let value_range = y_data_min_max_limits.1 as f32 - y_data_min_max_limits.0 as f32;
	// Find how much a suddivsion is in terms of data value
	let value_per_subdivision = value_range / y_axis_resolution as f32;
	// If required draw the x part of a background grid as grey vertical lines
	if has_grid {
		for i in 1..(y_axis_resolution + 1) {
			for px in origin_pixel.0..max_pixel.0 {
				canvas.put_pixel(px, origin_pixel.1 - (i * subdivision_length), Rgba(GREY));
			}
		}
	}
	// Draw a line of pixels down from the axis as each subdivisions
	for i in 0..(y_axis_resolution + 1) {
		// Draw each even section slightly longer
		let label_length_scale = if i & 1 == 1 { 1 } else { 2 };
		for n in 0..(data_label_length * label_length_scale) {
			canvas.put_pixel(
				origin_pixel.0 - n,
				origin_pixel.1 - (i * subdivision_length),
				Rgba(BLACK),
			);
		}
		// Draw the data label text
		let text = (value_per_subdivision * i as f32).to_string();
		let glyphs = create_glyphs(font_size, &text, &font);
		let origin_x = origin_pixel.0 - (data_label_length * label_length_scale) * 2;
		let origin_y = origin_pixel.1 - (i * subdivision_length);
		let offset = get_y_axis_scale_label_offset(&glyphs, origin_x, origin_y);
		trace!("Drawing y-axis label {} at {:?}", text, offset);
		draw_glyphs(canvas, BLACK, glyphs, offset);
	}
}

/// Using glyph sizes calculate by how much the axis data label should be offset from an origin point
fn get_y_axis_scale_label_offset(
	glyphs: &Vec<PositionedGlyph>,
	origin_x: u32,
	origin_y: u32,
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

	trace!("Y-axis data label width: {}", width);
	trace!("Y-axis data label height: {}", height);
	let horizontal_position = origin_x - width;
	trace!(
		"Y-axis data label horizontal offset: {}",
		horizontal_position
	);
	let vertical_postion = origin_y - (height / 2);
	trace!("Y-axis data label vertical offset: {}", vertical_postion);
	return (horizontal_position, vertical_postion);
}
