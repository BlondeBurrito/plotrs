//!

use image::{ImageBuffer, Rgba};
use rusttype::PositionedGlyph;
use tracing::{debug, trace};

use crate::{
	canvas::{
		glyphs::{get_maximum_height_of_glyphs, get_width_of_glyphs},
		VHConsumedCanvasSpace, CANVAS_BORDER_PIXELS,
	},
	colours::*,
	get_system_font,
};

use super::{
	glyphs::{create_glyphs, draw_glyphs},
	quadrants::Quadrants,
};

/// Draws the y-axis label onto the canvas, returns how much new vertical-horizontal space has been consumed on the canvas
pub fn build_y_axis_label(
	canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
	label: String,
	font_size: f32,
	quadrants: &Quadrants,
	vertical_pixels_from_top: u32,
	horizontal_pixels_from_right: u32,
	vertical_pixels_from_bottom: u32,
	horizontal_pixels_from_left: u32,
) -> VHConsumedCanvasSpace {
	let font = get_system_font();
	let glyphs: Vec<PositionedGlyph> = create_glyphs(font_size, label.as_str(), &font);
	let width = get_width_of_glyphs(&glyphs);
	let height = get_maximum_height_of_glyphs(&glyphs);
	match quadrants {
		Quadrants::RightPair => {
			debug!("Placing y-axis label in top left corner");
			let position: (u32, u32) = (CANVAS_BORDER_PIXELS, vertical_pixels_from_top);
			draw_glyphs(canvas, BLACK, glyphs, position);
			// return value of vertical space used is based on glyph height with a border
			// horizontal space ensures room for drawing scale markings/glyphs
			VHConsumedCanvasSpace {
				v_space_from_top: height + CANVAS_BORDER_PIXELS,
				h_space_from_left: width + CANVAS_BORDER_PIXELS,
				v_space_from_bottom: 0,
				h_space_from_right: 0,
			}
		},
		Quadrants::LeftPair => {
			debug!("Placing y-axis label in top right corner");
			let position: (u32, u32) = (canvas.dimensions().0 - width - CANVAS_BORDER_PIXELS - horizontal_pixels_from_right, vertical_pixels_from_top);
			draw_glyphs(canvas, BLACK, glyphs, position);
			// return value of vertical space used is based on glyph height with a border
			// horizontal space ensures room for drawing scale markings/glyphs
			VHConsumedCanvasSpace {
				v_space_from_top: height + CANVAS_BORDER_PIXELS,
				h_space_from_left: 0,
				v_space_from_bottom: 0,
				h_space_from_right: width + CANVAS_BORDER_PIXELS,
			}
		},
		Quadrants::TopPair => {
			debug!("Placing y-axis label in middle top area");
			let position: (u32, u32) = (
				(canvas.dimensions().0 / 2) - (width / 2),
				vertical_pixels_from_top,
			);
			draw_glyphs(canvas, BLACK, glyphs, position);
			// return value of vertical space used is based on glyph height with a border
			VHConsumedCanvasSpace {
				v_space_from_top: height + CANVAS_BORDER_PIXELS,
				h_space_from_left: 0,
				v_space_from_bottom: CANVAS_BORDER_PIXELS,
				h_space_from_right: 0,
			}
		},
		Quadrants::BottomPair => {
			debug!("Placing y-axis label in middle bottom area");
			let position: (u32, u32) = (
				(canvas.dimensions().0 / 2) - (width / 2),
				canvas.dimensions().1 - vertical_pixels_from_bottom - height,
			);
			draw_glyphs(canvas, BLACK, glyphs, position);
			// return value of vertical space used is based on glyph height with a border
			VHConsumedCanvasSpace {
				v_space_from_top: 0,
				h_space_from_left: 0,
				v_space_from_bottom: height + CANVAS_BORDER_PIXELS,
				h_space_from_right: 0,
			}
		},
		Quadrants::AllQuadrants => {
			debug!("Placing y-axis label in middle top area");
			let position: (u32, u32) = (
				(canvas.dimensions().0 / 2) - (width / 2),
				vertical_pixels_from_top,
			);
			draw_glyphs(canvas, BLACK, glyphs, position);
			// return value of vertical space used is based on glyph height with a border
			VHConsumedCanvasSpace {
				v_space_from_top: height + CANVAS_BORDER_PIXELS,
				h_space_from_left: 0,
				v_space_from_bottom: CANVAS_BORDER_PIXELS,
				h_space_from_right: 0,
			}
		}
		Quadrants::TopRight => {
			debug!("Placing y-axis label in top left corner");
			let position: (u32, u32) = (CANVAS_BORDER_PIXELS, vertical_pixels_from_top);
			draw_glyphs(canvas, BLACK, glyphs, position);
			// return value of vertical space used is based on glyph height with a border
			// horizontal space ensures room for drawing scale markings/glyphs
			VHConsumedCanvasSpace {
				v_space_from_top: height + CANVAS_BORDER_PIXELS,
				h_space_from_left: width + CANVAS_BORDER_PIXELS,
				v_space_from_bottom: 0,
				h_space_from_right: 0,
			}
		}
		Quadrants::TopLeft => {
			debug!("Placing y-axis label in top right corner");
			let position: (u32, u32) = (canvas.dimensions().0 - width - CANVAS_BORDER_PIXELS - horizontal_pixels_from_right, vertical_pixels_from_top);
			draw_glyphs(canvas, BLACK, glyphs, position);
			// return value of vertical space used is based on glyph height with a border
			// horizontal space ensures room for drawing scale markings/glyphs
			VHConsumedCanvasSpace {
				v_space_from_top: height + CANVAS_BORDER_PIXELS,
				h_space_from_left: 0,
				v_space_from_bottom: 0,
				h_space_from_right: width + CANVAS_BORDER_PIXELS,
			}
		},
		Quadrants::BottomRight => {
			debug!("Placing y-axis label in bottom left corner");
			let position: (u32, u32) = (horizontal_pixels_from_left + width + CANVAS_BORDER_PIXELS, canvas.dimensions().1 - vertical_pixels_from_bottom - height);
			draw_glyphs(canvas, BLACK, glyphs, position);
			VHConsumedCanvasSpace {
				v_space_from_top: 0,
				h_space_from_left: width + CANVAS_BORDER_PIXELS,
				v_space_from_bottom: height + CANVAS_BORDER_PIXELS,
				h_space_from_right: 0,
			}
		},
		Quadrants::BottomLeft => {
			debug!("Placing y-axis label in bottom right corner");
			let position: (u32, u32) = (canvas.dimensions().0 - horizontal_pixels_from_right - width, canvas.dimensions().1 - vertical_pixels_from_bottom - height);
			draw_glyphs(canvas, BLACK, glyphs, position);
			VHConsumedCanvasSpace {
				v_space_from_top: 0,
				h_space_from_left: 0,
				v_space_from_bottom: height + CANVAS_BORDER_PIXELS,
				h_space_from_right: width + CANVAS_BORDER_PIXELS,
			}
		},
	}
}

