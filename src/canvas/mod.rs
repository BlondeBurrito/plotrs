//! Methods for drawing onto a canvas, generating glyphs and saving images

use crate::colours::*;
use image::{ImageBuffer, Rgba, RgbaImage};
use regex::Regex;
use tracing::{error, info};

pub mod axes;
pub mod glyphs;
pub mod legend;
pub mod plot;
pub mod best_fit;
pub mod title;

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
