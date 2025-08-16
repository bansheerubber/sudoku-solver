use std::collections::{HashMap, HashSet};

use crate::{
	grid::{CellValue, Coord, Grid},
	grid_image::GridImage,
	square::{Square, SquareIndex},
	vec2::{Vec2, SUDOKU},
};

pub struct Analysis<'a> {
	cheating: bool,
	pub grid: &'a mut Grid,
}

pub const DEBUG: bool = true;

impl<'a> Analysis<'a> {
	pub fn new(grid: &'a mut Grid, cheating: bool) -> Self {
		Analysis { cheating, grid }
	}

	pub fn round(&mut self) -> usize {
		let mut numbers_inserted = 0;

		let answers = self.single_in_squares();
		for (point, answer) in answers {
			self.grid.insert_number(&point, answer);
			numbers_inserted += 1;
		}

		let answers = self.lonely_cells();
		for (point, answer) in answers {
			self.grid.insert_number(&point, answer);
			numbers_inserted += 1;
		}

		if self.single_line_in_squares() {
			numbers_inserted += 1;
		}

		if self.square_claim_rows() {
			numbers_inserted += 1;
		}

		if self.square_claim_columns() {
			numbers_inserted += 1;
		}

		if self.cheating && numbers_inserted == 0 && self.cheat() {
			return 0;
		}

		return numbers_inserted;
	}

	pub fn verify(&mut self) -> bool {
		self.grid.verify()
	}

	pub fn calculate_all_candidates(&mut self) {
		for point in SUDOKU.iter() {
			if !self.grid.has_number(point) {
				self.grid.calculate_candidates(point);
			}
		}
	}

	pub fn single_in_squares(&self) -> Vec<(Vec2, CellValue)> {
		let mut counts: HashMap<SquareIndex, HashMap<CellValue, Vec<Vec2>>> = HashMap::new();

		for point in SUDOKU.iter() {
			if self.grid.has_number(point) {
				continue;
			}

			let candidates = &self.grid.get_candidates(point);
			let square_index = Square::point_to_index(point);

			for candidate in candidates.iter() {
				counts
					.entry(square_index as SquareIndex)
					.or_default()
					.entry(*candidate)
					.or_default()
					.push(*point);
			}
		}

		let mut results = vec![];
		for maps in counts.values() {
			for (candidate, coords) in maps.iter() {
				if coords.len() == 1 {
					results.push((coords[0], *candidate));
				}
			}
		}

		return results;
	}

	pub fn lonely_cells(&self) -> Vec<(Vec2, CellValue)> {
		let mut results = vec![];

		for point in SUDOKU.iter() {
			let candidates = self.grid.get_candidates(point);
			if candidates.len() == 1 {
				results.push((*point, candidates[0]));
			}
		}

		return results;
	}

	pub fn single_line_in_squares(&mut self) -> bool {
		let mut changes_made = false;

		let mut candidate_rows: HashMap<SquareIndex, HashMap<CellValue, HashSet<Coord>>> =
			HashMap::new();

		let mut candidate_columns: HashMap<SquareIndex, HashMap<CellValue, HashSet<Coord>>> =
			HashMap::new();

		for x in 0..3 {
			for y in 0..3 {
				let square_index = Square::square_coord_to_index(x, y);

				for y2 in y * 3..y * 3 + 3 {
					for x2 in x * 3..x * 3 + 3 {
						for &candidate in self.grid.get_candidates(&Vec2::new(x2, y2)).iter() {
							candidate_rows
								.entry(square_index as u8)
								.or_default()
								.entry(candidate)
								.or_default()
								.insert(y2);

							candidate_columns
								.entry(square_index as u8)
								.or_default()
								.entry(candidate)
								.or_default()
								.insert(x2);
						}
					}
				}
			}
		}

		// TODO this sucks!!!!!
		for x in 0..3 {
			for y in 0..3 {
				let square_index = Square::square_coord_to_index(x, y) as u8;

				if let Some(cell_rows) = candidate_rows.get(&square_index) {
					for (&candidate, set) in cell_rows.iter() {
						if set.len() != 1 || set.len() == 0 {
							continue;
						}

						let mut candidates_to_remove = vec![];

						let row_index = *set.iter().nth(0).unwrap();
						let row = &self.grid.rows[row_index as usize];
						for point in row.coords() {
							if point.x >= x * 3 && point.y < x * 3 + 3 {
								continue;
							}

							let adjacent_square_index = Square::point_to_index(point) as u8;

							let Some(candidate_map) = candidate_rows.get(&adjacent_square_index)
							else {
								continue;
							};

							let Some(row_set) = candidate_map.get(&candidate) else {
								continue;
							};

							if row_set.len() <= 1 {
								continue;
							}

							candidates_to_remove.push((*point, candidate));
						}

						for &(point, candidate) in candidates_to_remove.iter() {
							let candidates = self.grid.get_candidates(&point);
							let start = candidates.len();

							self.grid.remove_candidate(&point, candidate);

							let candidates = self.grid.get_candidates(&point);
							if start != candidates.len() {
								changes_made = true;
							}
						}
					}
				}

				if let Some(cell_columns) = candidate_columns.get(&square_index) {
					for (&candidate, set) in cell_columns.iter() {
						if set.len() != 1 || set.len() == 0 {
							continue;
						}

						let mut candidates_to_remove = vec![];

						let column_index = *set.iter().nth(0).unwrap();
						let column = &self.grid.columns[column_index as usize];
						for point in column.coords() {
							if point.x >= x * 3 && point.y < x * 3 + 3 {
								continue;
							}

							let adjacent_square_index = Square::point_to_index(point) as u8;

							let Some(candidate_map) = candidate_columns.get(&adjacent_square_index)
							else {
								continue;
							};

							let Some(column_set) = candidate_map.get(&candidate) else {
								continue;
							};

							if column_set.len() <= 1 {
								continue;
							}

							candidates_to_remove.push((*point, candidate));
						}

						for &(point, candidate) in candidates_to_remove.iter() {
							let candidates = self.grid.get_candidates(&point);
							let start = candidates.len();

							self.grid.remove_candidate(&point, candidate);

							let candidates = self.grid.get_candidates(&point);
							if start != candidates.len() {
								changes_made = true;
							}
						}
					}
				}
			}
		}

		return changes_made;
	}

