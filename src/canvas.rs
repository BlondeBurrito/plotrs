//! Methods for drawing onto a canvas, generating glyphs and saving images

use crate::{colours::*, get_system_font, scatter::DataPoint};
use image::{DynamicImage, GenericImage, ImageBuffer, Rgba, RgbaImage};
use regex::Regex;
use rusttype::{point, Font, PositionedGlyph, Scale};
use serde::Deserialize;
use tracing::{debug, error, info, trace, warn};

pub const TEXT_PIXEL_BUFFER: f32 = 20.0;

/// Font sizes for the different elements of a graph
pub struct FontSizes {
	pub title_font_size: f32,
	pub axis_font_size: f32,
	pub axis_unit_font_size: f32,
}

impl FontSizes {
	/// Based on the golden ratio and canvas width generate appropriate font sizes
	pub fn new(canvas_pixel_size: &(u32, u32)) -> FontSizes {
		// using the golden ratio and canvas width calculate the title font size
		let gr = (1.0 + 5.0_f32.sqrt()) / 2.0;
		// line height is the root of canvas wdith
		let line_height = (canvas_pixel_size.0 as f32).sqrt();
		// font size is line height divided by the ratio
		let title_font_size = line_height / gr;
		debug!("Calculated title font size to be {}", title_font_size);
		// axis font size is based on a reduction of title size
		let axis_font_size = title_font_size / 2.0;
		debug!("Calculated x-axis font size to be {}", axis_font_size);
		//TODO: is there a better wa of scaling axis unit size?
		let axis_unit_font_size = axis_font_size * 0.75;
		FontSizes {
			title_font_size: title_font_size,
			axis_font_size: axis_font_size,
			axis_unit_font_size: axis_unit_font_size,
		}
	}
}
/// The shape a plotted data point should take
#[derive(Debug, Deserialize, Copy, Clone)]
pub enum DataSymbol {
	Cross,
	Circle,
	Triangle,
	Square,
}

/// Create a blank canvas which can be mutated with content
pub fn draw_base_canvas(canvas_pixel_size: (u32, u32)) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
	// create a new image buffer based on `canvas_pixel_size`
	let mut imgbuf = RgbaImage::new(canvas_pixel_size.0, canvas_pixel_size.1);

	// set all pixels to white
	for pixel in imgbuf.pixels_mut() {
		*pixel = image::Rgba(WHITE);
	}
	return imgbuf;
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
	font.layout(
		text,
		scale,
		point(TEXT_PIXEL_BUFFER, TEXT_PIXEL_BUFFER + v_metrics.ascent),
	)
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

/// Save finished image to disk
pub fn save_image(imgbuf: ImageBuffer<Rgba<u8>, Vec<u8>>, output_path: &str, title: String) {
	let re = Regex::new(r"\s|\W").unwrap();
	let file_name = re.replace_all(title.as_str(), "_").to_lowercase();
	let output = output_path.to_owned() + "/" + file_name.as_str() + ".png";
	info!("Saving image to {}", output);
	match imgbuf.save(output) {
		Ok(_) => {
			info!("Image saved");
			std::process::exit(0);
		}
		Err(e) => {
			error!("Unable to save image: {:?}", e);
			std::process::exit(1);
		}
	}
}

/// Draws the y-axis label onto the canvas, returns how much horizontal space has been occupied
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

pub fn draw_point(point: DataPoint) {
	trace!("Drawing point {:?}", point);
	let rgba = Colour::get_pixel_colour(point.colour);
}
