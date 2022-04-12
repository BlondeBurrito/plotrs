//! Constructs a scatter graph

use image::{ImageBuffer, Rgba};
use ron::de::from_reader;
use rusttype::PositionedGlyph;
use serde::Deserialize;
use std::fs::File;
use tracing::{debug, error, info};

use crate::{
	canvas::{create_glyphs, draw_base_canvas, draw_glyphs, save_image, CanvasElements},
	colours::*,
	get_system_font,
};
/// Specification of a scatter chart
#[derive(Debug, Deserialize)]
struct Scatter {
	title: String,
	canvas_pixel_size: (u32, u32),
	x_axis_label: String,
	y_axis_label: String,
	data_path: String,
}

/// Creates a canvas and draws the scatter graph over it
pub fn scatter_builder(path: &str, output: &str) {
	info!("Building scatter chart...");
	let scatter: Scatter = deserialise_config(path);
	let mut canvas = draw_base_canvas(scatter.canvas_pixel_size);
	// calcualte positions of elements on the graph
	let canvas_elements = CanvasElements::new(&scatter.canvas_pixel_size);
	//TODO:
	build_title(&mut canvas, &scatter);
	build_x_axis(&mut canvas, &scatter);
	// save the resulting image
	save_image(canvas, output, scatter.title);
}

/// Based on a path deserialise a `.ron` file into a data structure
fn deserialise_config(path: &str) -> Scatter {
	// attempt to open the .ron file
	let f = match File::open(path) {
		Ok(file) => file,
		Err(e) => {
			error!("Failed to open .ron file at {}, error: {:?}", path, e);
			std::process::exit(1)
		}
	};
	// attempt to deserialise the config data
	let scatter: Scatter = match from_reader(f) {
		Ok(x) => x,
		Err(e) => {
			error!("Failed to load config: {}", e);
			std::process::exit(1);
		}
	};
	debug!("Ron config {:?}", &scatter);
	return scatter;
}

/// Draws the title of the graph onto the canvas
fn build_title(canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, scatter: &Scatter) {
	let font = get_system_font();
	let title_font_size = (scatter.canvas_pixel_size.0 / scatter.title.len() as u32) as f32;
	debug!("Calculated title font size to be {}", title_font_size);
	let title_offset = (scatter.canvas_pixel_size.0 / 4, 5);
	debug!("Calculated title offset to be {:?}", title_offset);
	let title_glyphs: Vec<PositionedGlyph> =
	create_glyphs(title_font_size, scatter.title.as_str(), &font);
	draw_glyphs(canvas, BLACK, title_glyphs, title_offset);
}
/// Draws the x-axis label onto the canvas
fn build_x_axis(canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, scatter: &Scatter) {
	let font = get_system_font();
	let axis_font_size = (scatter.canvas_pixel_size.1 / scatter.x_axis_label.len() as u32) as f32;
	debug!("Calculated x-axis font size to be {}", axis_font_size);
	// position at the bottom in the centre
	let axis_offset = (scatter.canvas_pixel_size.0 / 4, scatter.canvas_pixel_size.1 /2);
	debug!("Calculated x-axis offset to be {:?}", axis_offset);
	let axis_glyphs: Vec<PositionedGlyph> =
		create_glyphs(axis_font_size, scatter.x_axis_label.as_str(), &font);
	draw_glyphs(canvas, BLACK, axis_glyphs, (axis_offset.0, axis_offset.1));
}
