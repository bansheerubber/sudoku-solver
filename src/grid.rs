use std::collections::{HashMap, HashSet};

use crate::{column::Column, row::Row, square::Square};

pub type CellValue = u8;
pub type Coord = u8;

#[derive(Clone, Default)]
pub struct Grid {
	pub columns: [Column; 9],
	pub original_numbers: HashSet<(Coord, Coord)>,
	pub rows: [Row; 9],
	pub squares: [Square; 9],
	pub invalid_cells: Vec<(Coord, Coord)>,
	pub solution: HashMap<(Coord, Coord), CellValue>,
}

impl Grid {
	pub fn new() -> Self {
		let mut grid = Grid::default();

		for row in 0..9 {
			grid.rows[row] = Row::new(row as u8);
		}

		for column in 0..9 {
			grid.columns[column] = Column::new(column as u8);
		}

		for x in 0..3 {
			for y in 0..3 {
				grid.squares[Square::coord_to_index(x, y)] = Square::new(x, y);
			}
		}

		for x in 0..3 {
			for y in 0..3 {
				let square = grid
					.squares
					.get_mut(Square::coord_to_index(x, y))
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
		let lines = std::fs::read_to_string("./puzzle3.txt").expect("Could not read puzzle");
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
						self.solution.insert((x, y), number);
					} else {
						self.insert_number(x, y, number);
						self.original_numbers.insert((x, y));
					}
				}

				x += 1;
			}

			x = 0;
			y += 1;
		}
	}

	pub fn has_number(&self, x: Coord, y: Coord) -> bool {
		self.rows[y as usize].get_number(x) != 0
	}

	pub fn insert_number(&mut self, x: Coord, y: Coord, number: CellValue) {
		let square = &mut self.squares[Square::coord_to_index(x / 3, y / 3)];
		square.cells.insert(number);

		let row = &mut self.rows[y as usize];
		let column = &mut self.columns[x as usize];

		row.set_number(x, number);
		column.set_number(y, number);

		row.clear_candidates(x);
		column.clear_candidates(y);

		for &(x, y) in square.coords() {
			let row = &mut self.rows[y as usize];
			let column = &mut self.columns[x as usize];

			row.remove_candidate(x, number);
			column.remove_candidate(y, number);
		}

		for x in 0..9 {
			self.remove_candidate(x, y, number);
		}

		for y in 0..9 {
			self.remove_candidate(x, y, number);
		}
	}

	pub fn verify_data_structure(&self) {
		for x in 0..9 {
			for y in 0..9 {
				let square = &self.squares[Square::coord_to_index(x / 3, y / 3)];
				let number = self.get_number(x, y);
				if number == 0 {
					continue;
				}

				assert!(square.cells.contains(&number), "({}, {}): {}", x, y, number);
			}
		}
	}

	pub fn verify(&mut self) -> bool {
		for row in 0..9 {
			let x = self.rows[row].verify();
			if x != 0 {
				self.invalid_cells.push((x, row as u8));
				return false;
			}
		}

		for column in 0..9 {
			let y = self.columns[column].verify();
			if y != 0 {
				self.invalid_cells.push((column as u8, y));
				return false;
			}
		}

		for x in 0..9 {
			for y in 0..9 {
				if !self.has_number(x, y) && self.get_candidates(x, y).len() == 0 {
					return false;
				}
			}
		}

		return true;
	}

	pub fn get_number(&self, x: Coord, y: Coord) -> CellValue {
		let row_number = self.rows[y as usize].get_number(x);
		let column_number = self.columns[x as usize].get_number(y);
		assert!(row_number == column_number);
		return row_number;
	}

	pub fn add_candidate(&mut self, x: Coord, y: Coord, number: CellValue) {
		self.rows[y as usize].add_candidate(x, number);
		self.columns[x as usize].add_candidate(y, number);
	}

	pub fn set_candidates(&mut self, x: Coord, y: Coord, candidates: Vec<CellValue>) {
		self.rows[y as usize].set_candidates(x, candidates.clone());
		self.columns[x as usize].set_candidates(y, candidates);
	}

	pub fn remove_candidate(&mut self, x: Coord, y: Coord, number: CellValue) -> bool {
		self.columns[x as usize].remove_candidate(y, number);
		return self.rows[y as usize].remove_candidate(x, number)
	}

	pub fn get_candidates(&self, x: Coord, y: Coord) -> &Vec<CellValue> {
		let row_candidates = self.rows[y as usize].get_candidates(x);
		let column_candidates = self.columns[x as usize].get_candidates(y);

		assert!(
			row_candidates == column_candidates,
			"({}, {}): {:?} != {:?}",
			x,
			y,
			row_candidates,
			column_candidates
		);

		return row_candidates;
	}

	pub fn calculate_candidates(&mut self, x: Coord, y: Coord) {
		let row = &self.rows[y as usize];
		let column = &self.columns[x as usize];
		let square = &self.squares[Square::coord_to_index(x / 3, y / 3)];

		let mut candidates = vec![];

		for candidate in 1..=9 {
			if !row.has_number(candidate)
				&& !column.has_number(candidate)
				&& !square.has_number(candidate)
			{
				candidates.push(candidate);
			}
		}

		self.set_candidates(x, y, candidates);
	}
}
