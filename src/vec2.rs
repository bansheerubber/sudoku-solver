use lazy_static::lazy_static;
use std::{
	fmt::Display,
	ops::{Add, Sub},
};

use crate::grid::Coord;

lazy_static! {
	pub static ref SUDOKU: Vec<Vec2> = {
		let mut sudoku = vec![];
		for x in 0..9 {
			for y in 0..9 {
				sudoku.push(Vec2::new(x, y));
			}
		}

		sudoku
	};
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Vec2 {
	pub x: Coord,
	pub y: Coord,
}

impl Vec2 {
	pub fn new(x: Coord, y: Coord) -> Self {
		Self { x, y }
	}
}

impl Add for Vec2 {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self {
			x: self.x + other.x,
			y: self.y + other.y,
		}
	}
}

impl Sub for Vec2 {
	type Output = Self;

	fn sub(self, other: Self) -> Self {
		Self {
			x: self.x - other.x,
			y: self.y - other.y,
		}
	}
}

impl Display for Vec2 {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("({}, {})", self.x, self.y))
	}
}
