use std::collections::HashSet;

use crate::{
	grid::{CellValue, Coord},
	mini_line::MiniLine,
	vec2::Vec2,
};

#[derive(Clone, Copy, Debug, Default)]
pub enum LineDirection {
	#[default]
	Row,
	Column,
}

impl LineDirection {
	pub fn coords(&self, start: &Vec2, amount: Coord) -> Vec<Vec2> {
		let mut result = vec![];

		match self {
			LineDirection::Row => {
				for x in start.x..start.x + amount {
					result.push(Vec2::new(x, start.y));
				}
			}
			LineDirection::Column => {
				for y in start.y..start.y + amount {
					result.push(Vec2::new(start.x, y));
				}
			}
		}

		return result;
	}

	pub fn mini_line_index(&self, point: &Vec2) -> (usize, usize) {
		match self {
			LineDirection::Row => (point.x as usize / 3, point.x as usize % 3),
			LineDirection::Column => (point.y as usize / 3, point.y as usize % 3),
		}
	}
}

#[derive(Clone, Debug, Default)]
pub struct Line {
	pub direction: LineDirection,
	pub mini_lines: [MiniLine; 3],
	pub point: Vec2,
	coords: Vec<Vec2>,
}

impl Line {
	pub fn new(point: Vec2, direction: LineDirection) -> Self {
		let mut mini_lines: [MiniLine; 3] = Default::default();

		match direction {
			LineDirection::Row => assert!(point.x == 0, "row has non-zero x"),
			LineDirection::Column => assert!(point.y == 0, "column has non-zero y"),
		}

		let mut index = 0;
		for square_coord in direction.coords(&Vec2::new(point.x / 3, point.y / 3), 3) {
			let start = match direction {
				LineDirection::Row => Vec2::new(square_coord.x * 3, point.y),
				LineDirection::Column => Vec2::new(point.x, square_coord.y * 3),
			};

			mini_lines[index] = MiniLine::new(start);

			index += 1;
		}

		Line {
			coords: direction.coords(&point, 9),
			direction,
			mini_lines,
			point,
		}
	}

	pub fn has_number(&self, number: CellValue) -> bool {
		for mini_row in self.mini_lines.iter() {
			for &cell in mini_row.cells.iter() {
				if cell == number {
					return true;
				}
			}
		}

		return false;
	}

	pub fn set_number(&mut self, point: &Vec2, number: CellValue) {
		self.assert_contains_point(point);

		let (mini_line_index, cell_index) = self.direction.mini_line_index(&point);
		self.mini_lines[mini_line_index].cells[cell_index] = number;
	}

	pub fn get_number(&self, point: &Vec2) -> CellValue {
		self.assert_contains_point(point);

		let (mini_line_index, cell_index) = self.direction.mini_line_index(&point);
		self.mini_lines[mini_line_index].cells[cell_index]
	}

	pub fn set_candidates(&mut self, point: &Vec2, candidates: Vec<CellValue>) {
		self.assert_contains_point(point);

		let (mini_line_index, cell_index) = self.direction.mini_line_index(&point);
		self.mini_lines[mini_line_index].set_candidates(cell_index, candidates);
	}

	pub fn remove_candidate(&mut self, point: &Vec2, number: CellValue) -> bool {
		self.assert_contains_point(point);

		let (mini_line_index, cell_index) = self.direction.mini_line_index(&point);
		self.mini_lines[mini_line_index].remove_candidate(cell_index, number)
	}

	pub fn clear_candidates(&mut self, point: &Vec2) {
		self.assert_contains_point(point);

		let (mini_line_index, cell_index) = self.direction.mini_line_index(&point);
		self.mini_lines[mini_line_index].clear_candidates(cell_index);
	}

	pub fn get_candidates(&self, point: &Vec2) -> &Vec<CellValue> {
		self.assert_contains_point(point);

		let (mini_line_index, cell_index) = self.direction.mini_line_index(&point);
		self.mini_lines[mini_line_index].get_candidates(cell_index)
	}

	fn assert_contains_point(&self, point: &Vec2) {
		match self.direction {
			LineDirection::Row => assert!(point.y == self.point.y, "row does not contain point"),
			LineDirection::Column => assert!(point.x == self.point.x, "column does not contain point"),
		}
	}

	pub fn coords(&self) -> impl Iterator<Item = &Vec2> {
		self.coords.iter()
	}

	pub fn rank(&self) -> Coord {
		match self.direction {
			LineDirection::Row => self.point.y,
			LineDirection::Column => self.point.x,
		}
	}

	pub fn verify(&self) -> Option<Vec2> {
		let mut set = HashSet::new();

		for mini_line in self.mini_lines.iter() {
			for i in 0..3 {
				let cell = mini_line.cells[i];
				if cell == 0 {
					continue;
				}

				if set.contains(&cell) {
					match self.direction {
						LineDirection::Row => {
							return Some(Vec2::new(i as Coord + mini_line.point.x, self.point.y));
						}
						LineDirection::Column => {
							return Some(Vec2::new(self.point.x, i as Coord + mini_line.point.y));
						}
					}
				}

				set.insert(cell);
			}
		}

		return None;
	}

	#[allow(unused)]
	pub fn print(&self) {
		for mini_line in self.mini_lines.iter() {
			for &value in mini_line.cells.iter() {
				if value == 0 {
					print!("_ ");
				} else {
					print!("{} ", value);
				}
			}
		}
		print!("\n");
	}
}
