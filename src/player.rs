use cgmath::Point2;
use crate::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Player {
	pub position: Point2<f32>,
	// Grid squares per tick
	pub speed: f32,
}

impl Player {
	pub fn new() -> Self {
		Player {
			position: Point2::new(5.0, 5.0),
			speed: 0.1,
		}
	}
}