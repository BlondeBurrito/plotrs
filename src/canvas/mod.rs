//! Methods for drawing onto a canvas, generating glyphs and saving images

use crate::colours::*;
use image::{ImageBuffer, Rgba, RgbaImage};
use regex::Regex;
use tracing::{error, info};

pub mod axes;
pub mod best_fit;
pub mod glyphs;
pub mod legend;
pub mod plot;
pub mod quadrants;
pub mod title;

pub const CANVAS_BORDER_PIXELS: u32 = 10;
/// Describes the amount of horizontal and vertical canvas pixel space that has been consumed from graph elements such as legend, labels and title.
/// The final form of this data describes the size of the pixel area avaialble for drawing the axes
pub struct VHConsumedCanvasSpace {
	pub v_space_from_top: u32,
	pub h_space_from_right: u32,
	pub v_space_from_bottom: u32,
	pub h_space_from_left: u32,
}

impl VHConsumedCanvasSpace {
	pub fn new() -> VHConsumedCanvasSpace {
		VHConsumedCanvasSpace {
			v_space_from_top: CANVAS_BORDER_PIXELS,
			h_space_from_right: CANVAS_BORDER_PIXELS,
			v_space_from_bottom: CANVAS_BORDER_PIXELS,
			h_space_from_left: CANVAS_BORDER_PIXELS,
		}
	}
	/// Adds `VHConsumedCanvasSpace` to the calling `VHConsumedCanvasSpace`
	pub fn add(&mut self, increment: VHConsumedCanvasSpace) {
		self.v_space_from_top += increment.v_space_from_top;
		self.h_space_from_left += increment.h_space_from_left;
		self.v_space_from_bottom += increment.v_space_from_bottom;
		self.h_space_from_right += increment.h_space_from_right;
	}
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
