use std::collections::{HashMap, HashSet};

use crate::{
	line::{Line, LineDirection},
	square::Square,
	vec2::Vec2,
};

pub type CellValue = u8;
pub type Coord = u8;

#[derive(Clone, Default)]
pub struct Grid {
	pub columns: [Line; 9],
	pub original_numbers: HashSet<Vec2>,
	pub rows: [Line; 9],
	pub squares: [Square; 9],
	pub invalid_cells: Vec<Vec2>,
	pub solution: HashMap<Vec2, CellValue>,
}

impl Grid {
	pub fn new() -> Self {
		let mut grid = Grid::default();

		for row in 0..9 {
			grid.rows[row] = Line::new(Vec2::new(0, row as Coord), LineDirection::Row);
		}

		for column in 0..9 {
			grid.columns[column] = Line::new(Vec2::new(column as Coord, 0), LineDirection::Column);
		}

		for x in 0..3 {
			for y in 0..3 {
				grid.squares[Square::square_coord_to_index(x, y)] = Square::new(x, y);
			}
		}

		for x in 0..3 {
			for y in 0..3 {
				let square = grid
					.squares
					.get_mut(Square::square_coord_to_index(x, y))
					.expect("Could not get square");

				let mut index = 0;
				for i in (x * 3)..(x * 3 + 3) {
					square.rows[index] = i as usize;
					index += 1;
				}

				let mut index = 0;
				for i in (y * 3)..(y * 3 + 3) {
					square.columns[index] = i as usize;
					index += 1;
				}
			}
		}

		return grid;
	}

	pub fn load(&mut self) {
		let lines = std::fs::read_to_string("./puzzle1.txt").expect("Could not read puzzle");
		let lines = lines.split("\n");

		let mut x = 0;
		let mut y = 0;

		let mut load_solution = false;
		for line in lines {
			if line.trim().len() == 0 {
				load_solution = true;
				x = 0;
				y = 0;
				continue;
			}

			for char in line.trim().chars() {
				if char != '_' {
					let number = char
						.to_string()
						.parse::<CellValue>()
						.expect("Could not parse number");

					if load_solution {
						self.solution.insert(Vec2::new(x, y), number);
					} else {
						self.insert_number(&Vec2::new(x, y), number);
						self.original_numbers.insert(Vec2::new(x, y));
					}
				}

				x += 1;
			}

			x = 0;
			y += 1;
		}
	}

	pub fn has_number(&self, point: &Vec2) -> bool {
		self.rows[point.y as usize].get_number(point) != 0
	}

	pub fn insert_number(&mut self, point: &Vec2, number: CellValue) {
		let square = &mut self.squares[Square::square_coord_to_index(point.x / 3, point.y / 3)];
		square.cells.insert(number);

		let row = &mut self.rows[point.y as usize];
		let column = &mut self.columns[point.x as usize];

		row.set_number(point, number);
		column.set_number(point, number);

		row.clear_candidates(point);
		column.clear_candidates(point);

		for point in square.coords() {
			let row = &mut self.rows[point.y as usize];
			let column = &mut self.columns[point.x as usize];

			row.remove_candidate(point, number);
			column.remove_candidate(point, number);
		}

		for x in 0..9 {
			self.remove_candidate(&Vec2::new(x, point.y), number);
		}

		for y in 0..9 {
			self.remove_candidate(&Vec2::new(point.x, y), number);
		}
	}

	pub fn verify_data_structure(&self) {
		for x in 0..9 {
			for y in 0..9 {
				let square = &self.squares[Square::square_coord_to_index(x / 3, y / 3)];
				let number = self.get_number(&Vec2::new(x, y));
				if number == 0 {
					continue;
				}

				assert!(square.cells.contains(&number), "({}, {}): {}", x, y, number);
			}
		}
	}

	pub fn verify(&mut self) -> bool {
		let mut valid = true;

		for row in self.rows.iter() {
			if let Some(invalid_point) = row.verify() {
				self.invalid_cells.push(invalid_point);
				valid = false;
			}
		}

		for column in self.columns.iter() {
			if let Some(invalid_point) = column.verify() {
				self.invalid_cells.push(invalid_point);
				valid = false;
			}
		}

		for x in 0..9 {
			for y in 0..9 {
				if !self.has_number(&Vec2::new(x, y))
					&& self.get_candidates(&Vec2::new(x, y)).len() == 0
				{
					return false;
				}
			}
		}

		return valid;
	}

	pub fn get_number(&self, point: &Vec2) -> CellValue {
		let row_number = self.rows[point.y as usize].get_number(point);
		let column_number = self.columns[point.x as usize].get_number(point);
		assert!(row_number == column_number);
		return row_number;
	}

	pub fn set_candidates(&mut self, point: &Vec2, candidates: Vec<CellValue>) {
		self.rows[point.y as usize].set_candidates(point, candidates.clone());
		self.columns[point.x as usize].set_candidates(point, candidates);
	}

	pub fn remove_candidate(&mut self, point: &Vec2, number: CellValue) -> bool {
		self.columns[point.x as usize].remove_candidate(point, number);
		return self.rows[point.y as usize].remove_candidate(point, number);
	}

	pub fn get_candidates(&self, point: &Vec2) -> &Vec<CellValue> {
		let row_candidates = self.rows[point.y as usize].get_candidates(point);
		let column_candidates = self.columns[point.x as usize].get_candidates(point);

		assert!(
			row_candidates == column_candidates,
			"{}: {:?} != {:?}",
			point,
			row_candidates,
			column_candidates
		);

		return row_candidates;
	}

	pub fn calculate_candidates(&mut self, point: &Vec2) {
		let row = &self.rows[point.y as usize];
		let column = &self.columns[point.x as usize];
		let square = &self.squares[Square::square_coord_to_index(point.x / 3, point.y / 3)];

		let mut candidates = vec![];

		for candidate in 1..=9 {
			if !row.has_number(candidate)
				&& !column.has_number(candidate)
				&& !square.has_number(candidate)
			{
				candidates.push(candidate);
			}
		}

		self.set_candidates(point, candidates);
	}
}
