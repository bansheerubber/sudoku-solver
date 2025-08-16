use crate::grid::{CellValue, Coord};

#[derive(Clone, Debug, Default)]
pub struct MiniColumn {
	pub cells: [CellValue; 3],
	pub column: Coord,
	pub square_y: Coord,
	coords: Vec<(Coord, Coord)>,
	candidates: [Vec<CellValue>; 3],
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
			candidates: Default::default(),
		}
	}

	pub fn add_candidate(&mut self, candidate_index: usize, number: CellValue) {
		self.candidates[candidate_index].push(number);
	}

	pub fn set_candidates(&mut self, candidate_index: usize, candidates: Vec<CellValue>) {
		self.candidates[candidate_index] = candidates;
	}

	pub fn remove_candidate(&mut self, candidate_index: usize, number: CellValue) -> bool {
		if let Some(index) = self.candidates[candidate_index]
			.iter()
			.position(|&candidate| candidate == number)
		{
			self.candidates[candidate_index].remove(index);
			return true;
		} else {
			return false;
		}
	}

	pub fn clear_candidates(&mut self, candidate_index: usize) {
		self.candidates[candidate_index].clear();
	}

	pub fn get_candidates(&self, candidate_index: usize) -> &Vec<CellValue> {
		&self.candidates[candidate_index]
	}

	pub fn has_candidate_anywhere(&self, number: CellValue) -> bool {
		for candidates in self.candidates.iter() {
			if candidates.contains(&number) {
				return true;
			}
		}

		return false;
	}
}
