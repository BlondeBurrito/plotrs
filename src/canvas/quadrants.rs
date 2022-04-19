//!

#[derive(PartialEq, Debug)]
pub enum Quadrants {
	/// Top right and bottom right quadrants, x is only positive, y is pos and neg
	RightPair,
	/// Top left and bottom left quadrants, x is only negative, y is pos and neg
	LeftPair,
	/// Top left and top right quadrants, x is pos and neg, y is onl positive
	TopPair,
	/// Bottom left and bottom right quadrants, x is neg and pos, y is onl negative
	BottomPair,
	/// Top left, top right, bottom left, bottom right quadrants, x is pos and neg, y is pos and neg
	AllQuadrants,
	/// Top right quadrant, x is positive, y is positive
	TopRight,
	/// Top left quadrant, x is negative, y is positive
	TopLeft,
	/// Bottom right quadrant, x is positive, y is negative
	BottomRight,
	/// Bottom left quadrant, x is negative, y is negative
	BottomLeft
}
/// Based on the minimum and maximum possible values in the data sets identify which quadrants need to be drawn
pub fn get_quadrants(min_xy: (i32, i32), max_xy: (i32, i32)) -> Quadrants {
	match min_xy.0.signum() {
		// x minimum is positive there x maximum is positive
		1 => {
			match min_xy.1.signum() {
				// y minimum is positive
				1 => {Quadrants::TopRight},
				// y minimum is zero
				0 => {Quadrants::TopRight},
				// y minimum is negative therefore bottom right or right pair
				-1 => {
					match max_xy.1.signum() {
						1 => {Quadrants::RightPair},
						0 => {Quadrants::BottomRight},
						-1 => {Quadrants::BottomRight},
						_ => {panic!("Invalid `signum()`?!")},
					}
				},
				_ => {panic!("Invalid `signum()`?!")},
			}
		},
		// x minimum is zero therefore x maximum is positive
		0 => {
			match min_xy.1.signum() {
				// y minimum is positive
				1 => {Quadrants::TopRight},
				// y minimum is zero therefore y maximum could be positive or negative
				0 => {
					match max_xy.1.signum() {
						1 => {Quadrants::TopRight},
						0 => {panic!("Quadrant bounds: invalid data, x is positive while y is exactly zero")},
						-1 => {panic!("Quadrant bounds: invalid data, x is positive y min cannot be zero with positive y max ")},
						_ => {panic!("Invalid `signum()`?!")},
					}
				},
				// y minimum is negative therefore could be right pair or bottom right
				-1 => {
					match max_xy.1.signum() {
						1 => {Quadrants::RightPair},
						0 => {Quadrants::BottomRight},
						-1 => {Quadrants::BottomRight},
						_ => {panic!("Invalid `signum()`?!")},
					}
				},
				_ => {panic!("Invalid `signum()`?!")},
			}
		},
		// x minimum is negative
		-1 => {
			match max_xy.0.signum() {
				// x maximum is positive so could be all quadrants or top/bottom pair
				1 => {
					match min_xy.1.signum() {
						// y minimum is positive therefore must be top pair
						1 => {Quadrants::TopPair},
						// y minimum is zero therefore must be top pair
						0 => {Quadrants::TopPair},
						// y minimum is negative so could be bottom pair or all quadrants
						-1 => {
							match max_xy.1.signum() {
								1 => {Quadrants::AllQuadrants},
								0 => {Quadrants::BottomPair},
								-1 => {Quadrants::BottomPair},
								_ => {panic!("Invalid `signum()`?!")},
							}
						},
						_ => {panic!("Invalid `signum()`?!")},
					}
				},
				// x maximum is zero therefore could be top left, bottom left or left pair
				0 => {
					match min_xy.1.signum() {
						// y minimum is positive therefore can only be top left
						1 => {Quadrants::TopLeft},
						// y minimum is zero so max must be positive
						0 => {Quadrants::TopLeft},
						// y minimum is negative so y maximum must be zero or positive
						-1 => {
							match max_xy.1.signum() {
								// y maximum is positive therefore left pair
								1 => {Quadrants::LeftPair},
								// y maximum is zero therefore must be bottom left
								0 => {Quadrants::BottomLeft},
								-1 => {Quadrants::BottomLeft},
								_ => {panic!("Invalid `signum()`?!")},
							}
						},
						_ => {panic!("Invalid `signum()`?!")},
					}
				},
				// x min-max is negative so must be left top/bottom/pair
				-1 => {
					match min_xy.1.signum() {
						1 => {Quadrants::TopLeft},
						0 => {Quadrants::TopLeft},
						-1 => {
							match max_xy.1.signum() {
								1 => {Quadrants::LeftPair},
								0 => {Quadrants::BottomLeft},
								-1 => {Quadrants::BottomLeft},
								_ => {panic!("Invalid `signum()`?!")},
							}
						},
						_ => {panic!("Invalid `signum()`?!")},
					}
				},
				_ => {panic!("Invalid `signum()`?!")},
			}
		},
		_ => {panic!("Invalid `signum()`?!")},
	}
}