/// Draws the x-axis label onto the canvas, returns how much new vertical-horizontal space has been consumed on the canvas
pub fn build_x_axis_label(
	canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
	label: String,
	font_size: f32,
	quadrants: &Quadrants,
	vertical_pixels_from_top: u32,
	horizontal_pixels_from_right: u32,
	vertical_pixels_from_bottom: u32,
	horizontal_pixels_from_left: u32,
) -> VHConsumedCanvasSpace {
	let font = get_system_font();
	let glyphs: Vec<PositionedGlyph> = create_glyphs(font_size, label.as_str(), &font);
	let width = get_width_of_glyphs(&glyphs);
	let height = get_maximum_height_of_glyphs(&glyphs);
	match quadrants {
		Quadrants::RightPair => {
			debug!("Placing x-axis label in centre right");
			let position: (u32, u32) = (
				canvas.dimensions().0 - CANVAS_BORDER_PIXELS - (width),
				(canvas.dimensions().1 - vertical_pixels_from_top - vertical_pixels_from_bottom)
					/ 2 + vertical_pixels_from_top
					+ height,
			);
			draw_glyphs(canvas, BLACK, glyphs, position);
			VHConsumedCanvasSpace {
				v_space_from_top: 0,
				h_space_from_left: width + CANVAS_BORDER_PIXELS,
				v_space_from_bottom: 0,
				h_space_from_right: width + CANVAS_BORDER_PIXELS,
			}
		},
		Quadrants::LeftPair => {
			debug!("Placing x-axis label in centre left");
			let position: (u32, u32) = (
				horizontal_pixels_from_left + CANVAS_BORDER_PIXELS + (width),
				(canvas.dimensions().1 - vertical_pixels_from_top - vertical_pixels_from_bottom)
					/ 2 + vertical_pixels_from_top
					+ height,
			);
			draw_glyphs(canvas, BLACK, glyphs, position);
			VHConsumedCanvasSpace {
				v_space_from_top: 0,
				h_space_from_left: width + CANVAS_BORDER_PIXELS,
				v_space_from_bottom: 0,
				h_space_from_right: width + CANVAS_BORDER_PIXELS,
			}
		},
		Quadrants::TopPair => {
			debug!("Placing x-axis label in bottom right corner");
			let position: (u32, u32) = (
				canvas.dimensions().0 - CANVAS_BORDER_PIXELS - (width),
				canvas.dimensions().1 - CANVAS_BORDER_PIXELS - height,
			);
			draw_glyphs(canvas, BLACK, glyphs, position);
			VHConsumedCanvasSpace {
				v_space_from_top: 0,
				h_space_from_left: 0,
				v_space_from_bottom: CANVAS_BORDER_PIXELS + height + CANVAS_BORDER_PIXELS,
				h_space_from_right: width + CANVAS_BORDER_PIXELS,
			}
		},
		Quadrants::BottomPair => {
			debug!("Placing x-axis label in top right corner");
			let position: (u32, u32) = (
				canvas.dimensions().0 - horizontal_pixels_from_right - width,
				vertical_pixels_from_top + CANVAS_BORDER_PIXELS + height,
			);
			draw_glyphs(canvas, BLACK, glyphs, position);
			VHConsumedCanvasSpace {
				v_space_from_top: height + CANVAS_BORDER_PIXELS,
				h_space_from_left: 0,
				v_space_from_bottom: 0,
				h_space_from_right: width + CANVAS_BORDER_PIXELS,
			}
		},
		Quadrants::AllQuadrants => {
			debug!("Placing x-axis label in centre right");
			let position: (u32, u32) = (
				canvas.dimensions().0 - CANVAS_BORDER_PIXELS - (width),
				(canvas.dimensions().1 - vertical_pixels_from_top - vertical_pixels_from_bottom)
					/ 2 + vertical_pixels_from_top
					+ height,
			);
			draw_glyphs(canvas, BLACK, glyphs, position);
			VHConsumedCanvasSpace {
				v_space_from_top: 0,
				h_space_from_left: width + CANVAS_BORDER_PIXELS,
				v_space_from_bottom: 0,
				h_space_from_right: width + CANVAS_BORDER_PIXELS,
			}
		}
		Quadrants::TopRight => {
			debug!("Placing x-axis label in bottom right corner");
			let position: (u32, u32) = (
				canvas.dimensions().0 - CANVAS_BORDER_PIXELS - (width),
				canvas.dimensions().1 - CANVAS_BORDER_PIXELS - height,
			);
			draw_glyphs(canvas, BLACK, glyphs, position);
			VHConsumedCanvasSpace {
				v_space_from_top: 0,
				h_space_from_left: 0,
				v_space_from_bottom: CANVAS_BORDER_PIXELS + height + CANVAS_BORDER_PIXELS,
				h_space_from_right: width + CANVAS_BORDER_PIXELS,
			}
		}
		Quadrants::TopLeft => {
			debug!("Placing x-axis label in bottom left corner");
			let position: (u32, u32) = (
				horizontal_pixels_from_left + (width/2),
				canvas.dimensions().1 - CANVAS_BORDER_PIXELS - height,
			);
			draw_glyphs(canvas, BLACK, glyphs, position);
			VHConsumedCanvasSpace {
				v_space_from_top: 0,
				h_space_from_left: width + CANVAS_BORDER_PIXELS,
				v_space_from_bottom: CANVAS_BORDER_PIXELS + (height * 2) + CANVAS_BORDER_PIXELS,
				h_space_from_right: 0,
			}
		},
		Quadrants::BottomRight => {
			debug!("Placing x-axis label in top left corner");
			let position: (u32, u32) = (
				canvas.dimensions().0 - horizontal_pixels_from_right - width,
				vertical_pixels_from_top + CANVAS_BORDER_PIXELS + height,
			);
			draw_glyphs(canvas, BLACK, glyphs, position);
			VHConsumedCanvasSpace {
				v_space_from_top: height + CANVAS_BORDER_PIXELS,
				h_space_from_left: 0,
				v_space_from_bottom: 0,
				h_space_from_right: width + CANVAS_BORDER_PIXELS,
			}
		},
		Quadrants::BottomLeft => {
			debug!("Placing x-axis label in top left corner");
			let position: (u32, u32) = (
				horizontal_pixels_from_left + width,
				vertical_pixels_from_top + CANVAS_BORDER_PIXELS + height,
			);
			draw_glyphs(canvas, BLACK, glyphs, position);
			VHConsumedCanvasSpace {
				v_space_from_top: height + CANVAS_BORDER_PIXELS,
				h_space_from_left: width + CANVAS_BORDER_PIXELS,
				v_space_from_bottom: 0,
				h_space_from_right: 0,
			}
		},
	}
}
/// Find the pixel pair which pinpoints the maxmium length and height of the axes. Resolutions are
/// used to ensure that the length of each axis is a natural scale factor of the resolution. This
/// allows for accurately plotting data points
pub fn get_xy_axis_pixel_min_max(
	quadrants: &Quadrants,
	vertical_pixels_from_top: u32,
	horizontal_pixels_from_right: u32,
	vertical_pixels_from_bottom: u32,
	horizontal_pixels_from_left: u32,
	legend_scale_factor: u32,
	canvas_size: (u32, u32),
	x_axis_resolution: u32,
	y_axis_resolution: u32,
) -> ((u32, u32), (u32, u32)) {
	match quadrants {
		Quadrants::RightPair => {
			// ensures the whitespace to the left and right are the same when a legend is not specified
			let minimum_possible_x = horizontal_pixels_from_left;
			let maximum_possible_x =
				canvas_size.0 - (horizontal_pixels_from_right * legend_scale_factor);
			// The true length of the axis must be a factor of the resolution so that axis scale markings
			// accurately line up with plotted points
			let mut x = maximum_possible_x.clone();
			'outer_x_rp: for i in 0..maximum_possible_x {
				if get_x_axis_pixel_length(minimum_possible_x, x - i) % x_axis_resolution == 0 {
					x = x - i;
					break 'outer_x_rp;
				}
			}
			// // ensures at least 5% of the top canvas is free
			let minimum_possible_y = vertical_pixels_from_top; // + (canvas_size.1 / 20);
			let maximum_possible_y = canvas_size.1 - vertical_pixels_from_bottom;
			// The true length of the axis must be a factor of the resolution so that axis scale markings
			// accurately line up with plotted points
			let mut y1 = minimum_possible_y.clone();
			let y0 = maximum_possible_y.clone();
			'outer_y_rp: for i in 0..minimum_possible_y {
				// axis extends into negative space so ensure resolution fitting matches half overall length
				if get_y_axis_pixel_length(y1 + i, maximum_possible_y)/2 % y_axis_resolution == 0 && get_y_axis_pixel_length(y1 + i, maximum_possible_y)/2 % 2 == 0 {
					y1 = y1 + i;
					break 'outer_y_rp;
				}
			}
			return ((minimum_possible_x, y0), (x, y1));
		},
		Quadrants::LeftPair => {
			// ensures the whitespace to the left and right are the same when a legend is not specified
			let minimum_possible_x = horizontal_pixels_from_left;
			let maximum_possible_x = canvas_size.0 - horizontal_pixels_from_right;
			// The true length of the axis must be a factor of the resolution so that axis scale markings
			// accurately line up with plotted points
			let mut x = minimum_possible_x.clone();
			'outer_x_lp: for i in 0..maximum_possible_x {
				if get_x_axis_pixel_length(x + i, minimum_possible_x) % x_axis_resolution == 0 {
					x = x + i;
					break 'outer_x_lp;
				}
			}
			// // ensures at least 5% of the top canvas is free
			let minimum_possible_y = vertical_pixels_from_top; // + (canvas_size.1 / 20);
			let maximum_possible_y = canvas_size.1 - vertical_pixels_from_bottom;
			// The true length of the axis must be a factor of the resolution so that axis scale markings
			// accurately line up with plotted points
			let mut y1 = minimum_possible_y.clone();
			let y0 = maximum_possible_y.clone();
			'outer_y_lp: for i in 0..minimum_possible_y {
				// axis extends into negative space so ensure resolution fitting matches half overall length
				if get_y_axis_pixel_length(y1 + i, maximum_possible_y)/2 % y_axis_resolution == 0 && get_y_axis_pixel_length(y1 + i, maximum_possible_y)/2 % 2 == 0 {
					y1 = y1 + i;
					break 'outer_y_lp;
				}
			}
			return ((x, y0), (maximum_possible_x, y1));
		},
		Quadrants::TopPair => {
			// ensures the whitespace to the left and right are the same when a legend is not specified
			let minimum_possible_x = horizontal_pixels_from_left;
			let maximum_possible_x =
				canvas_size.0 - (horizontal_pixels_from_right * legend_scale_factor);
			// The true length of the axis must be a factor of the resolution so that axis scale markings
			// accurately line up with plotted points
			let mut x0 = minimum_possible_x.clone();
			let x1 = maximum_possible_x.clone();
			'outer_x_tp: for i in 0..maximum_possible_x {
				// axis extends into negative space so ensure resolution fitting matches half overall length
				if get_x_axis_pixel_length(x0 + i, maximum_possible_x)/2 % x_axis_resolution == 0 {
					x0 = x0 + i;
					break 'outer_x_tp;
				}
			}
			// // ensures at least 5% of the top canvas is free
			let minimum_possible_y = vertical_pixels_from_top; // + (canvas_size.1 / 20);
			let maximum_possible_y = canvas_size.1 - vertical_pixels_from_bottom;
			// The true length of the axis must be a factor of the resolution so that axis scale markings
			// accurately line up with plotted points
			let mut y = minimum_possible_y.clone();
			'outer_y_tp: for i in 0..minimum_possible_y {
				if get_y_axis_pixel_length(y + i, maximum_possible_y) % y_axis_resolution == 0 {
					y = y + i;
					break 'outer_y_tp;
				}
			}
			return ((x0, maximum_possible_y), (x1, y));
		},
		Quadrants::BottomPair => {
			// ensures the whitespace to the left and right are the same when a legend is not specified
			let minimum_possible_x = horizontal_pixels_from_left;
			let maximum_possible_x =
				canvas_size.0 - (horizontal_pixels_from_right * legend_scale_factor);
			// The true length of the axis must be a factor of the resolution so that axis scale markings
			// accurately line up with plotted points
			let mut x0 = minimum_possible_x.clone();
			let x1 = maximum_possible_x.clone();
			'outer_x_bp: for i in 0..maximum_possible_x {
				// axis extends into negative space so ensure resolution fitting matches half overall length
				if get_x_axis_pixel_length(x0 + i, maximum_possible_x)/2 % x_axis_resolution == 0 {
					x0 = x0 + i;
					break 'outer_x_bp;
				}
			}
			// // ensures at least 5% of the top canvas is free
			let minimum_possible_y = vertical_pixels_from_top; // + (canvas_size.1 / 20);
			let maximum_possible_y = canvas_size.1 - vertical_pixels_from_bottom;
			// The true length of the axis must be a factor of the resolution so that axis scale markings
			// accurately line up with plotted points
			let mut y = minimum_possible_y.clone();
			'outer_y_bp: for i in 0..maximum_possible_y {
				if get_y_axis_pixel_length(y + i, maximum_possible_y) % y_axis_resolution == 0 {
					y = y + i;
					break 'outer_y_bp;
				}
			}

			return ((x0, maximum_possible_y), (x1, y));
		},
		Quadrants::AllQuadrants => {
			// ensures the whitespace to the left and right are the same when a legend is not specified
			let minimum_possible_x = horizontal_pixels_from_left;
			let maximum_possible_x =
				canvas_size.0 - (horizontal_pixels_from_right * legend_scale_factor);
			// The true length of the axis must be a factor of the resolution so that axis scale markings
			// accurately line up with plotted points
			let mut x0 = minimum_possible_x.clone();
			let x1 = maximum_possible_x.clone();
			'outer_x0: for i in 0..maximum_possible_x {
				// axis extends into negative space so ensure resolution fitting matches half overall length
				if get_x_axis_pixel_length(x0 + i, maximum_possible_x)/2 % x_axis_resolution == 0 {
					x0 = x0 + i;
					break 'outer_x0;
				}
			}
			// // ensures at least 5% of the top canvas is free
			let minimum_possible_y = vertical_pixels_from_top; // + (canvas_size.1 / 20);
			let maximum_possible_y = canvas_size.1 - vertical_pixels_from_bottom;
			// The true length of the axis must be a factor of the resolution so that axis scale markings
			// accurately line up with plotted points
			let mut y1 = minimum_possible_y.clone();
			let y0 = maximum_possible_y.clone();
			'outer_y1: for i in 0..minimum_possible_y {
				// axis extends into negative space so ensure resolution fitting matches half overall length
				if get_y_axis_pixel_length(y1 + i, maximum_possible_y)/2 % y_axis_resolution == 0 && get_y_axis_pixel_length(y1 + i, maximum_possible_y)/2 % 2 == 0 {
					y1 = y1 + i;
					break 'outer_y1;
				}
			}
			return ((x0, y0), (x1, y1));
		}
		Quadrants::TopRight => {
			// ensures the whitespace to the left and right are the same when a legend is not specified
			let minimum_possible_x = horizontal_pixels_from_left;
			let maximum_possible_x =
				canvas_size.0 - (horizontal_pixels_from_right * legend_scale_factor);
			// The true length of the axis must be a factor of the resolution so that axis scale markings
			// accurately line up with plotted points
			let mut x = maximum_possible_x.clone();
			'outer_x: for i in 0..maximum_possible_x {
				if get_x_axis_pixel_length(minimum_possible_x, x - i) % x_axis_resolution == 0 {
					x = x - i;
					break 'outer_x;
				}
			}
			// // ensures at least 5% of the top canvas is free
			let minimum_possible_y = vertical_pixels_from_top; // + (canvas_size.1 / 20);
			let maximum_possible_y = canvas_size.1 - vertical_pixels_from_bottom;
			// The true length of the axis must be a factor of the resolution so that axis scale markings
			// accurately line up with plotted points
			let mut y = minimum_possible_y.clone();
			'outer_y: for i in 0..minimum_possible_y {
				if get_y_axis_pixel_length(y + i, maximum_possible_y) % y_axis_resolution == 0 {
					y = y + i;
					break 'outer_y;
				}
			}
			return ((minimum_possible_x, maximum_possible_y), (x, y));
		}
		Quadrants::TopLeft => {
			// ensures the whitespace to the left and right are the same when a legend is not specified
			let minimum_possible_x = horizontal_pixels_from_left;
			let maximum_possible_x = canvas_size.0 - horizontal_pixels_from_right;
			// The true length of the axis must be a factor of the resolution so that axis scale markings
			// accurately line up with plotted points
			let mut x = minimum_possible_x.clone();
			'outer_x_tl: for i in 0..maximum_possible_x {
				if get_x_axis_pixel_length(x + i, minimum_possible_x) % x_axis_resolution == 0 {
					x = x + i;
					break 'outer_x_tl;
				}
			}
			// // ensures at least 5% of the top canvas is free
			let minimum_possible_y = vertical_pixels_from_top; // + (canvas_size.1 / 20);
			let maximum_possible_y = canvas_size.1 - vertical_pixels_from_bottom;
			// The true length of the axis must be a factor of the resolution so that axis scale markings
			// accurately line up with plotted points
			let mut y = minimum_possible_y.clone();
			'outer_y_tl: for i in 0..minimum_possible_y {
				if get_y_axis_pixel_length(y + i, maximum_possible_y) % y_axis_resolution == 0 {
					y = y + i;
					break 'outer_y_tl;
				}
			}
			return ((x, maximum_possible_y), (maximum_possible_x, y));
		},
		Quadrants::BottomRight => {
			// ensures the whitespace to the left and right are the same when a legend is not specified
			let minimum_possible_x = horizontal_pixels_from_left;
			let maximum_possible_x =
				canvas_size.0 - (horizontal_pixels_from_right * legend_scale_factor);
			// The true length of the axis must be a factor of the resolution so that axis scale markings
			// accurately line up with plotted points
			let mut x = maximum_possible_x.clone();
			'outer_x_br: for i in 0..maximum_possible_x {
				if get_x_axis_pixel_length(minimum_possible_x, x - i) % x_axis_resolution == 0 {
					x = x - i;
					break 'outer_x_br;
				}
			}
			// // ensures at least 5% of the top canvas is free
			let minimum_possible_y = vertical_pixels_from_top; // + (canvas_size.1 / 20);
			let maximum_possible_y = canvas_size.1 - vertical_pixels_from_bottom;
			// The true length of the axis must be a factor of the resolution so that axis scale markings
			// accurately line up with plotted points
			let mut y = minimum_possible_y.clone();
			'outer_y_br: for i in 0..maximum_possible_y {
				if get_y_axis_pixel_length(y + i, maximum_possible_y) % y_axis_resolution == 0 {
					y = y + i;
					break 'outer_y_br;
				}
			}
			return ((minimum_possible_x, maximum_possible_y), (x, y));
		},
		Quadrants::BottomLeft => {
			// ensures the whitespace to the left and right are the same when a legend is not specified
			let minimum_possible_x = horizontal_pixels_from_left;
			let maximum_possible_x = canvas_size.0 - horizontal_pixels_from_right;
			// The true length of the axis must be a factor of the resolution so that axis scale markings
			// accurately line up with plotted points
			let mut x = minimum_possible_x.clone();
			'outer_x_bl: for i in 0..maximum_possible_x {
				if get_x_axis_pixel_length(x + i, minimum_possible_x) % x_axis_resolution == 0 {
					x = x + i;
					break 'outer_x_bl;
				}
			}
			// // ensures at least 5% of the top canvas is free
			let minimum_possible_y = vertical_pixels_from_top; // + (canvas_size.1 / 20);
			let maximum_possible_y = canvas_size.1 - vertical_pixels_from_bottom;
			// The true length of the axis must be a factor of the resolution so that axis scale markings
			// accurately line up with plotted points
			let mut y = minimum_possible_y.clone();
			'outer_y_bl: for i in 0..maximum_possible_y {
				if get_y_axis_pixel_length(y + i, maximum_possible_y) % y_axis_resolution == 0 {
					y = y + i;
					break 'outer_y_bl;
				}
			}
			return ((x, maximum_possible_y), (maximum_possible_x, y));
		},
	}
}
/// Find the pixel pair which pinpoints the origin of the x-y axes based on wha quadrants need to be drawn.
pub fn get_xy_axis_pixel_origin(
	quadrants: &Quadrants,
	axis_min: (u32, u32),
	axis_max: (u32, u32),
) -> (u32, u32) {
	match quadrants {
		Quadrants::RightPair => {
			let y_l = get_y_axis_pixel_length(axis_max.1, axis_min.1);
			// true origin of y
			let y_o = axis_max.1 + y_l/2;
			return (axis_min.0, y_o);
		},
		Quadrants::LeftPair => {
			let y_l = get_y_axis_pixel_length(axis_max.1, axis_min.1);
			// true origin of y
			let y_o = axis_max.1 + y_l/2;
			return (axis_max.0, y_o);
		},
		Quadrants::TopPair => {
			// positive and negative x range means y bisects it in the middle
			let x_l = get_x_axis_pixel_length(axis_min.0, axis_max.0);
			// true origin of x
			let x_o = axis_min.0 + x_l/2;
			return (x_o, axis_min.1);
		},
		Quadrants::BottomPair => {
			// positive and negative x range means y bisects it in the middle
			let x_l = get_x_axis_pixel_length(axis_min.0, axis_max.0);
			// true origin of x
			let x_o = axis_min.0 + x_l/2;
			return (x_o, axis_max.1);
		},
		Quadrants::AllQuadrants => {
			// all quadrants means that the x-y axes are evenly bisected
			let x_l = get_x_axis_pixel_length(axis_min.0, axis_max.0);
			// true origin of x
			let x_o = axis_min.0 + x_l/2;
			let y_l = get_y_axis_pixel_length(axis_max.1, axis_min.1);
			// true origin of y
			let y_o = axis_max.1 + y_l/2;
			return (x_o, y_o);
		}
		Quadrants::TopRight => {
			// axis min-max can be ised to create a diagnonal line equation which can be used to find the `y` pixel
			// which corresponds to the origin
			let gradient = (axis_min.1 - axis_max.1) / (axis_max.0 - axis_min.0);
			let intercept = axis_min.1 - (gradient * axis_min.0);
			let pixel_origin_y = gradient * axis_min.0 + intercept;
			(axis_min.0, pixel_origin_y)
		}
		Quadrants::TopLeft => {
			// axis min-max can be ised to create a diagnonal line equation which can be used to find the `y` pixel
			// which corresponds to the origin
			let gradient = (axis_min.1 - axis_max.1) / (axis_max.0 - axis_min.0);
			let intercept = axis_min.1 - (gradient * axis_min.0);
			let pixel_origin_y = gradient * axis_max.0 + intercept;
			(axis_max.0, pixel_origin_y)
		},
		Quadrants::BottomRight => {
			(axis_min.0, axis_max.1)
		},
		Quadrants::BottomLeft => {
			(axis_max.0, axis_max.1)
		},
	}
}
/// Get the pixel length of the x-axis
pub fn get_x_axis_pixel_length(min_pixel: u32, max_pixel: u32) -> u32 {
	let length = max_pixel.overflowing_sub(min_pixel);
	if length.1 {
		panic!("X-axis length overflow!");
	}
	length.0
}
/// Get the pixel length of the y-axis
pub fn get_y_axis_pixel_length(min_pixel: u32, max_pixel: u32) -> u32 {
	let length = max_pixel.overflowing_sub(min_pixel);
	if length.1 {
		panic!("Y-axis length overflow!");
	}
	length.0
}

