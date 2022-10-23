use cgmath::Point2;

pub trait Point2Addons {
	fn to_f32(&self) -> Point2<f32>;
}

impl Point2Addons for Point2<usize> {
	fn to_f32(&self) -> Point2<f32> {
		Point2::new(self.x as f32, self.y as f32)
	}
}