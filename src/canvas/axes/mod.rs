//! Methods for creating and labeling axes and determining dimensions

use image::{ImageBuffer, Rgba};

use self::{
	axis_x::{draw_x_axis, draw_x_axis_scale_markings, get_x_axis_pixel_length},
	axis_y::{draw_y_axis, draw_y_axis_scale_markings, get_y_axis_pixel_length},
};

use super::quadrants::Quadrants;

pub mod axis_x;
pub mod axis_y;

/// Find the pixel pair which pinpoints the maxmium length and height of the axes. Resolutions are
/// used to ensure that the length of each axis is a natural scale factor of the resolution. This
/// allows for accurately plotting data points
pub fn get_xy_axis_pixel_min_max(
	quadrants: &Quadrants,
	vertical_pixels_from_top: u32,
	horizontal_pixels_from_right: u32,
	vertical_pixels_from_bottom: u32,
	horizontal_pixels_from_left: u32,
	canvas_size: (u32, u32),
	x_axis_resolution: u32,
	y_axis_resolution: u32,
) -> ((u32, u32), (u32, u32)) {
	match quadrants {
		Quadrants::RightPair => {
			// ensures the whitespace to the left and right are the same when a legend is not specified
			let minimum_possible_x = horizontal_pixels_from_left;
			let maximum_possible_x = canvas_size.0 - horizontal_pixels_from_right;
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
				if get_y_axis_pixel_length(y1 + i, maximum_possible_y) / 2 % y_axis_resolution == 0
					&& get_y_axis_pixel_length(y1 + i, maximum_possible_y) / 2 % 2 == 0
				{
					y1 = y1 + i;
					break 'outer_y_rp;
				}
			}
			return ((minimum_possible_x, y0), (x, y1));
		}
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
				if get_y_axis_pixel_length(y1 + i, maximum_possible_y) / 2 % y_axis_resolution == 0
					&& get_y_axis_pixel_length(y1 + i, maximum_possible_y) / 2 % 2 == 0
				{
					y1 = y1 + i;
					break 'outer_y_lp;
				}
			}
			return ((x, y0), (maximum_possible_x, y1));
		}
		Quadrants::TopPair => {
			// ensures the whitespace to the left and right are the same when a legend is not specified
			let minimum_possible_x = horizontal_pixels_from_left;
			let maximum_possible_x = canvas_size.0 - horizontal_pixels_from_right;
			// The true length of the axis must be a factor of the resolution so that axis scale markings
			// accurately line up with plotted points
			let mut x0 = minimum_possible_x.clone();
			let x1 = maximum_possible_x.clone();
			'outer_x_tp: for i in 0..maximum_possible_x {
				// axis extends into negative space so ensure resolution fitting matches half overall length
				if get_x_axis_pixel_length(x0 + i, maximum_possible_x) / 2 % x_axis_resolution == 0
				{
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
		}
		Quadrants::BottomPair => {
			// ensures the whitespace to the left and right are the same when a legend is not specified
			let minimum_possible_x = horizontal_pixels_from_left;
			let maximum_possible_x = canvas_size.0 - horizontal_pixels_from_right;
			// The true length of the axis must be a factor of the resolution so that axis scale markings
			// accurately line up with plotted points
			let mut x0 = minimum_possible_x.clone();
			let x1 = maximum_possible_x.clone();
			'outer_x_bp: for i in 0..maximum_possible_x {
				// axis extends into negative space so ensure resolution fitting matches half overall length
				if get_x_axis_pixel_length(x0 + i, maximum_possible_x) / 2 % x_axis_resolution == 0
				{
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
		}
		Quadrants::AllQuadrants => {
			// ensures the whitespace to the left and right are the same when a legend is not specified
			let minimum_possible_x = horizontal_pixels_from_left;
			let maximum_possible_x = canvas_size.0 - horizontal_pixels_from_right;
			// The true length of the axis must be a factor of the resolution so that axis scale markings
			// accurately line up with plotted points
			let mut x0 = minimum_possible_x.clone();
			let x1 = maximum_possible_x.clone();
			'outer_x0: for i in 0..maximum_possible_x {
				// axis extends into negative space so ensure resolution fitting matches half overall length
				if get_x_axis_pixel_length(x0 + i, maximum_possible_x) / 2 % x_axis_resolution == 0
				{
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
				if get_y_axis_pixel_length(y1 + i, maximum_possible_y) / 2 % y_axis_resolution == 0
					&& get_y_axis_pixel_length(y1 + i, maximum_possible_y) / 2 % 2 == 0
				{
					y1 = y1 + i;
					break 'outer_y1;
				}
			}
			return ((x0, y0), (x1, y1));
		}
		Quadrants::TopRight => {
			// ensures the whitespace to the left and right are the same when a legend is not specified
			let minimum_possible_x = horizontal_pixels_from_left;
			let maximum_possible_x = canvas_size.0 - horizontal_pixels_from_right;
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
		}
		Quadrants::BottomRight => {
			// ensures the whitespace to the left and right are the same when a legend is not specified
			let minimum_possible_x = horizontal_pixels_from_left;
			let maximum_possible_x = canvas_size.0 - horizontal_pixels_from_right;
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
		}
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
		}
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
			let y_o = axis_max.1 + y_l / 2;
			return (axis_min.0, y_o);
		}
		Quadrants::LeftPair => {
			let y_l = get_y_axis_pixel_length(axis_max.1, axis_min.1);
			// true origin of y
			let y_o = axis_max.1 + y_l / 2;
			return (axis_max.0, y_o);
		}
		Quadrants::TopPair => {
			// positive and negative x range means y bisects it in the middle
			let x_l = get_x_axis_pixel_length(axis_min.0, axis_max.0);
			// true origin of x
			let x_o = axis_min.0 + x_l / 2;
			return (x_o, axis_min.1);
		}
		Quadrants::BottomPair => {
			// positive and negative x range means y bisects it in the middle
			let x_l = get_x_axis_pixel_length(axis_min.0, axis_max.0);
			// true origin of x
			let x_o = axis_min.0 + x_l / 2;
			return (x_o, axis_max.1);
		}
		Quadrants::AllQuadrants => {
			// all quadrants means that the x-y axes are evenly bisected
			let x_l = get_x_axis_pixel_length(axis_min.0, axis_max.0);
			// true origin of x
			let x_o = axis_min.0 + x_l / 2;
			let y_l = get_y_axis_pixel_length(axis_max.1, axis_min.1);
			// true origin of y
			let y_o = axis_max.1 + y_l / 2;
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
		}
		Quadrants::BottomRight => (axis_min.0, axis_max.1),
		Quadrants::BottomLeft => (axis_max.0, axis_max.1),
	}
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
	draw_x_axis(canvas, axis_min_pixel, axis_origin_pixel, axis_max_pixel);
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
	draw_y_axis(canvas, axis_min_pixel, axis_origin_pixel, axis_max_pixel);
}
