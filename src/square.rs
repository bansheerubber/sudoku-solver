use std::collections::HashSet;

use crate::{
	grid::{CellValue, Coord},
	vec2::Vec2,
};

pub type SquareIndex = u8;

#[derive(Clone, Debug, Default)]
pub struct Square {
	pub cells: HashSet<CellValue>,
	pub columns: [usize; 3],
	pub coords: Vec<Vec2>,
	pub rows: [usize; 3],
	pub x: Coord,
	pub y: Coord,
}

impl Square {
	pub fn new(x: Coord, y: Coord) -> Self {
		let mut coords = vec![];
		for x in x * 3..x * 3 + 3 {
			for y in y * 3..y * 3 + 3 {
				coords.push(Vec2::new(x, y));
			}
		}

		Square {
			cells: HashSet::default(),
			columns: Default::default(),
			coords,
			rows: Default::default(),
			x,
			y,
		}
	}

	pub fn has_number(&self, number: CellValue) -> bool {
		self.cells.contains(&number)
	}

	pub fn square_coord_to_index(x: Coord, y: Coord) -> usize {
		(x + y * 3) as usize
	}

	pub fn point_to_index(point: &Vec2) -> usize {
		Square::square_coord_to_index(point.x / 3, point.y / 3)
	}

	pub fn coords(&self) -> impl Iterator<Item = &Vec2> {
		self.coords.iter()
	}
}
