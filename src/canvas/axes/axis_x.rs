//! Draws the x-axis with labels and scale markers

use image::{ImageBuffer, Rgba};
use rusttype::PositionedGlyph;
use tracing::{debug, trace};

use crate::{
	canvas::{
		glyphs::{create_glyphs, draw_glyphs, get_maximum_height_of_glyphs, get_width_of_glyphs},
		quadrants::Quadrants,
		VHConsumedCanvasSpace, CANVAS_BORDER_PIXELS,
	},
	colours::*,
	get_system_font,
};

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
		Quadrants::RightPair | Quadrants::AllQuadrants => {
			debug!("Placing x-axis label in centre right");
			let position: (u32, u32) = (
				canvas.dimensions().0 - CANVAS_BORDER_PIXELS - width - horizontal_pixels_from_right,
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
		}
		Quadrants::TopPair | Quadrants::TopRight => {
			debug!("Placing x-axis label in bottom right corner");
			let position: (u32, u32) = (
				canvas.dimensions().0 - CANVAS_BORDER_PIXELS - width - horizontal_pixels_from_right,
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
		Quadrants::BottomPair | Quadrants::BottomRight => {
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
		}
		Quadrants::TopLeft => {
			debug!("Placing x-axis label in bottom left corner");
			let position: (u32, u32) = (
				horizontal_pixels_from_left + (width / 2),
				canvas.dimensions().1 - CANVAS_BORDER_PIXELS - height,
			);
			draw_glyphs(canvas, BLACK, glyphs, position);
			VHConsumedCanvasSpace {
				v_space_from_top: 0,
				h_space_from_left: width + CANVAS_BORDER_PIXELS,
				v_space_from_bottom: CANVAS_BORDER_PIXELS + (height * 2) + CANVAS_BORDER_PIXELS,
				h_space_from_right: 0,
			}
		}
		Quadrants::BottomLeft => {
			debug!("Placing x-axis label in top left corner");
			let position: (u32, u32) = (
				horizontal_pixels_from_left + (width/2),
				vertical_pixels_from_top + height,
			);
			draw_glyphs(canvas, BLACK, glyphs, position);
			VHConsumedCanvasSpace {
				v_space_from_top: (height * 2) + CANVAS_BORDER_PIXELS + CANVAS_BORDER_PIXELS,
				h_space_from_left: width + CANVAS_BORDER_PIXELS,
				v_space_from_bottom: 0,
				h_space_from_right: 0,
			}
		}
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
/// Draws the x-axis where the static `y` poistion is defined in the `axis_origin_pixel` tuple. This is a result
/// of the image origin being based in the top-left corner while the graph origin is in the bottom left
pub fn draw_x_axis(
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
pub fn draw_x_axis_scale_markings(
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
			let x_subdivision_length = (x_axis_length / 2) / x_axis_resolution;
			// The pixel length of each data label
			let data_label_length = 5;
			// For writing a value for each data label we need to know the overall data size that corrpesonds to the axis
			let x_value_range = x_data_min_max_limits.1 as f32 - x_data_min_max_limits.0 as f32;
			// Find how much a suddivsion is in terms of data value
			let x_value_per_subdivision = (x_value_range / 2.0) / x_axis_resolution as f32;
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
					// So that scale markers are not drawn on the graph area itself check which quadrant type
					// and flip if necessary so they are drawn in the available whitespace outside the axis
					let px = axis_origin_pixel.0 + (i * x_subdivision_length);
					let py = if *quadrants == Quadrants::BottomPair {
						axis_origin_pixel.1 - n
					} else {
						axis_origin_pixel.1 + n
					};
					canvas.put_pixel(px, py, Rgba(BLACK));
				}
				// Draw the data label text
				// Don't draw text over origin otherwise axis is obscured
				if i != 0 {
					let text = (x_value_per_subdivision * i as f32).to_string();
					let glyphs = create_glyphs(font_size, &text, &font);
					let origin_x = axis_origin_pixel.0 + (i * x_subdivision_length);
					// So that scale markers are not drawn on the graph area itself check which quadrant type
					// and flip if necessary so they are drawn in the available whitespace outside the axis
					let origin_y = if *quadrants == Quadrants::BottomPair {
						axis_origin_pixel.1
							- (data_label_length * label_length_scale)
					} else {
						axis_origin_pixel.1 + (data_label_length * label_length_scale)
					};
					// let origin_y = origin_pixel.1 + (data_label_length);
					let offset = get_x_axis_scale_label_offset(&glyphs, origin_x, origin_y, quadrants);
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
									let px = axis_origin_pixel.0
										+ ((i * x_subdivision_length) + (j * marker_spacing));
									// ensure mrkers are drawn in whitespace
									let py = if *quadrants == Quadrants::BottomPair {
										axis_origin_pixel.1 - n
									} else {
										axis_origin_pixel.1 + n
									};
									canvas.put_pixel(px, py, Rgba(BLACK));
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
					// So that scale markers are not drawn on the graph area itself check which quadrant type
					// and flip if necessary so they are drawn in the available whitespace outside the axis
					let px = axis_origin_pixel.0 - (i * x_subdivision_length);
					let py = if *quadrants == Quadrants::BottomPair {
						axis_origin_pixel.1 - n
					} else {
						axis_origin_pixel.1 + n
					};
					canvas.put_pixel(px, py, Rgba(BLACK));
				}
				// Draw the data label text
				// Don't draw text over origin otherwise axis is obscured
				if i != 0 {
					let text = (-x_value_per_subdivision * i as f32).to_string();
					let glyphs = create_glyphs(font_size, &text, &font);
					let origin_x = axis_origin_pixel.0 - (i * x_subdivision_length);
					// So that scale markers are not drawn on the graph area itself check which quadrant type
					// and flip if necessary so they are drawn in the available whitespace outside the axis
					let origin_y = if *quadrants == Quadrants::BottomPair {
						axis_origin_pixel.1
							- (data_label_length * label_length_scale)
					} else {
						axis_origin_pixel.1 + (data_label_length * label_length_scale)
					};
					// let origin_y = origin_pixel.1 + (data_label_length);
					let offset = get_x_axis_scale_label_offset(&glyphs, origin_x, origin_y, quadrants);
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
									let px = axis_origin_pixel.0
										- ((i * x_subdivision_length) + (j * marker_spacing));
									// ensure mrkers are drawn in whitespace
									let py = if *quadrants == Quadrants::BottomPair {
										axis_origin_pixel.1 - n
									} else {
										axis_origin_pixel.1 + n
									};
									canvas.put_pixel(px, py, Rgba(BLACK));
								}
							}
							break 'outer_neg;
						}
					}
				}
			}
		}
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
					// So that scale markers are not drawn on the graph area itself check which quadrant type
					// and flip if necessary so they are drawn in the available whitespace outside the axis
					let px = axis_min_pixel.0 + (i * x_subdivision_length);
					let py = if *quadrants == Quadrants::BottomRight {
						axis_origin_pixel.1 - n
					} else {
						axis_origin_pixel.1 + n
					};
					canvas.put_pixel(px, py, Rgba(BLACK));
				}
				// Draw the data label text
				let text = (x_data_min_max_limits.0 as f32 + (x_value_per_subdivision * i as f32))
					.to_string();
				let glyphs = create_glyphs(font_size, &text, &font);
				let origin_x = axis_min_pixel.0 + (i * x_subdivision_length);
				// So that scale markers are not drawn on the graph area itself check which quadrant type
				// and flip if necessary so they are drawn in the available whitespace outside the axis
				let origin_y = if *quadrants == Quadrants::BottomRight {
					axis_origin_pixel.1
						- (data_label_length * label_length_scale)
				} else {
					axis_origin_pixel.1 + (data_label_length * label_length_scale)
				};
				// let origin_y = origin_pixel.1 + (data_label_length);
				let offset = get_x_axis_scale_label_offset(&glyphs, origin_x, origin_y, quadrants);
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
									// So that scale markers are not drawn on the graph area itself check which quadrant type
									// and flip if necessary so they are drawn in the available whitespace outside the axis
									let px = axis_min_pixel.0
										+ ((i * x_subdivision_length) + (j * marker_spacing));
									let py = if *quadrants == Quadrants::BottomRight {
										axis_origin_pixel.1 - n
									} else {
										axis_origin_pixel.1 + n
									};
									canvas.put_pixel(px, py, Rgba(BLACK));
								}
							}
							break 'outer;
						}
					}
				}
			}
		}
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
					// So that scale markers are not drawn on the graph area itself check which quadrant type
					// and flip if necessary so they are drawn in the available whitespace outside the axis
					let px = axis_origin_pixel.0 - (i * x_subdivision_length);
					let py = if *quadrants == Quadrants::BottomLeft {
						axis_origin_pixel.1 - n
					} else {
						axis_origin_pixel.1 + n
					};
					canvas.put_pixel(
						px,
						py,
						Rgba(BLACK),
					);
				}
				// Draw the data label text
				let text = (-x_value_per_subdivision * i as f32).to_string();
				let glyphs = create_glyphs(font_size, &text, &font);
				let origin_x = axis_origin_pixel.0 - (i * x_subdivision_length);
				// So that scale markers are not drawn on the graph area itself check which quadrant type
				// and flip if necessary so they are drawn in the available whitespace outside the axis
				let origin_y = if *quadrants == Quadrants::BottomLeft {
					axis_origin_pixel.1 - (data_label_length * label_length_scale)
				} else {
					axis_origin_pixel.1 + (data_label_length * label_length_scale)
				};
				// let origin_y = origin_pixel.1 + (data_label_length);
				let offset = get_x_axis_scale_label_offset(&glyphs, origin_x, origin_y, quadrants);
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
									// So that scale markers are not drawn on the graph area itself check which quadrant type
									// and flip if necessary so they are drawn in the available whitespace outside the axis
									let px = axis_origin_pixel.0 - ((i * x_subdivision_length) + (j * marker_spacing));
									let py = if *quadrants == Quadrants::BottomLeft {
										axis_origin_pixel.1 - n
									} else {
										axis_origin_pixel.1 + n
									};
									canvas.put_pixel(
										px,
										py,
										Rgba(BLACK),
									);
								}
							}
							break 'outer_l_neg;
						}
					}
				}
			}
		}
	}
}

/// Using glyph sizes calculate by how much the axis data label should be offset from an origin point
fn get_x_axis_scale_label_offset(
	glyphs: &Vec<PositionedGlyph>,
	origin_x: u32,
	origin_y: u32,
	quadrants: &Quadrants,
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
	let vertical_postion = match quadrants {
		Quadrants::BottomPair => origin_y - height,
		Quadrants::BottomRight => origin_y - height,
		Quadrants::BottomLeft => origin_y - height,
		_ => origin_y + height,
	};
	trace!("X-axis data label vertical offset: {}", vertical_postion);
	return (horizontal_position, vertical_postion);
}
