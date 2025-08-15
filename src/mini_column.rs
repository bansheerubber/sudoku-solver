use crate::grid::{CellValue, Coord};

#[derive(Clone, Debug, Default)]
pub struct MiniColumn {
	pub cells: [CellValue; 3],
	pub column: Coord,
	pub square_y: Coord,
	coords: Vec<(Coord, Coord)>,
}

impl MiniColumn {
	pub fn new(square_y: Coord, column: Coord) -> Self {
		let mut coords = vec![];
		for y in square_y * 3..square_y * 3 + 3 {
			coords.push((column, y));
		}

		MiniColumn {
			cells: [0; 3],
			column,
			square_y,
			coords,
		}
	}
}
