use ggez::Context;
use ggez::graphics::spritebatch::SpriteBatch;

pub struct CoolContext<'a> {
	pub ctx: &'a mut Context,
	pub sprite_batch: &'a mut SpriteBatch,
}