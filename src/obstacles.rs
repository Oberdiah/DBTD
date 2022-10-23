use cgmath::Point2;
use enum_dispatch::enum_dispatch;
use ggez::graphics::Color;
use ggez::Context;

use crate::{Deserialize, Serialize};

#[enum_dispatch(ObstacleEnum)]
pub trait Obstacle {
	fn render(&self, ctx: &mut Context);
	fn update(&mut self, delta_time: f32);
}

#[enum_dispatch]
#[derive(Clone, Serialize, Deserialize)]
pub enum ObstacleEnum {
	SpinnyCircle,
	MovingLine,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SpinnyCircle {
	pub centre:        Point2<f32>,
	pub current_pos:   f32, // From 0 to 1, where 1 is a full rotation.
	pub chain_length:  u32,
	pub chain_spacing: f32,
	pub circle_size:   f32,
	pub radius:        f32,
	pub speed:         f32,
}

impl Obstacle for SpinnyCircle {
	fn render(&self, ctx: &mut Context) {
		crate::draw_rect_raw(ctx, Color::YELLOW, self.centre, Point2::new(self.radius, self.radius));
		todo!()
	}

	fn update(&mut self, delta_time: f32) {}
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MovingLine {}

impl Obstacle for MovingLine {
	fn render(&self, ctx: &mut Context) {
		todo!()
	}

	fn update(&mut self, delta_time: f32) {
		todo!()
	}
}
