//!

use std::collections::HashMap;

use serde::Deserialize;
use tracing::{error, trace};

use crate::{
	canvas::plot::{DataPoint, DataSymbol},
	colours::Colour,
};

/// Types of curve that cna be fitted to a graph
#[derive(Debug, Deserialize, Clone)]
pub enum BestFit {
	/// Equation of a straight line, `y = mx + c`
	Linear {
		gradient: f32,
		y_intercept: f32,
		colour: Colour,
	},
	/// Equation of form `y = a + bx + cx^2`
	Quadratic {
		intercept: f32,
		linear_coeff: f32,
		quadratic_coeff: f32,
		colour: Colour,
	},
	/// Equation of form `y = a + bx + cx^2 + dx^3`
	Cubic {
		intercept: f32,
		linear_coeff: f32,
		quadratic_coeff: f32,
		cubic_coeff: f32,
		colour: Colour,
	},
	/// Equation of form `y = a + bx + cx^2 + dx^3....n`
	///
	/// Each `HashMap<u32, f32>` key corresponds to an `nth` order power while the value is the coefficient.
	/// ```txt
	/// let mut y = 0.0;
	/// for (k, v) in coefficients.iter() {
	///     y += v * x.powf(k as f32);
	/// }
	/// ```
	GenericPolynomial {
		coefficients: HashMap<u32, f32>,
		colour: Colour,
	},
	/// Equation of form `y = an^(bx) + c`
	Exponential {
		constant: f32,
		base: f32,
		power: f32,
		vertical_shift: f32,
		colour: Colour,
	},
	// /// Equation of form `y = a(1 - n^(-bx)) + c`
	// ExponentialApproach {
	// 	constant: f32,
	// 	base: f32,
	// 	power: f32,
	// 	vertical_shift: f32,
	// 	colour: Colour,
	// },
	/// Equation of form `y = a * sin(bx + c) + d`
	///
	/// `y = amplitude * sin( period * x + phase_shift) + vertical_shift`
	Sine {
		amplitude: f32,
		period: f32,
		phase_shift: f32,
		vertical_shift: f32,
		colour: Colour,
	},
	/// Equation of form `y = a * cos(bx + c) + d`
	///
	/// `y = amplitude * cos( period * x + phase_shift) + vertical_shift`
	Cosine {
		amplitude: f32,
		period: f32,
		phase_shift: f32,
		vertical_shift: f32,
		colour: Colour,
	},
}

