use cgmath::{Point2, Vector2};
use ggez::glam::Vec2;
use crate::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Player {
	/// In grid-space
	pub position: Point2<f32>,

	/// Grid squares per tick
	pub speed: f32,

	/// The player is a rect.
	pub size: Vector2<f32>,
}

impl Player {
	pub fn new() -> Self {
		Player {
			position: Point2::new(2.0, 5.0),
			speed: 0.1,
			size: Vector2::new(0.5, 0.5),
		}
	}
}