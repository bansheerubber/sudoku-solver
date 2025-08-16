use crate::analysis::DEBUG;

mod analysis;
mod grid;
mod grid_image;
mod line;
mod mini_line;
mod square;
mod vec2;

fn main() {
	let mut grid = grid::Grid::new();
	grid.load();
	grid.verify_data_structure();

	let mut analysis = analysis::Analysis::new(&mut grid, false);
	analysis.calculate_all_candidates();

	if DEBUG {
		analysis.draw();

		let mut string = String::new();
		std::io::stdin().read_line(&mut string).unwrap();
	}

	while analysis.round() != 0 {
		if DEBUG {
			analysis.draw();

			let mut string = String::new();
			std::io::stdin().read_line(&mut string).unwrap();
		}
	}

	analysis.draw();
}