impl BestFit {
	/// Based of type of `BestFit` curve generate its coordinates within the given bounds
	pub fn find_coordinates(
		&self,
		x_min: u32,
		x_max: u32,
		y_min: u32,
		y_max: u32,
		scale_factor: u32,
	) -> Vec<DataPoint> {
		match self {
			BestFit::Linear {
				gradient,
				y_intercept,
				colour,
			} => {
				trace!("Finding coordinates for Linear best fit line with gradient {}, y_intercept {} and between ({}, {}) and ({}, {})", gradient, y_intercept, x_min, y_min, x_max, y_max);
				let mut points: Vec<DataPoint> = Vec::new();
				for scaled_x in x_min..=(x_max * scale_factor) {
					let x = scaled_x as f32 / scale_factor as f32;
					let y = (*gradient * x) + *y_intercept;
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
			BestFit::Quadratic {
				intercept,
				linear_coeff,
				quadratic_coeff,
				colour,
			} => {
				trace!("Finding coordinates for Quadratic best fit line with intercept {}, linear coefficient {} and quadratic coefficient {}", intercept, linear_coeff, quadratic_coeff);
				let mut points: Vec<DataPoint> = Vec::new();
				for scaled_x in x_min..=(x_max * scale_factor) {
					let x = scaled_x as f32 / scale_factor as f32;
					let y = intercept + (linear_coeff * x) + (quadratic_coeff * x.powf(2.0));
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
			BestFit::Cubic {
				intercept,
				linear_coeff,
				quadratic_coeff,
				cubic_coeff,
				colour,
			} => {
				trace!("Finding coordinates for Cubic best fit line with intercept {}, linear coefficient {}, quadratic coefficient {} and cubic coefficient {}", intercept, linear_coeff, quadratic_coeff, cubic_coeff);
				let mut points: Vec<DataPoint> = Vec::new();
				for scaled_x in x_min..=(x_max * scale_factor) {
					let x = scaled_x as f32 / scale_factor as f32;
					let y = intercept
						+ (linear_coeff * x) + (quadratic_coeff * x.powf(2.0))
						+ (cubic_coeff * x.powf(3.0));
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
			BestFit::GenericPolynomial {
				coefficients,
				colour,
			} => {
				trace!("Finding coordinates for GenericPolynomial best fit line");
				let mut points: Vec<DataPoint> = Vec::new();
				for scaled_x in x_min..=(x_max * scale_factor) {
					let x = scaled_x as f32 / scale_factor as f32;
					let mut y = 0.0;
					for (k, v) in coefficients.iter() {
						y += v * x.powf(*k as f32);
					}
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
			BestFit::Exponential {
				constant,
				base,
				power,
				vertical_shift,
				colour,
			} => {
				trace!("Finding coordinates for Exponential best fit line with constant {}, base {}, power {} and vertica shift {}", constant, base, power, vertical_shift);
				if *base <= 0.0 {
					error!("The base used in an exponential best fit must be greater than zero, you specified {}", base);
					std::process::exit(1);
				}
				let mut points: Vec<DataPoint> = Vec::new();
				for scaled_x in x_min..=(x_max * scale_factor) {
					let x = scaled_x as f32 / scale_factor as f32;
					let y = (constant * base.powf(power * x)) + vertical_shift;
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
			// BestFit::ExponentialApproach { constant, base, power, vertical_shift, colour } => {
			// 	trace!("Finding coordinates for ExponentialApproach best fit line with constant {}, base {}, power {} and vertica shift {}", constant, base, power, vertical_shift);
			// 	if *base <= 0.0 {
			// 		error!("The base used in an exponential best fit must be greater than zero, you specified {}", base);
			// 		std::process::exit(1);
			// 	}
			// 	let mut points: Vec<DataPoint> = Vec::new();
			// 	for scaled_x in x_min..=(x_max * scale_factor) {
			// 		let x = scaled_x as f32 / scale_factor as f32;
			// 		let y = constant * (1.0 - base.powf(-power * x)) + vertical_shift;
			// 		if y > y_min as f32 && y < y_max as f32 {
			// 			points.push(DataPoint {
			// 				x: x,
			// 				ux: None,
			// 				y: y,
			// 				uy: None,
			// 				colour: *colour,
			// 				symbol: DataSymbol::Point,
			// 				symbol_radius: 1,
			// 				symbol_thickness: 1,
			// 			});
			// 		}
			// 	}
			// 	return points
			// },
			BestFit::Sine {
				amplitude,
				period,
				phase_shift,
				vertical_shift,
				colour,
			} => {
				trace!("Finding coordinates for Sinusoidal best fit line with amplitude {}, period {}, phase shift {} and vertical shift {}", amplitude, period, phase_shift, vertical_shift);
				let mut points: Vec<DataPoint> = Vec::new();
				for scaled_x in x_min..=(x_max * scale_factor) {
					let x = scaled_x as f32 / scale_factor as f32;
					let y = amplitude * ((period * x) + phase_shift).sin() + vertical_shift;
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
			BestFit::Cosine {
				amplitude,
				period,
				phase_shift,
				vertical_shift,
				colour,
			} => {
				trace!("Finding coordinates for Cosinusoidal best fit line with amplitude {}, period {}, phase shift {} and vertical shift {}", amplitude, period, phase_shift, vertical_shift);
				let mut points: Vec<DataPoint> = Vec::new();
				for scaled_x in x_min..=(x_max * scale_factor) {
					let x = scaled_x as f32 / scale_factor as f32;
					let y = amplitude * ((period * x) + phase_shift).cos() + vertical_shift;
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
		}
	}
}
