use std::collections::HashSet;

use crate::grid::{CellValue, Coord};

pub type SquareIndex = u8;

#[derive(Clone, Debug, Default)]
pub struct Square {
	pub cells: HashSet<CellValue>,
	pub columns: [usize; 3],
	pub rows: [usize; 3],
	pub coords: Vec<(Coord, Coord)>,
	pub x: Coord,
	pub y: Coord,
}

impl Square {
	pub fn new(x: Coord, y: Coord) -> Self {
		let mut coords = vec![];
		for x in x * 3..x * 3 + 3 {
			for y in y * 3..y * 3 + 3 {
				coords.push((x, y));
			}
		}

		Square {
			cells: HashSet::default(),
			columns: Default::default(),
			rows: Default::default(),
			coords,
			x,
			y,
		}
	}

	pub fn has_number(&self, number: CellValue) -> bool {
		self.cells.contains(&number)
	}

	pub fn coord_to_index(x: Coord, y: Coord) -> usize {
		(x + y * 3) as usize
	}

	pub fn coords(&self) -> impl Iterator<Item = &(Coord, Coord)> {
		self.coords.iter()
	}
}
