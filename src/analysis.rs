use std::collections::{HashMap, HashSet};

use crate::{
	grid::{CellValue, Coord, Grid},
	grid_image::GridImage,
	square::{Square, SquareIndex},
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
		for ((x, y), answer) in answers {
			self.grid.insert_number(x, y, answer);
			numbers_inserted += 1;
		}

		let answers = self.lonely_cells();
		for ((x, y), answer) in answers {
			self.grid.insert_number(x, y, answer);
			numbers_inserted += 1;
		}

		if self.single_line_in_squares() {
			numbers_inserted += 1;
		}

		// self.square_claim();

		if self.cheating && numbers_inserted == 0 && self.cheat() {
			return 0;
		}

		return numbers_inserted;
	}

	pub fn verify(&mut self) -> bool {
		self.grid.verify()
	}

	pub fn calculate_all_candidates(&mut self) {
		self.grid.candidates = Default::default();
		for x in 0..9 {
			for y in 0..9 {
				if !self.grid.has_number(x, y) {
					self.grid.calculate_candidates(x, y);
				}
			}
		}
	}

	pub fn single_in_squares(&self) -> Vec<((Coord, Coord), CellValue)> {
		let mut counts: HashMap<SquareIndex, HashMap<CellValue, Vec<(Coord, Coord)>>> =
			HashMap::new();

		for x in 0..9 {
			for y in 0..9 {
				if self.grid.has_number(x, y) {
					continue;
				}

				let candidates = &self.grid.candidates[&(x, y)];
				let square_index = Square::coord_to_index(x / 3, y / 3);

				for candidate in candidates {
					counts
						.entry(square_index as SquareIndex)
						.or_default()
						.entry(*candidate)
						.or_default()
						.push((x, y));
				}
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

	pub fn lonely_cells(&self) -> Vec<((Coord, Coord), CellValue)> {
		let mut results = vec![];
		for ((x, y), candidates) in self.grid.candidates.iter() {
			if candidates.len() == 1 {
				results.push(((*x, *y), candidates[0]));
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
				let square_index = Square::coord_to_index(x, y);

				for y2 in y * 3..y * 3 + 3 {
					for x2 in x * 3..x * 3 + 3 {
						for &candidate in self
							.grid
							.candidates
							.get(&(x2, y2))
							.unwrap_or(&vec![])
							.iter()
						{
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
				let square_index = Square::coord_to_index(x, y) as u8;

				if let Some(cell_rows) = candidate_rows.get(&square_index) {
					for (&candidate, set) in cell_rows.iter() {
						if set.len() != 1 || set.len() == 0 {
							continue;
						}

						let row_index = *set.iter().nth(0).unwrap();
						let row = &self.grid.rows[row_index as usize];
						for &(row_x, row_y) in row.coords() {
							if row_x >= x * 3 && row_x < x * 3 + 3 {
								continue;
							}

							let adjacent_square_index =
								Square::coord_to_index(row_x / 3, row_y / 3) as u8;

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

							let candidates =
								self.grid.candidates.entry((row_x, row_y)).or_default();

							let start = candidates.len();
							candidates.retain(|&c| c != candidate);

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

						let column_index = *set.iter().nth(0).unwrap();
						let column = &self.grid.columns[column_index as usize];
						for &(column_x, column_y) in column.coords() {
							if column_x >= x * 3 && column_x < x * 3 + 3 {
								continue;
							}

							let adjacent_square_index =
								Square::coord_to_index(column_x / 3, column_y / 3) as u8;

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

							let candidates = self
								.grid
								.candidates
								.entry((column_x, column_y))
								.or_default();

							let start = candidates.len();
							candidates.retain(|&c| c != candidate);

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

	/*pub fn square_claim(&mut self) {
		for square in self.grid.squares.iter() {
			let mut only_in_row: HashMap<CellValue, HashSet<u8>> = HashMap::new();

			for row in square.y * 3..square.y * 3 + 3 {
				println!("{}", row);

				for x in square.x * 3..square.x * 3 + 3 {
					for &candidate in self.grid.candidates.entry((x, row)).or_default().iter() {
						only_in_row.entry(candidate).or_default().push(row);
					}
				}
			}

			println!("{:#?}", only_in_row);

			std::process::exit(0);
		}
	}*/

	pub fn cheat(&mut self) -> bool {
		let mut best_square = &self.grid.squares[0];
		for square in self.grid.squares.iter() {
			if (square.cells.len() > best_square.cells.len() && square.cells.len() != 9)
				|| best_square.cells.len() == 0
			{
				best_square = square;
			}
		}

		let mut candidate_coords = (0, 0);
		let mut candidates = vec![];

		for &(x, y) in best_square.coords() {
			let candidates2 = self.grid.candidates.entry((x, y)).or_default();
			if candidates2.len() != 0 {
				candidate_coords = (x, y);
				candidates = candidates2.clone();
				break;
			}
		}

		let mut grids = vec![];

		let (x, y) = candidate_coords;

		for &candidate in candidates.iter() {
			let mut new_grid = self.grid.clone();
			new_grid.insert_number(x, y, candidate);

			grids.push(new_grid);
		}

		for mut grid in grids.iter_mut() {
			let mut analysis = Analysis::new(&mut grid, self.cheating);
			while analysis.round() != 0 && analysis.verify() {
				analysis.draw();

				let mut string = String::new();
				std::io::stdin().read_line(&mut string).unwrap();
			}

			analysis.draw();

			let mut string = String::new();
			std::io::stdin().read_line(&mut string).unwrap();
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
