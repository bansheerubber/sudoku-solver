use std::collections::HashSet;

use crate::{
	grid::{CellValue, Coord},
	mini_column::MiniColumn,
};

#[derive(Clone, Debug, Default)]
pub struct Column {
	pub mini_columns: [MiniColumn; 3],
	coords: Vec<(Coord, Coord)>,
	pub column: Coord,
}

impl Column {
	pub fn new(column: u8) -> Self {
		let mut mini_columns: [MiniColumn; 3] = Default::default();

		for square_y in 0..3 {
			mini_columns[square_y] = MiniColumn::new(square_y as Coord, column);
		}

		let mut coords = vec![];
		for y in 0..9 {
			coords.push((column, y));
		}

		Column {
			mini_columns,
			coords,
			column,
		}
	}

	pub fn has_number(&self, number: CellValue) -> bool {
		for mini_row in self.mini_columns.iter() {
			for &cell in mini_row.cells.iter() {
				if cell == number {
					return true;
				}
			}
		}

		return false;
	}

	pub fn set_number(&mut self, y: Coord, number: CellValue) {
		self.mini_columns[y as usize / 3].cells[y as usize % 3] = number;
	}

	pub fn get_number(&self, y: Coord) -> CellValue {
		self.mini_columns[y as usize / 3].cells[y as usize % 3]
	}

	pub fn coords(&self) -> impl Iterator<Item = &(Coord, Coord)> {
		self.coords.iter()
	}

	pub fn add_candidate(&mut self, y: Coord, number: CellValue) {
		self.mini_columns[y as usize / 3].add_candidate(y as usize % 3, number);
	}

	pub fn set_candidates(&mut self, y: Coord, candidates: Vec<CellValue>) {
		self.mini_columns[y as usize / 3].set_candidates(y as usize % 3, candidates);
	}

	pub fn remove_candidate(&mut self, y: Coord, number: CellValue) -> bool {
		self.mini_columns[y as usize / 3].remove_candidate(y as usize % 3, number)
	}

	pub fn clear_candidates(&mut self, y: Coord) {
		self.mini_columns[y as usize / 3].clear_candidates(y as usize % 3);
	}

	pub fn get_candidates(&self, y: Coord) -> &Vec<CellValue> {
		self.mini_columns[y as usize / 3].get_candidates(y as usize % 3)
	}

	pub fn verify(&self) -> Coord {
		let mut set = HashSet::new();

		for mini_row in self.mini_columns.iter() {
			for i in 0..3 {
				let cell = mini_row.cells[i];
				if cell == 0 {
					continue;
				}

				if set.contains(&cell) {
					return i as u8 + mini_row.square_y * 3;
				}

				set.insert(cell);
			}
		}

		return 0;
	}
}
