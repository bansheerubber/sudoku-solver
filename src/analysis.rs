use std::collections::{HashMap, HashSet};

use crate::{
	grid::{CellValue, Coord, Grid},
	grid_image::GridImage,
	line::LineDirection,
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

		if self.square_claim() {
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
		let mut single_line_candidates: HashMap<
			usize,
			HashMap<(LineDirection, CellValue), HashSet<Coord>>,
		> = HashMap::new();

		for x in 0..3 {
			for y in 0..3 {
				let square_index = Square::square_coord_to_index(x, y);

				for y2 in y * 3..y * 3 + 3 {
					for x2 in x * 3..x * 3 + 3 {
						for &candidate in self.grid.get_candidates(&Vec2::new(x2, y2)).iter() {
							single_line_candidates
								.entry(square_index)
								.or_default()
								.entry((LineDirection::Row, candidate))
								.or_default()
								.insert(y2);

							single_line_candidates
								.entry(square_index)
								.or_default()
								.entry((LineDirection::Column, candidate))
								.or_default()
								.insert(x2);
						}
					}
				}
			}
		}

		let mut changes_made = false;
		let mut candidates_to_remove = vec![];

		for square in self.grid.squares.iter() {
			let square_index = Square::square_coord_to_index(square.x, square.y);
			let Some(single_line_candidates_for_square) = single_line_candidates.get(&square_index)
			else {
				continue;
			};

			for (&(direction, candidate), set) in single_line_candidates_for_square.iter() {
				if set.len() != 1 || set.len() == 0 {
					continue;
				}

				let rank = *set.iter().nth(0).unwrap() as usize;
				let line = match direction {
					LineDirection::Row => &self.grid.rows[rank],
					LineDirection::Column => &self.grid.columns[rank],
				};

				for point in line.coords() {
					match direction {
						LineDirection::Row => {
							if point.x >= square.x * 3 && point.x < square.x * 3 + 3 {
								continue;
							}
						}
						LineDirection::Column => {
							if point.y >= square.y * 3 && point.y < square.y * 3 + 3 {
								continue;
							}
						}
					}

					let adjacent_square_index = Square::point_to_index(point);

					let Some(candidate_map) = single_line_candidates.get(&adjacent_square_index)
					else {
						continue;
					};

					let Some(row_set) = candidate_map.get(&(direction, candidate)) else {
						continue;
					};

					if row_set.len() <= 1 {
						continue;
					}

					candidates_to_remove.push((*point, candidate));
				}
			}
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

		return changes_made;
	}

	pub fn square_claim(&mut self) -> bool {
		let mut exclusives = HashMap::new();

		for line in self.grid.lines() {
			for number in 1..=9 {
				for mini_line in line.mini_lines.iter() {
					if !mini_line.has_candidate_anywhere(number) {
						continue;
					}

					if let Some(square_rank) =
						exclusives.get(&(line.rank(), line.direction, number))
					{
						if let Some(square_rank) = square_rank
							&& *square_rank != mini_line.square_rank()
						{
							exclusives.insert((line.rank(), line.direction, number), None);
						}
					} else {
						exclusives.insert(
							(line.rank(), line.direction, number),
							Some(mini_line.square_rank()),
						);
					}
				}
			}
		}

		let mut changes_made = false;

		for ((rank, direction, number), square_rank) in exclusives {
			let Some(square_rank) = square_rank else {
				continue;
			};

			let start_rank = (rank / 3) * 3;
			let end_rank = start_rank + 3;

			for i in start_rank..end_rank {
				if i == rank {
					continue;
				}

				for candidate_index in 0..3 {
					let point = match direction {
						LineDirection::Row => Vec2::new(square_rank * 3 + candidate_index, i),
						LineDirection::Column => Vec2::new(i, square_rank * 3 + candidate_index),
					};

					if self.grid.remove_candidate(&point, number) {
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
