//!

use crate::colours::*;
use image::{ImageBuffer, Rgba, RgbaImage};
use regex::Regex;
use rusttype::{point, Font, PositionedGlyph, Scale};
use tracing::{error, info, warn};

pub struct CanvasElements {
	title_font_size: f32,
}

impl CanvasElements {
	pub fn new(canvas_pixel_size: &(u32, u32)) -> CanvasElements {

		CanvasElements { title_font_size: 33.0 }
	}
}

/// Create a blank canvas which can be mutated with content
pub fn draw_base_canvas(canvas_pixel_size: (u32, u32)) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
	// create a new image buffer based on `canvas_pixel_size`
	let mut imgbuf = RgbaImage::new(canvas_pixel_size.0, canvas_pixel_size.1);

	// let e = DynamicImage::new_rgba8(00, 800).to_rgba8();

	// set all pixels to white
	for pixel in imgbuf.pixels_mut() {
		*pixel = image::Rgba(WHITE);
	}
	return imgbuf;
}

/// Creates a vector go gyphs
pub fn create_glyphs<'a>(
	font_size: f32,
	text: &'a str,
	font: &'a Font,
) -> Vec<PositionedGlyph<'a>> {
	let scale = Scale::uniform(font_size);
	let v_metrics = font.v_metrics(scale);

	// layout the glyphs in a line with 20 pixels padding
	font.layout(text, scale, point(20.0, 20.0 + v_metrics.ascent))
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
					None => warn!("Cannot draw text outside of canvas at ({}, {}), shorter title/labels required", px, py),
				}
			});
		}
	}
	// drawing glyphs creates pixel with an alpha channel of 0
	// these indicate the empty space around a character and we fill them in with white background pixels
	for pixel in canvas.pixels_mut() {
		if pixel.0[3] < 1 {
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
			info!("Output saved");
			std::process::exit(0);
		}
		Err(e) => {
			error!("Unable to save output: {:?}", e);
			std::process::exit(1);
		}
	}
}
