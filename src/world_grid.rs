use std::iter;
use cgmath::Point2;
use crate::{Deserialize, MAP_SIZE_X, MAP_SIZE_Y, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct WorldGrid<T: Clone> {
	grid:  [[T; MAP_SIZE_X]; MAP_SIZE_Y],
}

impl <T: Clone> WorldGrid<T> {
	pub fn new_from(v: T) -> Self {
		Self {
			grid: [[0; MAP_SIZE_X]; MAP_SIZE_Y].map(|a| a.map(|_| v.clone())),
		}
	}

	pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
		&mut self.grid[y][x]
	}

	pub fn iter_mut(&mut self) -> impl Iterator<Item = (Point2<usize>, &mut T)> {
		iter::from_generator(|| {
			yield (Point2::new(0, 0), &mut self.grid[0][0])
		})
	}
}