/// Within the acceptable pixel space for the axes draw them, note the top left corner of the canvas is the origin `(0, 0)` with bottom right `(canvas.dimensions().0, canvas.dimensions().1)`
pub fn draw_xy_axes(
	quadrants: &Quadrants,
	canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
	axis_origin_pixel: (u32, u32),
	axis_min_pixel: (u32, u32),
	axis_max_pixel: (u32, u32),
	x_axis_length: u32,
	y_axis_length: u32,
	x_data_min_max_limits: (i32, i32),
	y_data_min_max_limits: (i32, i32),
	font_size: f32,
	has_grid: bool,
	x_axis_resolution: u32,
	y_axis_resolution: u32,
) {
	// x-axis data labels
	draw_x_axis_scale_markings(
		quadrants,
		canvas,
		axis_min_pixel,
		axis_origin_pixel,
		axis_max_pixel,
		x_axis_length,
		x_data_min_max_limits,
		font_size,
		has_grid,
		x_axis_resolution,
	);
	// x-axis
	draw_x_axis(
		canvas,
		axis_min_pixel,
		axis_origin_pixel,
		axis_max_pixel,
	);
	// y-axis data labels
	draw_y_axis_scale_markings(
		quadrants,
		canvas,
		axis_min_pixel,
		axis_origin_pixel,
		axis_max_pixel,
		y_axis_length,
		y_data_min_max_limits,
		font_size,
		has_grid,
		y_axis_resolution,
	);
	// y-axis
	draw_y_axis(
		canvas,
		axis_min_pixel,
		axis_origin_pixel,
		axis_max_pixel,
	);
}
/// Draws the x-axis where the static `y` poistion is defined in the `axis_origin_pixel` tuple. This is a result
/// of the image origin being based in the top-left corner while the graph origin is in the bottom left
fn draw_x_axis(
	canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
	axis_min_pixel: (u32, u32),
	axis_origin_pixel: (u32, u32),
	axis_max_pixel: (u32, u32),
) {
	debug!("Drawing x-axis");
	// draw from the origin to max pixel
	for px in axis_origin_pixel.0..=(axis_max_pixel.0) {
		canvas.put_pixel(px, axis_origin_pixel.1, Rgba(BLACK));
	}
	// draw from min pixel to origin
	for px in axis_min_pixel.0..=(axis_origin_pixel.0) {
		canvas.put_pixel(px, axis_origin_pixel.1, Rgba(BLACK));
	}
}
/// Draws the scale markings along the x-axis
fn draw_x_axis_scale_markings(
	quadrants: &Quadrants,
	canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
	axis_min_pixel: (u32, u32),
	axis_origin_pixel: (u32, u32),
	axis_max_pixel: (u32, u32),
	x_axis_length: u32,
	x_data_min_max_limits: (i32, i32),
	font_size: f32,
	has_grid: bool,
	x_axis_resolution: u32,
) {
	let font = get_system_font();
	debug!("Drawing x-axis scale markings");
	// we find the appropriate layout to draw markers, this ensures scale resolution markers are correctly drawn across positive and negative axes
	match quadrants {
		// vareints with a positive and negative x-axis
		Quadrants::AllQuadrants | Quadrants::TopPair | Quadrants::BottomPair => {
			// Subdivide the x-axis length into a number of points we can draw labels at.
			// The number of pixels along the x-axis between each data label.
			// We halve the axis length as we need to draw the resolution twice, once is the
			// positive direction and once in the negative direction
			let x_subdivision_length = (x_axis_length/2) / x_axis_resolution;
			// The pixel length of each data label
			let data_label_length = 5;
			// For writing a value for each data label we need to know the overall data size that corrpesonds to the axis
			let x_value_range = x_data_min_max_limits.1 as f32 - x_data_min_max_limits.0 as f32;
			// Find how much a suddivsion is in terms of data value
			let x_value_per_subdivision = (x_value_range/2.0) / x_axis_resolution as f32;
			// If required draw the x part of a background grid as grey vertical lines
			if has_grid {
				trace!("Drawing grey background grid...");
				// draw in positive x direction
				for i in 0..(x_axis_resolution + 1) {
					for py in axis_max_pixel.1..axis_min_pixel.1 {
						canvas.put_pixel(
							axis_origin_pixel.0 + (i * x_subdivision_length),
							py,
							Rgba(GREY),
						);
					}
				}
				// draw in negative x direction
				for i in 0..(x_axis_resolution + 1) {
					for py in axis_max_pixel.1..axis_min_pixel.1 {
						canvas.put_pixel(
							axis_origin_pixel.0 - (i * x_subdivision_length),
							py,
							Rgba(GREY),
						);
					}
				}
			}
			// Draw a line of pixels down from the axis as each subdivision
			// Draw is positive x direction
			trace!("Marking each x-axis subdivision...");
			for i in 0..(x_axis_resolution + 1) {
				// Draw each even section slightly longer
				let label_length_scale = if i & 1 == 1 { 2 } else { 3 };
				for n in 0..(data_label_length * label_length_scale) {
					// for n in 0..(data_label_length) {
					canvas.put_pixel(
						axis_origin_pixel.0 + (i * x_subdivision_length),
						axis_origin_pixel.1 + n,
						Rgba(BLACK),
					);
				}
				// Draw the data label text
				// Don't draw text over origin otherwise axis is obscured
				if i != 0 {
					let text = (x_value_per_subdivision * i as f32).to_string();
				let glyphs = create_glyphs(font_size, &text, &font);
				let origin_x = axis_origin_pixel.0 + (i * x_subdivision_length);
				let origin_y = axis_origin_pixel.1 + (data_label_length * label_length_scale);
				// let origin_y = origin_pixel.1 + (data_label_length);
				let offset = get_x_axis_scale_label_offset(&glyphs, origin_x, origin_y);
				trace!("Drawing x-axis label {} at {:?}", text, offset);
				draw_glyphs(canvas, BLACK, glyphs, offset);
				}

				// If there's enough space between each scale marker create mini-markings
				// mini-marker varients
				let marker_count = vec![9, 4, 3, 2, 1];
				'outer_pos: for mc in marker_count.iter() {
					// (mc + 1) because there needs to be a gap between final mini-marker and scale marker
					if x_subdivision_length % (mc + 1) == 0 {
						if i < x_axis_resolution {
							for j in 1..=*mc {
								let marker_spacing = x_subdivision_length / (mc + 1);
								for n in 0..data_label_length {
									canvas.put_pixel(
										axis_origin_pixel.0
											+ ((i * x_subdivision_length) + (j * marker_spacing)),
										axis_origin_pixel.1 + n,
										Rgba(BLACK),
									);
								}
							}
							break 'outer_pos;
						}
					}
				}
			}
			// draw markers in negative x direction
			for i in 0..(x_axis_resolution + 1) {
				// Draw each even section slightly longer
				let label_length_scale = if i & 1 == 1 { 2 } else { 3 };
				for n in 0..(data_label_length * label_length_scale) {
					// for n in 0..(data_label_length) {
					canvas.put_pixel(
						axis_origin_pixel.0 - (i * x_subdivision_length),
						axis_origin_pixel.1 + n,
						Rgba(BLACK),
					);
				}
				// Draw the data label text
				// Don't draw text over origin otherwise axis is obscured
				if i != 0 {
					let text =  (-x_value_per_subdivision * i as f32).to_string();
				let glyphs = create_glyphs(font_size, &text, &font);
				let origin_x = axis_origin_pixel.0 - (i * x_subdivision_length);
				let origin_y = axis_origin_pixel.1 + (data_label_length * label_length_scale);
				// let origin_y = origin_pixel.1 + (data_label_length);
				let offset = get_x_axis_scale_label_offset(&glyphs, origin_x, origin_y);
				trace!("Drawing x-axis label {} at {:?}", text, offset);
				draw_glyphs(canvas, BLACK, glyphs, offset);
				}

				// If there's enough space between each scale marker create mini-markings
				// mini-marker varients
				let marker_count = vec![9, 4, 3, 2, 1];
				'outer_neg: for mc in marker_count.iter() {
					// (mc + 1) because there needs to be a gap between final mini-marker and scale marker
					if x_subdivision_length % (mc + 1) == 0 {
						if i < x_axis_resolution {
							for j in 1..=*mc {
								let marker_spacing = x_subdivision_length / (mc + 1);
								for n in 0..data_label_length {
									canvas.put_pixel(
										axis_origin_pixel.0
											- ((i * x_subdivision_length) + (j * marker_spacing)),
										axis_origin_pixel.1 + n,
										Rgba(BLACK),
									);
								}
							}
							break 'outer_neg;
						}
					}
				}
			}
		},
		// varients with only a positive x-axis
		Quadrants::TopRight | Quadrants::BottomRight | Quadrants::RightPair => {
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
				trace!("Drawing grey background grid...");
				for i in 0..(x_axis_resolution + 1) {
					for py in axis_max_pixel.1..axis_min_pixel.1 {
						canvas.put_pixel(
							axis_min_pixel.0 + (i * x_subdivision_length),
							py,
							Rgba(GREY),
						);
					}
				}
			}
			// Draw a line of pixels down from the axis as each subdivision
			trace!("Marking each x-axis subdivision...");
			for i in 0..(x_axis_resolution + 1) {
				// Draw each even section slightly longer
				let label_length_scale = if i & 1 == 1 { 2 } else { 3 };
				for n in 0..(data_label_length * label_length_scale) {
					// for n in 0..(data_label_length) {
					canvas.put_pixel(
						axis_min_pixel.0 + (i * x_subdivision_length),
						axis_origin_pixel.1 + n,
						Rgba(BLACK),
					);
				}
				// Draw the data label text
				let text = (x_data_min_max_limits.0 as f32 + (x_value_per_subdivision * i as f32)).to_string();
				let glyphs = create_glyphs(font_size, &text, &font);
				let origin_x = axis_min_pixel.0 + (i * x_subdivision_length);
				let origin_y = axis_origin_pixel.1 + (data_label_length * label_length_scale);
				// let origin_y = origin_pixel.1 + (data_label_length);
				let offset = get_x_axis_scale_label_offset(&glyphs, origin_x, origin_y);
				trace!("Drawing x-axis label {} at {:?}", text, offset);
				draw_glyphs(canvas, BLACK, glyphs, offset);

				// If there's enough space between each scale marker create mini-markings
				// mini-marker varients
				let marker_count = vec![9, 4, 3, 2, 1];
				'outer: for mc in marker_count.iter() {
					// (mc + 1) because there needs to be a gap between final mini-marker and scale marker
					if x_subdivision_length % (mc + 1) == 0 {
						if i < x_axis_resolution {
							for j in 1..=*mc {
								let marker_spacing = x_subdivision_length / (mc + 1);
								for n in 0..data_label_length {
									canvas.put_pixel(
										axis_min_pixel.0
											+ ((i * x_subdivision_length) + (j * marker_spacing)),
										axis_origin_pixel.1 + n,
										Rgba(BLACK),
									);
								}
							}
							break 'outer;
						}
					}
				}
			}
		},
		// varients with a negative x-axis
		Quadrants::LeftPair | Quadrants::TopLeft | Quadrants::BottomLeft => {
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
				trace!("Drawing grey background grid...");
				for i in 0..(x_axis_resolution + 1) {
					for py in axis_max_pixel.1..axis_min_pixel.1 {
						canvas.put_pixel(
							axis_origin_pixel.0 - (i * x_subdivision_length),
							py,
							Rgba(GREY),
						);
					}
				}
			}
			// Draw a line of pixels down from the axis as each subdivision
			trace!("Marking each x-axis subdivision...");
			for i in 0..(x_axis_resolution + 1) {
				// Draw each even section slightly longer
				let label_length_scale = if i & 1 == 1 { 2 } else { 3 };
				for n in 0..(data_label_length * label_length_scale) {
					// for n in 0..(data_label_length) {
					canvas.put_pixel(
						axis_origin_pixel.0 - (i * x_subdivision_length),
						axis_origin_pixel.1 + n,
						Rgba(BLACK),
					);
				}
				// Draw the data label text
				let text = (-x_value_per_subdivision * i as f32).to_string();
				let glyphs = create_glyphs(font_size, &text, &font);
				let origin_x = axis_origin_pixel.0 - (i * x_subdivision_length);
				let origin_y = axis_origin_pixel.1 + (data_label_length * label_length_scale);
				// let origin_y = origin_pixel.1 + (data_label_length);
				let offset = get_x_axis_scale_label_offset(&glyphs, origin_x, origin_y);
				trace!("Drawing x-axis label {} at {:?}", text, offset);
				draw_glyphs(canvas, BLACK, glyphs, offset);

				// If there's enough space between each scale marker create mini-markings
				// mini-marker varients
				let marker_count = vec![9, 4, 3, 2, 1];
				'outer_l_neg: for mc in marker_count.iter() {
					// (mc + 1) because there needs to be a gap between final mini-marker and scale marker
					if x_subdivision_length % (mc + 1) == 0 {
						if i < x_axis_resolution {
							for j in 1..=*mc {
								let marker_spacing = x_subdivision_length / (mc + 1);
								for n in 0..data_label_length {
									canvas.put_pixel(
										axis_origin_pixel.0
											- ((i * x_subdivision_length) + (j * marker_spacing)),
										axis_origin_pixel.1 + n,
										Rgba(BLACK),
									);
								}
							}
							break 'outer_l_neg;
						}
					}
				}
			}
		},
	}
}

