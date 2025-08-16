use crate::{grid::CellValue, vec2::Vec2};

#[derive(Clone, Debug, Default)]
pub struct MiniLine {
	pub cells: [CellValue; 3],
	pub point: Vec2,
	pub square_point: Vec2,
	candidates: [Vec<CellValue>; 3],
}

impl MiniLine {
	pub fn new(point: Vec2) -> Self {
		MiniLine {
			cells: [0; 3],
			point,
			square_point: Vec2::new(point.x / 3, point.y / 3),
			candidates: Default::default(),
		}
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
