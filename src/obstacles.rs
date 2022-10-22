use crate::{Deserialize, Serialize};
use enum_dispatch::enum_dispatch;

#[enum_dispatch(ObstacleEnum)]
pub trait Obstacle {
	fn render(&self);
}

#[enum_dispatch]
#[derive(Clone, Serialize, Deserialize)]
pub enum ObstacleEnum {
	SpinnyCircle,
	MovingLine,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SpinnyCircle {
}

impl Obstacle for SpinnyCircle {
	fn render(&self) {
		todo!()
	}
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MovingLine {
}

impl Obstacle for MovingLine {
	fn render(&self) {
		todo!()
	}
}