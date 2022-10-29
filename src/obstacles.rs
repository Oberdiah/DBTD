use cgmath::{EuclideanSpace, Point2};
use enum_dispatch::enum_dispatch;
use ggez::graphics::{Color, Rect};
use ggez::Context;

use crate::cool_context::CoolContext;
use crate::{Deserialize, Player, Serialize};

#[enum_dispatch(ObstacleEnum)]
pub trait Obstacle {
	fn render(&self, ctx: &mut CoolContext);
	fn update(&mut self, delta_time: f32);
	fn does_player_die(&self, player: &Player) -> bool;
}

#[enum_dispatch]
#[derive(Clone, Serialize, Deserialize)]
pub enum ObstacleEnum {
	SpinnyCircle,
	MovingLine,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SpinnyCircle {
	pub parent_position: Point2<f32>,
	pub current_time:    f32, // From 0 to 1, where 1 is a full rotation.
	pub child_count:     u32,
	pub child_spacing:   f32,
	pub child_radius:    f32,
	pub parent_radius:   f32,
	pub child_speed:     f32,
}
impl SpinnyCircle {
	pub fn create(
		position: Point2<f32>,
		chain_length: u32,
		chain_spacing: f32,
		circle_size: f32,
		radius: f32,
		speed: f32,
	) -> ObstacleEnum {
		let mut spinny = SpinnyCircle {
			parent_position: position,
			current_time:    0.0,
			child_count:     chain_length,
			child_spacing:   chain_spacing,
			child_radius:    circle_size,
			parent_radius:   radius,
			child_speed:     speed,
		};
		let e = ObstacleEnum::from(spinny);
		return e;
	}
}

impl Obstacle for SpinnyCircle {
	fn render(&self, ctx: &mut CoolContext) {
		for child_index in 0..self.child_count {
			let angle =
				child_index as f32 * self.child_spacing + self.current_time * 2.0 * std::f32::consts::PI;
			let child_position = Point2::new(
				self.parent_position.x + self.parent_radius * angle.cos(),
				self.parent_position.y + self.parent_radius * angle.sin(),
			);

			crate::draw_rect_raw(
				ctx,
				Color::MAGENTA,
				child_position,
				Point2::new(self.child_radius, self.child_radius),
			)
		}
	}

	fn does_player_die(&self, player: &Player) -> bool {
		let mut died = false;
		for child_index in 0..self.child_count {
			let angle =
				child_index as f32 * self.child_spacing + self.current_time * 2.0 * std::f32::consts::PI;
			let child_position = Point2::new(
				self.parent_position.x + self.parent_radius * angle.cos(),
				self.parent_position.y + self.parent_radius * angle.sin(),
			);
			if player.get_rect().overlaps(&Rect::new(
				child_position.x,
				child_position.y,
				self.child_radius,
				self.child_radius,
			)) {
				died = true;
			};
		}
		return died;
	}

	fn update(&mut self, delta_time: f32) {
		self.current_time += self.child_speed * delta_time;
		if self.current_time > 1.0 {
			self.current_time -= 1.0;
		}
	}
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MovingLine {}

impl Obstacle for MovingLine {
	fn render(&self, ctx: &mut CoolContext) {
		todo!()
	}

	fn update(&mut self, delta_time: f32) {
		todo!()
	}

	fn does_player_die(&self, player: &Player) -> bool {
		false
	}
}
