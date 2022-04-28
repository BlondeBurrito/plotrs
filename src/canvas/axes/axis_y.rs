//! Draws the y-axis with labels and scale markers

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
		Quadrants::RightPair | Quadrants::TopRight => {
			debug!("Placing y-axis label in top left corner");
			let position: (u32, u32) = (CANVAS_BORDER_PIXELS, vertical_pixels_from_top);
			draw_glyphs(canvas, BLACK, glyphs, position);
			VHConsumedCanvasSpace {
				v_space_from_top: height + CANVAS_BORDER_PIXELS,
				h_space_from_left: width + CANVAS_BORDER_PIXELS,
				v_space_from_bottom: 0,
				h_space_from_right: 0,
			}
		}
		Quadrants::LeftPair | Quadrants::TopLeft => {
			debug!("Placing y-axis label in top right corner");
			let position: (u32, u32) = (
				canvas.dimensions().0 - width - CANVAS_BORDER_PIXELS - horizontal_pixels_from_right,
				vertical_pixels_from_top,
			);
			draw_glyphs(canvas, BLACK, glyphs, position);
			VHConsumedCanvasSpace {
				v_space_from_top: height + CANVAS_BORDER_PIXELS,
				h_space_from_left: 0,
				v_space_from_bottom: 0,
				h_space_from_right: width + CANVAS_BORDER_PIXELS,
			}
		}
		Quadrants::TopPair | Quadrants::AllQuadrants => {
			debug!("Placing y-axis label in middle top area");
			let position: (u32, u32) = (
				(canvas.dimensions().0 / 2) - (width / 2),
				vertical_pixels_from_top,
			);
			draw_glyphs(canvas, BLACK, glyphs, position);
			VHConsumedCanvasSpace {
				v_space_from_top: height + CANVAS_BORDER_PIXELS,
				h_space_from_left: 0,
				v_space_from_bottom: CANVAS_BORDER_PIXELS,
				h_space_from_right: 0,
			}
		}
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
		}
		Quadrants::BottomRight => {
			debug!("Placing y-axis label in bottom left corner");
			let position: (u32, u32) = (
				horizontal_pixels_from_left + width + CANVAS_BORDER_PIXELS,
				canvas.dimensions().1 - vertical_pixels_from_bottom - height,
			);
			draw_glyphs(canvas, BLACK, glyphs, position);
			VHConsumedCanvasSpace {
				v_space_from_top: 0,
				h_space_from_left: width + CANVAS_BORDER_PIXELS,
				v_space_from_bottom: height + CANVAS_BORDER_PIXELS,
				h_space_from_right: 0,
			}
		}
		Quadrants::BottomLeft => {
			debug!("Placing y-axis label in bottom right corner");
			let position: (u32, u32) = (
				canvas.dimensions().0 - horizontal_pixels_from_right - width,
				canvas.dimensions().1 - vertical_pixels_from_bottom - height,
			);
			draw_glyphs(canvas, BLACK, glyphs, position);
			VHConsumedCanvasSpace {
				v_space_from_top: 0,
				h_space_from_left: 0,
				v_space_from_bottom: height + CANVAS_BORDER_PIXELS,
				h_space_from_right: width + CANVAS_BORDER_PIXELS,
			}
		}
	}
}
/// Get the pixel length of the y-axis
pub fn get_y_axis_pixel_length(min_pixel: u32, max_pixel: u32) -> u32 {
	let length = max_pixel.overflowing_sub(min_pixel);
	if length.1 {
		panic!("Y-axis length overflow!");
	}
	length.0
}
/// Draws the y-axis with s static `x` position taken from `axis_origin_pixel`
pub fn draw_y_axis(
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
pub fn draw_y_axis_scale_markings(
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
			let subdivision_length = (y_axis_length / 2) / y_axis_resolution;
			// The pixel length of each data label
			let data_label_length = 5;
			// For writing a value for each data label we need to know the overall data size that corrpesonds to the axis
			let value_range = y_data_min_max_limits.1 as f32 - y_data_min_max_limits.0 as f32;
			// Find how much a suddivsion is in terms of data value
			let value_per_subdivision = (value_range / 2.0) / y_axis_resolution as f32;
			// If required draw the y part of a background grid as grey vertical lines
			if has_grid {
				trace!("Drawing grey background grid...");
				// draw lines in positive space
				for i in 1..(y_axis_resolution + 1) {
					for px in axis_min_pixel.0..axis_max_pixel.0 {
						canvas.put_pixel(
							px,
							axis_origin_pixel.1 - (i * subdivision_length),
							Rgba(GREY),
						);
					}
				}
				// draw lines in negative space
				for i in 1..(y_axis_resolution + 1) {
					for px in axis_min_pixel.0..axis_max_pixel.0 {
						canvas.put_pixel(
							px,
							axis_origin_pixel.1 + (i * subdivision_length),
							Rgba(GREY),
						);
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
					// So that scale markers are not drawn on the graph area itself check which quadrant type
					// and flip if necessary so they are drawn in the available whitespace outside the axis
					let px = if *quadrants == Quadrants::LeftPair {
						axis_origin_pixel.0 + n
					} else {
						axis_origin_pixel.0 - n
					};
					let py = axis_origin_pixel.1 - (i * subdivision_length);
					canvas.put_pixel(px, py, Rgba(BLACK));
				}
				// Draw the data label text
				// Don't draw text directly over origin ortherwise it obscures axis
				if i != 0 {
					let text = (value_per_subdivision * i as f32).to_string();
					let glyphs = create_glyphs(font_size, &text, &font);
					let origin_x = axis_origin_pixel.0 - (data_label_length * label_length_scale);
					let origin_y = axis_origin_pixel.1 - (i * subdivision_length);
					let offset =
						get_y_axis_scale_label_offset(&glyphs, origin_x, origin_y, quadrants);
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
									// So that scale markers are not drawn on the graph area itself check which quadrant type
									// and flip if necessary so they are drawn in the available whitespace outside the axis
									let px = if *quadrants == Quadrants::LeftPair {
										axis_origin_pixel.0 + n
									} else {
										axis_origin_pixel.0 - n
									};
									let py = axis_origin_pixel.1
										- ((i * subdivision_length) + (j * marker_spacing));
									canvas.put_pixel(px, py, Rgba(BLACK));
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
					// So that scale markers are not drawn on the graph area itself check which quadrant type
					// and flip if necessary so they are drawn in the available whitespace outside the axis
					let px = if *quadrants == Quadrants::LeftPair {
						axis_origin_pixel.0 + n
					} else {
						axis_origin_pixel.0 - n
					};
					let py = axis_origin_pixel.1 + (i * subdivision_length);
					canvas.put_pixel(px, py, Rgba(BLACK));
				}
				// Draw the data label text
				// Don't draw text directly over origin ortherwise it obscures axis
				if i != 0 {
					let text = (-value_per_subdivision * i as f32).to_string();
					let glyphs = create_glyphs(font_size, &text, &font);
					let origin_x = axis_origin_pixel.0 - (data_label_length * label_length_scale);
					let origin_y = axis_origin_pixel.1 + (i * subdivision_length);
					let offset =
						get_y_axis_scale_label_offset(&glyphs, origin_x, origin_y, quadrants);
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
									// So that scale markers are not drawn on the graph area itself check which quadrant type
									// and flip if necessary so they are drawn in the available whitespace outside the axis
									let px = if *quadrants == Quadrants::LeftPair {
										axis_origin_pixel.0 + n
									} else {
										axis_origin_pixel.0 - n
									};
									let py = axis_origin_pixel.1
										+ ((i * subdivision_length) + (j * marker_spacing));
									canvas.put_pixel(px, py, Rgba(BLACK));
								}
							}
							break 'outer_neg;
						}
					}
				}
			}
		}
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
						canvas.put_pixel(
							px,
							axis_min_pixel.1 - (i * subdivision_length),
							Rgba(GREY),
						);
					}
				}
			}
			// Draw a line of pixels down from the axis as each subdivisions
			trace!("Marking each y-axis subdivision...");
			for i in 0..(y_axis_resolution + 1) {
				// Draw each even section slightly longer
				let label_length_scale = if i & 1 == 1 { 2 } else { 3 };
				for n in 0..(data_label_length * label_length_scale) {
					// So that scale markers are not drawn on the graph area itself check which quadrant type
					// and flip if necessary so they are drawn in the available whitespace outside the axis
					let px = if *quadrants == Quadrants::TopLeft {
						axis_origin_pixel.0 + n
					} else {
						axis_origin_pixel.0 - n
					};
					let py = axis_min_pixel.1 - (i * subdivision_length);
					canvas.put_pixel(px, py, Rgba(BLACK));
				}
				// Draw the data label text
				let text = (y_data_min_max_limits.0 as f32 + (value_per_subdivision * i as f32))
					.to_string();
				let glyphs = create_glyphs(font_size, &text, &font);
				// So that scale markers are not drawn on the graph area itself check which quadrant type
				// and flip if necessary so they are drawn in the available whitespace outside the axis
				let origin_x = if *quadrants == Quadrants::TopLeft {
					axis_origin_pixel.0 + (data_label_length * label_length_scale)
				} else {
					axis_origin_pixel.0 - (data_label_length * label_length_scale)
				};
				let origin_y = axis_min_pixel.1 - (i * subdivision_length);
				let offset = get_y_axis_scale_label_offset(&glyphs, origin_x, origin_y, quadrants);
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
									// So that scale markers are not drawn on the graph area itself check which quadrant type
									// and flip if necessary so they are drawn in the available whitespace outside the axis
									let px = if *quadrants == Quadrants::TopLeft {
										axis_origin_pixel.0 + n
									} else {
										axis_origin_pixel.0 - n
									};
									let py = axis_min_pixel.1
										- ((i * subdivision_length) + (j * marker_spacing));
									canvas.put_pixel(px, py, Rgba(BLACK));
								}
							}
							break 'outer;
						}
					}
				}
			}
		}
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
						canvas.put_pixel(
							px,
							axis_origin_pixel.1 + (i * subdivision_length),
							Rgba(GREY),
						);
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
					// So that scale markers are not drawn on the graph area itself check which quadrant type
					// and flip if necessary so they are drawn in the available whitespace outside the axis
					let px = if *quadrants == Quadrants::BottomLeft {
						axis_origin_pixel.0 + n
					} else {
						axis_origin_pixel.0 - n
					};
					let py = axis_origin_pixel.1 + (i * subdivision_length);
					canvas.put_pixel(px, py, Rgba(BLACK));
				}
				// Draw the data label text
				// Don't draw text directly over origin ortherwise it obscures axis
				if i != 0 {
					let text = (-value_per_subdivision * i as f32).to_string();
					let glyphs = create_glyphs(font_size, &text, &font);
					// So that scale markers are not drawn on the graph area itself check which quadrant type
					// and flip if necessary so they are drawn in the available whitespace outside the axis
					let origin_x = if *quadrants == Quadrants::BottomLeft {
						axis_origin_pixel.0 + (data_label_length * label_length_scale)
					} else {
						axis_origin_pixel.0 - (data_label_length * label_length_scale)
					};
					let origin_y = axis_origin_pixel.1 + (i * subdivision_length);
					let offset =
						get_y_axis_scale_label_offset(&glyphs, origin_x, origin_y, quadrants);
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
									// So that scale markers are not drawn on the graph area itself check which quadrant type
									// and flip if necessary so they are drawn in the available whitespace outside the axis
									let px = if *quadrants == Quadrants::BottomLeft {
										axis_origin_pixel.0 + n
									} else {
										axis_origin_pixel.0 - n
									};
									let py = axis_origin_pixel.1
										+ ((i * subdivision_length) + (j * marker_spacing));
									canvas.put_pixel(px, py, Rgba(BLACK));
								}
							}
							break 'outer_b_neg;
						}
					}
				}
			}
		}
	}
}

/// Using glyph sizes calculate by how much the axis data label should be offset from an origin point
fn get_y_axis_scale_label_offset(
	glyphs: &Vec<PositionedGlyph>,
	origin_x: u32,
	origin_y: u32,
	quadrants: &Quadrants,
) -> (u32, u32) {
	let width = get_width_of_glyphs(glyphs);
	let height = get_maximum_height_of_glyphs(glyphs);
	trace!("Y-axis data label width: {}", width);
	trace!("Y-axis data label height: {}", height);
	let horizontal_position = match quadrants {
		Quadrants::TopLeft => origin_x + width,
		Quadrants::BottomLeft => origin_x + width,
		Quadrants::LeftPair => origin_x + width,
		_ => origin_x - width,
	};
	trace!(
		"Y-axis data label horizontal offset: {}",
		horizontal_position
	);
	let vertical_postion = origin_y - (height / 2);
	trace!("Y-axis data label vertical offset: {}", vertical_postion);
	return (horizontal_position, vertical_postion);
}
