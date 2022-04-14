//! Shorthand constants for common colours

use serde::Deserialize;

pub const WHITE: [u8; 4] = [255, 255, 255, 255];
pub const BLACK: [u8; 4] = [0, 0, 0, 255];
pub const ORANGE: [u8; 4] = [255, 146, 0, 255];
pub const RED: [u8; 4] = [255, 0, 0, 255];
pub const BLUE: [u8; 4] = [0, 255, 0, 255];
pub const GREEN: [u8; 4] = [0, 0, 255, 255];
pub const PINK: [u8; 4] = [255, 169, 208, 255];

/// Colours that can be used to plot data points
#[derive(Debug, Deserialize, Copy, Clone)]
pub enum Colour {
	Orange,
	Red,
	Blue,
	Green,
	Pink,
}

impl Colour {
	/// Get the colour that should be used when drawing data points
	pub fn get_pixel_colour(colour: Colour) -> [u8; 4] {
		match colour {
			Colour::Orange => ORANGE,
			Colour::Red => RED,
			Colour::Blue => BLUE,
			Colour::Green => GREEN,
			Colour::Pink => PINK,
		}
	}
}