#[cfg(test)]
mod tests {
    use crate::canvas::quadrants::{get_quadrants, Quadrants};
    #[test]
    fn all_quadrants() {
		let min_xy: (i32, i32) = (-1, -1);
		let max_xy: (i32, i32) = (1, 1);
		let q = get_quadrants(min_xy, max_xy);
        assert_eq!(Quadrants::AllQuadrants, q);
    }
	#[test]
    fn top_pair_ensure_valid_zero() {
		let min_xy: (i32, i32) = (-1, 0);
		let max_xy: (i32, i32) = (1, 1);
		let q = get_quadrants(min_xy, max_xy);
        assert_eq!(Quadrants::TopPair, q);
    }
	#[test]
    fn top_pair() {
		let min_xy: (i32, i32) = (-1, 1);
		let max_xy: (i32, i32) = (1, 2);
		let q = get_quadrants(min_xy, max_xy);
        assert_eq!(Quadrants::TopPair, q);
    }
	#[test]
    fn bottom_pair_ensure_valid_zero() {
		let min_xy: (i32, i32) = (-1, -1);
		let max_xy: (i32, i32) = (1, 0);
		let q = get_quadrants(min_xy, max_xy);
        assert_eq!(Quadrants::BottomPair, q);
    }
	#[test]
    fn bottom_pair() {
		let min_xy: (i32, i32) = (-1, -1);
		let max_xy: (i32, i32) = (1, -2);
		let q = get_quadrants(min_xy, max_xy);
        assert_eq!(Quadrants::BottomPair, q);
    }
	#[test]
    fn left_pair_ensure_valid_zero() {
		let min_xy: (i32, i32) = (-1, -1);
		let max_xy: (i32, i32) = (0, 1);
		let q = get_quadrants(min_xy, max_xy);
        assert_eq!(Quadrants::LeftPair, q);
    }
	#[test]
    fn left_pair() {
		let min_xy: (i32, i32) = (-2, -1);
		let max_xy: (i32, i32) = (-1, 1);
		let q = get_quadrants(min_xy, max_xy);
        assert_eq!(Quadrants::LeftPair, q);
    }
	#[test]
    fn right_pair_ensure_valid_zero() {
		let min_xy: (i32, i32) = (0, -1);
		let max_xy: (i32, i32) = (1, 1);
		let q = get_quadrants(min_xy, max_xy);
        assert_eq!(Quadrants::RightPair, q);
    }
	#[test]
    fn right_pair() {
		let min_xy: (i32, i32) = (1, -1);
		let max_xy: (i32, i32) = (2, 1);
		let q = get_quadrants(min_xy, max_xy);
        assert_eq!(Quadrants::RightPair, q);
    }
	#[test]
    fn top_right_ensure_zero() {
		let min_xy: (i32, i32) = (0, 0);
		let max_xy: (i32, i32) = (1, 1);
		let q = get_quadrants(min_xy, max_xy);
        assert_eq!(Quadrants::TopRight, q);
    }
	#[test]
    fn top_right_ensure_zero2() {
		let min_xy: (i32, i32) = (1, 0);
		let max_xy: (i32, i32) = (2, 1);
		let q = get_quadrants(min_xy, max_xy);
        assert_eq!(Quadrants::TopRight, q);
    }
	#[test]
    fn top_right_ensure_zero3() {
		let min_xy: (i32, i32) = (0, 1);
		let max_xy: (i32, i32) = (1, 2);
		let q = get_quadrants(min_xy, max_xy);
        assert_eq!(Quadrants::TopRight, q);
    }
	#[test]
    fn top_right() {
		let min_xy: (i32, i32) = (1, 1);
		let max_xy: (i32, i32) = (2, 2);
		let q = get_quadrants(min_xy, max_xy);
        assert_eq!(Quadrants::TopRight, q);
    }
	#[test]
    fn top_left_ensure_zero() {
		let min_xy: (i32, i32) = (-1, 0);
		let max_xy: (i32, i32) = (0, 1);
		let q = get_quadrants(min_xy, max_xy);
        assert_eq!(Quadrants::TopLeft, q);
    }
	#[test]
    fn top_left_ensure_zero2() {
		let min_xy: (i32, i32) = (-2, 0);
		let max_xy: (i32, i32) = (-1, 1);
		let q = get_quadrants(min_xy, max_xy);
        assert_eq!(Quadrants::TopLeft, q);
    }
	#[test]
    fn top_left_ensure_zero3() {
		let min_xy: (i32, i32) = (-1, 1);
		let max_xy: (i32, i32) = (0, 2);
		let q = get_quadrants(min_xy, max_xy);
        assert_eq!(Quadrants::TopLeft, q);
    }
	#[test]
    fn top_left() {
		let min_xy: (i32, i32) = (-2, 1);
		let max_xy: (i32, i32) = (-1, 2);
		let q = get_quadrants(min_xy, max_xy);
        assert_eq!(Quadrants::TopLeft, q);
    }
	#[test]
    fn bottom_left_ensure_zero() {
		let min_xy: (i32, i32) = (-1, -1);
		let max_xy: (i32, i32) = (0, 0);
		let q = get_quadrants(min_xy, max_xy);
        assert_eq!(Quadrants::BottomLeft, q);
    }
	#[test]
    fn bottom_left_ensure_zero2() {
		let min_xy: (i32, i32) = (-2, -1);
		let max_xy: (i32, i32) = (-1, 0);
		let q = get_quadrants(min_xy, max_xy);
        assert_eq!(Quadrants::BottomLeft, q);
    }
	#[test]
    fn bottom_left_ensure_zero3() {
		let min_xy: (i32, i32) = (-1, -2);
		let max_xy: (i32, i32) = (0, -1);
		let q = get_quadrants(min_xy, max_xy);
        assert_eq!(Quadrants::BottomLeft, q);
    }
	#[test]
    fn bottom_left() {
		let min_xy: (i32, i32) = (-2, -2);
		let max_xy: (i32, i32) = (-1, -1);
		let q = get_quadrants(min_xy, max_xy);
        assert_eq!(Quadrants::BottomLeft, q);
    }
	#[test]
    fn bottom_right_ensure_zero() {
		let min_xy: (i32, i32) = (0, -1);
		let max_xy: (i32, i32) = (1, 0);
		let q = get_quadrants(min_xy, max_xy);
        assert_eq!(Quadrants::BottomRight, q);
    }
	#[test]
    fn bottom_right_ensure_zero2() {
		let min_xy: (i32, i32) = (1, -1);
		let max_xy: (i32, i32) = (2, 0);
		let q = get_quadrants(min_xy, max_xy);
        assert_eq!(Quadrants::BottomRight, q);
    }
	#[test]
    fn bottom_right_ensure_zero3() {
		let min_xy: (i32, i32) = (0, -2);
		let max_xy: (i32, i32) = (1, -1);
		let q = get_quadrants(min_xy, max_xy);
        assert_eq!(Quadrants::BottomRight, q);
    }
	#[test]
    fn bottom_right() {
		let min_xy: (i32, i32) = (1, -2);
		let max_xy: (i32, i32) = (2, -1);
		let q = get_quadrants(min_xy, max_xy);
        assert_eq!(Quadrants::BottomRight, q);
    }
}