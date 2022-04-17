//!

use serde::Deserialize;
use tracing::trace;

use crate::{colours::Colour, canvas::plot::{DataPoint, DataSymbol}};

// pub trait BestFit {
// 	/// Finds the x-y data points between the min and max bounds
// 	fn find_coordinates(&self, x_min: u32, x_max: u32, y_min: u32, y_max: u32) -> Vec<DataPoint>;
// }
// /// Components of the equation of a straight line, `y = mx + c`
// #[derive(Debug, Deserialize)]
// pub struct Linear {
// 	gradient: f32,
// 	y_intercept: f32,
// 	// colour: Colour,
// }

#[derive(Debug, Deserialize)]
pub enum BestFit {
	Linear {gradient: f32, y_intercept: f32, colour: Colour}
}

impl BestFit {
    fn find_coordinates(&self, x_min: u32, x_max: u32, y_min: u32, y_max: u32) -> Vec<DataPoint> {
		match self {
			BestFit::Linear { gradient, y_intercept, colour } => {
				trace!("Finding coordinates for Linear best fit line with gradient {}, y_intercept {} and between ({}, {}) and ({}, {})", gradient, y_intercept, x_min, y_min, x_max, y_max);
				let mut points: Vec<DataPoint> = Vec::new();
				for x in x_min..=x_max {
					let y = (gradient * x as f32) + y_intercept;
					if y > y_min as f32 && y < y_max as f32 {
						points.push(DataPoint {
							x: x as f32,
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
			return points
			},
		}
	}
}