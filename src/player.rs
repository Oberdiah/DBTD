use cgmath::{Point2, Vector2};
use ggez::glam::Vec2;
use ggez::graphics::Rect;
use crate::{Deserialize, Point2Addons, Serialize};

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

	pub fn get_rect(&self) -> Rect {
		Rect::new(self.position.x, self.position.y, self.size.x, self.size.y)
	}

	pub fn teleport_to_square(&mut self, square: Point2<usize>) {
		self.position.x = square.to_f32().x + (1.0 - self.size.x) / 2.0;
		self.position.y = square.to_f32().y + (1.0 - self.size.y) / 2.0;
	}
}