use std::collections::HashSet;

use crate::{
	grid::{CellValue, Coord},
	mini_row::MiniRow,
};

#[derive(Clone, Default)]
pub struct Row {
	pub mini_rows: [MiniRow; 3],
	coords: Vec<(Coord, Coord)>,
	pub row: Coord,
}

impl Row {
	pub fn new(row: u8) -> Self {
		let mut mini_rows: [MiniRow; 3] = Default::default();

		for square_x in 0..3 {
			mini_rows[square_x] = MiniRow::new(square_x as Coord, row);
		}

		let mut coords = vec![];
		for x in 0..9 {
			coords.push((x, row));
		}

		Row {
			mini_rows,
			coords,
			row,
		}
	}

	pub fn has_number(&self, number: CellValue) -> bool {
		for mini_row in self.mini_rows.iter() {
			for &cell in mini_row.cells.iter() {
				if cell == number {
					return true;
				}
			}
		}

		return false;
	}

	pub fn set_number(&mut self, x: Coord, number: CellValue) {
		self.mini_rows[x as usize / 3].cells[x as usize % 3] = number;
	}

	pub fn get_number(&self, x: Coord) -> CellValue {
		self.mini_rows[x as usize / 3].cells[x as usize % 3]
	}

	pub fn coords(&self) -> impl Iterator<Item = &(Coord, Coord)> {
		self.coords.iter()
	}

	pub fn add_candidate(&mut self, x: Coord, number: CellValue) {
		self.mini_rows[x as usize / 3].add_candidate(x as usize % 3, number);
	}

	pub fn set_candidates(&mut self, x: Coord, candidates: Vec<CellValue>) {
		self.mini_rows[x as usize / 3].set_candidates(x as usize % 3, candidates);
	}

	pub fn remove_candidate(&mut self, x: Coord, number: CellValue) -> bool {
		self.mini_rows[x as usize / 3].remove_candidate(x as usize % 3, number)
	}

	pub fn clear_candidates(&mut self, x: Coord) {
		self.mini_rows[x as usize / 3].clear_candidates(x as usize % 3);
	}

	pub fn get_candidates(&self, x: Coord) -> &Vec<CellValue> {
		self.mini_rows[x as usize / 3].get_candidates(x as usize % 3)
	}

	pub fn verify(&self) -> Coord {
		let mut set = HashSet::new();

		for mini_row in self.mini_rows.iter() {
			for i in 0..3 {
				let cell = mini_row.cells[i];
				if cell == 0 {
					continue;
				}

				if set.contains(&cell) {
					return i as u8 + mini_row.square_x * 3;
				}

				set.insert(cell);
			}
		}

		return 0;
	}
}
