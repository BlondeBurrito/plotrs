//! Shorthand constants for common colours

use serde::Deserialize;
/// Rgba White
pub const WHITE: [u8; 4] = [255, 255, 255, 255];
/// Rgba Black
pub const BLACK: [u8; 4] = [0, 0, 0, 255];
/// Rgba Grey
pub const GREY: [u8; 4] = [161, 161, 161, 255];
/// Rgba Orange
pub const ORANGE: [u8; 4] = [255, 146, 0, 255];
/// Rgba Red
pub const RED: [u8; 4] = [255, 0, 0, 255];
/// Rgba Blue
pub const BLUE: [u8; 4] = [0, 0, 255, 255];
/// Rgba Green
pub const GREEN: [u8; 4] = [0, 255, 0, 255];
/// Rgba Pink
pub const PINK: [u8; 4] = [255, 169, 208, 255];

/// Colours that can be used to plot data points
#[derive(Debug, Deserialize, Copy, Clone)]
#[allow(clippy::missing_docs_in_private_items)]
pub enum Colour {
	White,
	Black,
	Grey,
	Orange,
	Red,
	Blue,
	Green,
	Pink,
}

impl Colour {
	/// Get the `u8` representation of a colour that should be used when drawing data points
	pub fn get_pixel_colour(colour: Colour) -> [u8; 4] {
		match colour {
			Colour::White => WHITE,
			Colour::Black => BLACK,
			Colour::Grey => GREY,
			Colour::Orange => ORANGE,
			Colour::Red => RED,
			Colour::Blue => BLUE,
			Colour::Green => GREEN,
			Colour::Pink => PINK,
		}
	}
}
