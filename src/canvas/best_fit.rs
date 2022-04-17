//!

use serde::Deserialize;
use tracing::trace;

use crate::{
	canvas::plot::{DataPoint, DataSymbol},
	colours::Colour,
};

/// Types of curve that cna be fitted to a graph
#[derive(Debug, Deserialize, Copy, Clone)]
pub enum BestFit {
	/// Equation of a straight line, `y = mx + c`
	Linear {
		gradient: f32,
		y_intercept: f32,
		colour: Colour,
	},
	Abc,
}

impl BestFit {
	/// Based of type of `BestFit` curve generate its coordinates within the given bounds
	pub fn find_coordinates(&self, x_min: u32, x_max: u32, y_min: u32, y_max: u32) -> Vec<DataPoint> {
		match self {
			BestFit::Linear {
				gradient,
				y_intercept,
				colour,
			} => {
				trace!("Finding coordinates for Linear best fit line with gradient {}, y_intercept {} and between ({}, {}) and ({}, {})", gradient, y_intercept, x_min, y_min, x_max, y_max);
				let mut points: Vec<DataPoint> = Vec::new();
				let scale_factor = 1000;
				for scaled_x in x_min..=(x_max * scale_factor) {
					let x = scaled_x as f32 / scale_factor as f32;
					let y = (*gradient * x ) + *y_intercept;
					if y > y_min as f32 && y < y_max as f32 {
						points.push(DataPoint {
							x: x,
							ux: None,
							y: y,
							uy: None,
							colour: *colour,
							symbol: DataSymbol::Point,
							symbol_radius: 1,
							symbol_thickness: 1,
						});
					}
				}
				return points;
			}
    BestFit::Abc => todo!(),
		}
	}
}