/// Using glyph sizes calculate by how much the axis data label should be offset from an origin point
fn get_x_axis_scale_label_offset(
	glyphs: &Vec<PositionedGlyph>,
	origin_x: u32,
	origin_y: u32,
) -> (u32, u32) {
	let width = get_width_of_glyphs(glyphs);
	let height = get_maximum_height_of_glyphs(glyphs);
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

/// Draws the y-axis with s static `x` position taken from `axis_origin_pixel`
fn draw_y_axis(
	canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
	axis_min_pixel: (u32, u32),
	axis_origin_pixel: (u32, u32),
	axis_max_pixel: (u32, u32),
) {
	debug!("Drawing y-axis");
	// max to origin
	for py in axis_max_pixel.1..=axis_origin_pixel.1 {
		canvas.put_pixel(axis_origin_pixel.0, py, Rgba(BLACK));
	}
	// origin to min
	for py in axis_origin_pixel.1..=axis_min_pixel.1 {
		canvas.put_pixel(axis_origin_pixel.0, py, Rgba(BLACK));
	}
}

/// Draws the scale markings along the x-axis
fn draw_y_axis_scale_markings(
	quadrants: &Quadrants,
	canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
	axis_min_pixel: (u32, u32),
	axis_origin_pixel: (u32, u32),
	axis_max_pixel: (u32, u32),
	y_axis_length: u32,
	y_data_min_max_limits: (i32, i32),
	font_size: f32,
	has_grid: bool,
	y_axis_resolution: u32,
) {
	let font = get_system_font();
	debug!("Drawing y-axis scale markers");
	// we find the appropriate layout to draw markers, this ensures scale resolution markers are correctly drawn across positive and negative axes
	match quadrants {
     // varients with a positive and negative y-axis
    Quadrants::AllQuadrants | Quadrants::LeftPair | Quadrants::RightPair => {
		// Subdivide the y-axis length into a number of points we can draw labels at.
		// The number of pixels along the x-axis between each data label
		// Halve axis length as resolution needs to fit both positive and negative wings
		let subdivision_length = (y_axis_length/2) / y_axis_resolution;
		// The pixel length of each data label
		let data_label_length = 5;
		// For writing a value for each data label we need to know the overall data size that corrpesonds to the axis
		let value_range = y_data_min_max_limits.1 as f32 - y_data_min_max_limits.0 as f32;
		// Find how much a suddivsion is in terms of data value
		let value_per_subdivision = (value_range/2.0) / y_axis_resolution as f32;
		// If required draw the y part of a background grid as grey vertical lines
		if has_grid {
			trace!("Drawing grey background grid...");
			// draw lines in positive space
			for i in 1..(y_axis_resolution + 1) {
				for px in axis_min_pixel.0..axis_max_pixel.0 {
					canvas.put_pixel(px, axis_origin_pixel.1 - (i * subdivision_length), Rgba(GREY));
				}
			}
			// draw lines in negative space
			for i in 1..(y_axis_resolution + 1) {
				for px in axis_min_pixel.0..axis_max_pixel.0 {
					canvas.put_pixel(px, axis_origin_pixel.1 + (i * subdivision_length), Rgba(GREY));
				}
			}
		}
		// Draw a line of pixels down from the axis as each subdivision
		// First in positive y space
		trace!("Marking each y-axis subdivision...");
		for i in 0..(y_axis_resolution + 1) {
			// Draw each even section slightly longer
			let label_length_scale = if i & 1 == 1 { 2 } else { 3 };
			for n in 0..(data_label_length * label_length_scale) {
				canvas.put_pixel(
					axis_origin_pixel.0 - n,
					axis_origin_pixel.1 - (i * subdivision_length),
					Rgba(BLACK),
				);
			}
			// Draw the data label text
			// Don't draw text directly over origin ortherwise it obscures axis
			if i != 0 {
				let text = (value_per_subdivision * i as f32).to_string();
			let glyphs = create_glyphs(font_size, &text, &font);
			let origin_x = axis_origin_pixel.0 - (data_label_length * label_length_scale) * 2;
			let origin_y = axis_origin_pixel.1 - (i * subdivision_length);
			let offset = get_y_axis_scale_label_offset(&glyphs, origin_x, origin_y);
			trace!("Drawing y-axis label {} at {:?}", text, offset);
			draw_glyphs(canvas, BLACK, glyphs, offset);
			}

			// If there's enough space between each scale marker create mini-markings
			// mini-marker varients
			let marker_count = vec![9, 4, 3, 2, 1];
			'outer_pos: for mc in marker_count.iter() {
				// (mc + 1) because there needs to be a gap between final mini-marker and scale marker
				if subdivision_length % (mc + 1) == 0 {
					if i < y_axis_resolution {
						for j in 1..=*mc {
							let marker_spacing = subdivision_length / (mc + 1);
							for n in 0..data_label_length {
								canvas.put_pixel(
									axis_origin_pixel.0 - n,
									axis_origin_pixel.1 - ((i * subdivision_length) + (j * marker_spacing)),
									Rgba(BLACK),
								);
							}
						}
						break 'outer_pos;
					}
				}
			}
		}
		// markers in negative space
		for i in 0..(y_axis_resolution + 1) {
			// Draw each even section slightly longer
			let label_length_scale = if i & 1 == 1 { 2 } else { 3 };
			for n in 0..(data_label_length * label_length_scale) {
				canvas.put_pixel(
					axis_origin_pixel.0 - n,
					axis_origin_pixel.1 + (i * subdivision_length),
					Rgba(BLACK),
				);
			}
			// Draw the data label text
			// Don't draw text directly over origin ortherwise it obscures axis
			if i != 0 {
				let text = (-value_per_subdivision * i as f32).to_string();
			let glyphs = create_glyphs(font_size, &text, &font);
			let origin_x = axis_origin_pixel.0 - (data_label_length * label_length_scale) * 2;
			let origin_y = axis_origin_pixel.1 + (i * subdivision_length);
			let offset = get_y_axis_scale_label_offset(&glyphs, origin_x, origin_y);
			trace!("Drawing y-axis label {} at {:?}", text, offset);
			draw_glyphs(canvas, BLACK, glyphs, offset);
			}

			// If there's enough space between each scale marker create mini-markings
			// mini-marker varients
			let marker_count = vec![9, 4, 3, 2, 1];
			'outer_neg: for mc in marker_count.iter() {
				// (mc + 1) because there needs to be a gap between final mini-marker and scale marker
				if subdivision_length % (mc + 1) == 0 {
					if i < y_axis_resolution {
						for j in 1..=*mc {
							let marker_spacing = subdivision_length / (mc + 1);
							for n in 0..data_label_length {
								canvas.put_pixel(
									axis_origin_pixel.0 - n,
									axis_origin_pixel.1 + ((i * subdivision_length) + (j * marker_spacing)),
									Rgba(BLACK),
								);
							}
						}
						break 'outer_neg;
					}
				}
			}
		}
	},
	// varients with just a positive y-axis
    Quadrants::TopRight | Quadrants::TopLeft | Quadrants::TopPair => {
		// Subdivide the y-axis length into a number of points we can draw labels at.
		// The number of pixels along the x-axis between each data label
		let subdivision_length = y_axis_length / y_axis_resolution;
		// The pixel length of each data label
		let data_label_length = 5;
		// For writing a value for each data label we need to know the overall data size that corrpesonds to the axis
		let value_range = y_data_min_max_limits.1 as f32 - y_data_min_max_limits.0 as f32;
		// Find how much a suddivsion is in terms of data value
		let value_per_subdivision = value_range / y_axis_resolution as f32;
		// If required draw the y part of a background grid as grey vertical lines
		if has_grid {
			trace!("Drawing grey background grid...");
			for i in 0..(y_axis_resolution + 1) {
				for px in axis_min_pixel.0..axis_max_pixel.0 {
					canvas.put_pixel(px, axis_min_pixel.1 - (i * subdivision_length), Rgba(GREY));
				}
			}
		}
		// Draw a line of pixels down from the axis as each subdivisions
		trace!("Marking each y-axis subdivision...");
		for i in 0..(y_axis_resolution + 1) {
			// Draw each even section slightly longer
			let label_length_scale = if i & 1 == 1 { 2 } else { 3 };
			for n in 0..(data_label_length * label_length_scale) {
				canvas.put_pixel(
					axis_origin_pixel.0 - n,
					axis_min_pixel.1 - (i * subdivision_length),
					Rgba(BLACK),
				);
			}
			// Draw the data label text
			let text = (y_data_min_max_limits.0 as f32 + (value_per_subdivision * i as f32)).to_string();
			let glyphs = create_glyphs(font_size, &text, &font);
			let origin_x = axis_origin_pixel.0 - (data_label_length * label_length_scale) * 2;
			let origin_y = axis_min_pixel.1 - (i * subdivision_length);
			let offset = get_y_axis_scale_label_offset(&glyphs, origin_x, origin_y);
			trace!("Drawing y-axis label {} at {:?}", text, offset);
			draw_glyphs(canvas, BLACK, glyphs, offset);

			// If there's enough space between each scale marker create mini-markings
			// mini-marker varients
			let marker_count = vec![9, 4, 3, 2, 1];
			'outer: for mc in marker_count.iter() {
				// (mc + 1) because there needs to be a gap between final mini-marker and scale marker
				if subdivision_length % (mc + 1) == 0 {
					if i < y_axis_resolution {
						for j in 1..=*mc {
							let marker_spacing = subdivision_length / (mc + 1);
							for n in 0..data_label_length {
								canvas.put_pixel(
									axis_origin_pixel.0 - n,
									axis_min_pixel.1 - ((i * subdivision_length) + (j * marker_spacing)),
									Rgba(BLACK),
								);
							}
						}
						break 'outer;
					}
				}
			}
		}
	},
	// varients with just a negative y-axis
    Quadrants::BottomRight | Quadrants::BottomLeft | Quadrants::BottomPair => {
		// Subdivide the y-axis length into a number of points we can draw labels at.
		// The number of pixels along the x-axis between each data label
		// Halve axis length as resolution needs to fit both positive and negative wings
		let subdivision_length = y_axis_length / y_axis_resolution;
		// The pixel length of each data label
		let data_label_length = 5;
		// For writing a value for each data label we need to know the overall data size that corrpesonds to the axis
		let value_range = y_data_min_max_limits.1 as f32 - y_data_min_max_limits.0 as f32;
		// Find how much a suddivsion is in terms of data value
		let value_per_subdivision = value_range / y_axis_resolution as f32;
		// If required draw the y part of a background grid as grey vertical lines
		if has_grid {
			trace!("Drawing grey background grid...");
			// draw lines in negative space
			for i in 0..(y_axis_resolution + 1) {
				for px in axis_min_pixel.0..axis_max_pixel.0 {
					canvas.put_pixel(px, axis_origin_pixel.1 + (i * subdivision_length), Rgba(GREY));
				}
			}
		}
		// Draw a line of pixels down from the axis as each subdivision
		trace!("Marking each y-axis subdivision...");
		// markers in negative space
		for i in 0..(y_axis_resolution + 1) {
			// Draw each even section slightly longer
			let label_length_scale = if i & 1 == 1 { 2 } else { 3 };
			for n in 0..(data_label_length * label_length_scale) {
				canvas.put_pixel(
					axis_origin_pixel.0 - n,
					axis_origin_pixel.1 + (i * subdivision_length),
					Rgba(BLACK),
				);
			}
			// Draw the data label text
			// Don't draw text directly over origin ortherwise it obscures axis
			if i != 0 {
				let text = (-value_per_subdivision * i as f32).to_string();
			let glyphs = create_glyphs(font_size, &text, &font);
			let origin_x = axis_origin_pixel.0 - (data_label_length * label_length_scale) * 2;
			let origin_y = axis_origin_pixel.1 + (i * subdivision_length);
			let offset = get_y_axis_scale_label_offset(&glyphs, origin_x, origin_y);
			trace!("Drawing y-axis label {} at {:?}", text, offset);
			draw_glyphs(canvas, BLACK, glyphs, offset);
			}

			// If there's enough space between each scale marker create mini-markings
			// mini-marker varients
			let marker_count = vec![9, 4, 3, 2, 1];
			'outer_b_neg: for mc in marker_count.iter() {
				// (mc + 1) because there needs to be a gap between final mini-marker and scale marker
				if subdivision_length % (mc + 1) == 0 {
					if i < y_axis_resolution {
						for j in 1..=*mc {
							let marker_spacing = subdivision_length / (mc + 1);
							for n in 0..data_label_length {
								canvas.put_pixel(
									axis_origin_pixel.0 - n,
									axis_origin_pixel.1 + ((i * subdivision_length) + (j * marker_spacing)),
									Rgba(BLACK),
								);
							}
						}
						break 'outer_b_neg;
					}
				}
			}
		}
	},
}
}

/// Using glyph sizes calculate by how much the axis data label should be offset from an origin point
fn get_y_axis_scale_label_offset(
	glyphs: &Vec<PositionedGlyph>,
	origin_x: u32,
	origin_y: u32,
) -> (u32, u32) {
	let width = get_width_of_glyphs(glyphs);
	let height = get_maximum_height_of_glyphs(glyphs);

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