	pub fn square_claim_rows(&mut self) -> bool {
		let mut exclusives: HashMap<(Coord, CellValue), u8> = HashMap::new();

		for row in self.grid.rows.iter() {
			for number in 1..=9 {
				for mini_line in row.mini_lines.iter() {
					if !mini_line.has_candidate_anywhere(number) {
						continue;
					}

					if let Some(square_x) = exclusives.get(&(row.rank(), number))
						&& *square_x != mini_line.square_point.x
					{
						exclusives.insert((row.rank(), number), 4);
					} else {
						exclusives.insert((row.rank(), number), mini_line.square_point.x);
					}
				}
			}
		}

		let mut changes_made = false;

		for (&(row, number), &square_x) in exclusives.iter() {
			if square_x == 4 {
				continue;
			}

			let start_row = (row / 3) * 3;
			let end_row = start_row + 3;

			for i in start_row..end_row {
				if i == row {
					continue;
				}

				for candidate_index in 0..3 {
					if self
						.grid
						.remove_candidate(&Vec2::new(square_x * 3 + candidate_index, i), number)
					{
						changes_made = true;
					}
				}
			}
		}

		return changes_made;
	}

	pub fn square_claim_columns(&mut self) -> bool {
		let mut exclusives: HashMap<(Coord, CellValue), u8> = HashMap::new();

		for column in self.grid.columns.iter() {
			for number in 1..=9 {
				for mini_line in column.mini_lines.iter() {
					if !mini_line.has_candidate_anywhere(number) {
						continue;
					}

					if let Some(square_x) = exclusives.get(&(column.rank(), number))
						&& *square_x != mini_line.square_point.y
					{
						exclusives.insert((column.rank(), number), 4);
					} else {
						exclusives.insert((column.rank(), number), mini_line.square_point.y);
					}
				}
			}
		}

		let mut changes_made = false;

		for (&(column, number), &square_y) in exclusives.iter() {
			if square_y == 4 {
				continue;
			}

			let start_column = (column / 3) * 3;
			let end_column = start_column + 3;

			for i in start_column..end_column {
				if i == column {
					continue;
				}

				for candidate_index in 0..3 {
					if self
						.grid
						.remove_candidate(&Vec2::new(i, square_y * 3 + candidate_index), number)
					{
						changes_made = true;
					}
				}
			}
		}

		return changes_made;
	}

	pub fn cheat(&mut self) -> bool {
		let mut best_square = &self.grid.squares[0];
		for square in self.grid.squares.iter() {
			if (square.cells.len() > best_square.cells.len() && square.cells.len() != 9)
				|| best_square.cells.len() == 0
			{
				best_square = square;
			}
		}

		let mut candidate_coords = Vec2::new(0, 0);
		let mut candidates = vec![];

		for point in best_square.coords() {
			let candidates2 = self.grid.get_candidates(point);
			if candidates2.len() != 0 {
				candidate_coords = *point;
				candidates = candidates2.clone();
				break;
			}
		}

		let mut grids = vec![];

		for &candidate in candidates.iter() {
			let mut new_grid = self.grid.clone();
			new_grid.insert_number(&candidate_coords, candidate);

			grids.push(new_grid);
		}

		for mut grid in grids.iter_mut() {
			let mut analysis = Analysis::new(&mut grid, self.cheating);
			while analysis.round() != 0 && analysis.verify() {
				if DEBUG {
					analysis.draw();

					let mut string = String::new();
					std::io::stdin().read_line(&mut string).unwrap();
				}
			}

			if DEBUG {
				analysis.draw();

				let mut string = String::new();
				std::io::stdin().read_line(&mut string).unwrap();
			}
		}

		return true;
	}

	pub fn draw(&self) {
		let mut grid_image = GridImage::new(&self.grid);
		grid_image.create_image();
		grid_image.draw_candidates();
		grid_image.save();
	}
}
