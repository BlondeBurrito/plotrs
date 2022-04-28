//! Based on a type of BestFit this module will calculate the valid data points for the given axes/canvas size

use std::collections::HashMap;

use serde::Deserialize;
use tracing::{error, trace};

use crate::{
	canvas::plot::{DataPoint, DataSymbol},
	colours::Colour,
};

/// Types of curve that can be fitted to a graph
#[derive(Debug, Deserialize, Clone)]
pub enum BestFit {
	/// Equation of a straight line, `y = mx + c`
	Linear {
		/// Incline of the line
		gradient: f32,
		/// The point of y-axis interception
		y_intercept: f32,
		/// The colour of the best fit curve
		colour: Colour,
	},
	/// Equation of form `y = a + bx + cx^2`
	Quadratic {
		/// Point of interception when `x = 0`
		intercept: f32,
		/// Coefficient of the respective base
		linear_coeff: f32,
		/// Coefficient of the respective base
		quadratic_coeff: f32,
		/// The colour of the best fit curve
		colour: Colour,
	},
	/// Equation of form `y = a + bx + cx^2 + dx^3`
	Cubic {
		/// Point of interception when `x = 0`
		intercept: f32,
		/// Coefficient of the respective base
		linear_coeff: f32,
		/// Coefficient of the respective base
		quadratic_coeff: f32,
		/// Coefficient of the respective base
		cubic_coeff: f32,
		/// The colour of the best fit curve
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
	///
	/// For instance a Quartic (4th power) polynomial could be represented in `.ron` as
	/// `Some(GenericPolynomial(coefficients: {0: 1.0, 1: 1.0, 2: 1.0, 3: 1.0, 4: -1.0}, colour: Black))`
	GenericPolynomial {
		/// Keys are powers `x` will be raised by and values are the coefficient
		coefficients: HashMap<u32, f32>,
		/// The colour of the best fit curve
		colour: Colour,
	},
	/// Equation of form `y = an^(bx) + c`
	Exponential {
		/// Coefficient/amplitude/size of the exponential
		constant: f32,
		/// The base
		base: f32,
		/// Value exponent is multiplied by, effectively the power the exponential is raised by
		power: f32,
		/// Vertical offset from the origin
		vertical_shift: f32,
		/// The colour of the best fit curve
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
		/// Max size of a periodic quantity
		amplitude: f32,
		/// Factor indicating the amount of time to oscillaite through one period
		period: f32,
		/// Angle-like quantity to modify a cycle by
		phase_shift: f32,
		/// Vertical offset from the origin
		vertical_shift: f32,
		/// The colour of the best fit curve
		colour: Colour,
	},
	/// Equation of form `y = a * cos(bx + c) + d`
	///
	/// `y = amplitude * cos( period * x + phase_shift) + vertical_shift`
	Cosine {
		/// Max size of a periodic quantity
		amplitude: f32,
		/// Factor indicating the amount of time to oscillaite through one period
		period: f32,
		/// Angle-like quantity to modify a cycle by
		phase_shift: f32,
		/// Vertical offset from the origin
		vertical_shift: f32,
		/// The colour of the best fit curve
		colour: Colour,
	},
}

impl BestFit {
	/// Based on the type of `BestFit` curve generate its coordinates within the given bounds and a scale factor is used to create a seamless curve, i.e a large number of tightly knit points to create the illusion of a line
	pub fn find_coordinates(
		&self,
		x_min: i32,
		x_max: i32,
		y_min: i32,
		y_max: i32,
		scale_factor: i32,
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
							x,
							ux: None,
							y,
							uy: None,
							colour: *colour,
							symbol: DataSymbol::Point,
							symbol_radius: 1,
							symbol_thickness: 1,
						});
					}
				}
				points
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
							x,
							ux: None,
							y,
							uy: None,
							colour: *colour,
							symbol: DataSymbol::Point,
							symbol_radius: 1,
							symbol_thickness: 1,
						});
					}
				}
				points
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
							x,
							ux: None,
							y,
							uy: None,
							colour: *colour,
							symbol: DataSymbol::Point,
							symbol_radius: 1,
							symbol_thickness: 1,
						});
					}
				}
				points
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
							x,
							ux: None,
							y,
							uy: None,
							colour: *colour,
							symbol: DataSymbol::Point,
							symbol_radius: 1,
							symbol_thickness: 1,
						});
					}
				}
				points
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
							x,
							ux: None,
							y,
							uy: None,
							colour: *colour,
							symbol: DataSymbol::Point,
							symbol_radius: 1,
							symbol_thickness: 1,
						});
					}
				}
				points
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
							x,
							ux: None,
							y,
							uy: None,
							colour: *colour,
							symbol: DataSymbol::Point,
							symbol_radius: 1,
							symbol_thickness: 1,
						});
					}
				}
				points
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
							x,
							ux: None,
							y,
							uy: None,
							colour: *colour,
							symbol: DataSymbol::Point,
							symbol_radius: 1,
							symbol_thickness: 1,
						});
					}
				}
				points
			}
		}
	}
}
