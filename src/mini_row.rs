use crate::grid::{CellValue, Coord};

#[derive(Clone, Debug, Default)]
pub struct MiniRow {
	pub cells: [CellValue; 3],
	pub row: Coord,
	pub square_x: Coord,
	coords: Vec<(Coord, Coord)>,
	candidates: [Vec<CellValue>; 3],
}

impl MiniRow {
	pub fn new(square_x: Coord, row: Coord) -> Self {
		let mut coords = vec![];
		for x in square_x * 3..square_x * 3 + 3 {
			coords.push((x, row));
		}

		MiniRow {
			cells: [0; 3],
			row,
			square_x,
			coords,
			candidates: Default::default(),
		}
	}
}
