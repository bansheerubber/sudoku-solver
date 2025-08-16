use ab_glyph::{FontRef, PxScale};
use image::{Rgb, RgbImage};
use imageproc::{
	drawing::{draw_filled_rect_mut, draw_text_mut},
	rect::Rect,
};

use crate::{
	grid::Grid,
	vec2::{Vec2, SUDOKU},
};

pub struct GridImage<'a> {
	font: FontRef<'a>,
	cell_size: i32,
	grid: &'a Grid,
	image: RgbImage,
	width: u32,
	height: u32,
}

const OFFSETS: [(i32, i32); 9] = [
	(4, 2),
	(28, 2),
	(50, 2),
	(4, 23),
	(28, 23),
	(50, 23),
	(4, 46),
	(28, 46),
	(50, 46),
];

impl<'a> GridImage<'a> {
	pub fn new(grid: &'a Grid) -> Self {
		let font =
			FontRef::try_from_slice(include_bytes!("/usr/share/fonts/TTF/DejaVuSansMono.ttf"))
				.expect("Deja Vu font not found");

		let cell_size = 66;

		let width = cell_size as u32 * 9;
		let height = cell_size as u32 * 9;

		let image = RgbImage::new(width, height);

		GridImage {
			font,
			cell_size,
			grid,
			image,
			width,
			height,
		}
	}

	pub fn create_image(&mut self) {
		let x_adjust = 17;
		let y_adjust = 2;

		draw_filled_rect_mut(
			&mut self.image,
			Rect::at(0, 0).of_size(self.width, self.height),
			Rgb([255u8, 255u8, 255u8]),
		);

		for point in SUDOKU.iter() {
			let number = self.grid.get_number(point);
			if number == 0 {
				continue;
			}

			let color = if self.grid.original_numbers.contains(point) {
				Rgb([0u8, 0u8, 0u8])
			} else if self.grid.invalid_cells.contains(point) {
				Rgb([200u8, 30u8, 30u8])
			} else if self.grid.solution.len() != 0
				&& *self.grid.solution.get(point).unwrap() != number
			{
				Rgb([200u8, 100u8, 30u8])
			} else {
				Rgb([30u8, 200u8, 30u8])
			};

			draw_text_mut(
				&mut self.image,
				color,
				point.x as i32 * self.cell_size + x_adjust,
				point.y as i32 * self.cell_size + y_adjust,
				PxScale {
					x: self.cell_size as f32,
					y: self.cell_size as f32,
				},
				&self.font,
				&format!("{}", number),
			);
		}

		for y in 1..9 {
			if y % 3 == 0 {
				draw_filled_rect_mut(
					&mut self.image,
					Rect::at(0, y as i32 * self.cell_size).of_size(self.width, 2),
					Rgb([0u8, 0u8, 0u8]),
				);
			} else {
				draw_filled_rect_mut(
					&mut self.image,
					Rect::at(0, y as i32 * self.cell_size).of_size(self.width, 1),
					Rgb([100u8, 100u8, 100u8]),
				);
			}
		}

		for x in 1..9 {
			if x % 3 == 0 {
				draw_filled_rect_mut(
					&mut self.image,
					Rect::at(x as i32 * self.cell_size, 0).of_size(2, self.height),
					Rgb([0u8, 0u8, 0u8]),
				);
			} else {
				draw_filled_rect_mut(
					&mut self.image,
					Rect::at(x as i32 * self.cell_size, 0).of_size(1, self.height),
					Rgb([100u8, 100u8, 100u8]),
				);
			}
		}
	}

	fn draw_candidate(&mut self, point: &Vec2, number: u8) {
		let font_size = 20;

		let (x_offset, y_offset) = OFFSETS[number as usize - 1];

		draw_text_mut(
			&mut self.image,
			Rgb([30u8, 30u8, 30u8]),
			point.x as i32 * self.cell_size + x_offset,
			point.y as i32 * self.cell_size + y_offset,
			PxScale {
				x: font_size as f32,
				y: font_size as f32,
			},
			&self.font,
			&format!("{}", number),
		);
	}

	pub fn draw_candidates(&mut self) {
		for point in SUDOKU.iter() {
			for candidate in self.grid.get_candidates(point).iter() {
				self.draw_candidate(point, *candidate);
			}
		}
	}

	pub fn save(&self) {
		self.image.save("sudoku.png").expect("Could not save image");
	}
}
