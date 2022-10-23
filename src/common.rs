use cgmath::Point2;
use ggez::graphics::Rect;

pub trait Point2Addons {
	fn to_f32(&self) -> Point2<f32>;
}

impl Point2Addons for Point2<usize> {
	fn to_f32(&self) -> Point2<f32> {
		Point2::new(self.x as f32, self.y as f32)
	}
}

pub trait RectAddons {
	fn top_left(&self) -> Point2<f32>;
	fn top_right(&self) -> Point2<f32>;
	fn bottom_left(&self) -> Point2<f32>;
	fn bottom_right(&self) -> Point2<f32>;
}

impl RectAddons for Rect {
	fn top_left(&self) -> Point2<f32> {
		Point2::new(self.left(), self.top())
	}

	fn top_right(&self) -> Point2<f32> {
		Point2::new(self.right(), self.top())
	}

	fn bottom_left(&self) -> Point2<f32> {
		Point2::new(self.left(), self.bottom())
	}

	fn bottom_right(&self) -> Point2<f32> {
		Point2::new(self.right(), self.bottom())
	}
}