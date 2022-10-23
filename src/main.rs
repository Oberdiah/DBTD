#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_attributes)]
#![allow(non_camel_case_types)]
#![feature(iter_from_generator)]
#![feature(generators)]

pub mod common;
pub mod databases;
pub mod game_update;
pub mod login_manager;
pub mod obstacles;
pub mod player;
pub mod world_grid;

use cgmath::{EuclideanSpace, InnerSpace, Point2, Vector2};
use egui::*;
use ggez::conf::WindowMode;
use ggez::event::{self, quit, EventHandler, MouseButton};
use ggez::graphics::{self, draw, Canvas, Color, Drawable, Rect};
use ggez::input::keyboard::KeyCode;
use ggez::input::mouse::position;
use ggez::{Context, ContextBuilder, GameError, GameResult};
use ggez_egui::EguiBackend;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::common::Point2Addons;
use crate::databases::*;
use crate::game_update::update_game;
use crate::login_manager::get_my_name;
use crate::obstacles::*;
use crate::player::Player;
use crate::world_grid::WorldGrid;

pub const MAP_SIZE_X: usize = 25;
pub const MAP_SIZE_Y: usize = 15;

fn main() {
	let mut db = Db::init();

	// Make a Context.
	let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
		.window_mode(WindowMode::default().resizable(true))
		.build()
		.expect("aieee, could not create ggez context!");

	let my_game = MyGame::new(&mut ctx);
	event::run(ctx, event_loop, my_game);
}

#[derive(Clone, Copy, Serialize, Deserialize)]
enum WorldSquare {
	Air,
	Wall,
	Fire,
	Slime,
	StartingSquare,
	GoalSquare,
}

impl WorldSquare {
	pub fn get_color(&self) -> Color {
		match self {
			WorldSquare::Air => Color::WHITE,
			WorldSquare::Wall => Color::from((0.2, 0.1, 0.2)),
			WorldSquare::Fire => Color::from((0.8, 0.2, 0.2)),
			WorldSquare::Slime => Color::from((0.1, 0.5, 0.2)),
			WorldSquare::StartingSquare => Color::from((0.1, 0.1, 0.1)),
			WorldSquare::GoalSquare => Color::GREEN,
		}
	}

	/// In world-space
	pub fn get_rect(pos: Point2<usize>) -> Rect {
		Rect::new(pos.x as f32, pos.y as f32, 1.0, 1.0)
	}
}

pub struct LoadedMap {
	pub game_state: GameState,
	pub game_state_template: GameState,
	pub map_name: String,
	pub owner: String,
	pub best_owner_completion_time: Option<f32>,
	pub current_time: f32,
	pub record_time: f32,
}

impl LoadedMap {
	pub fn new_empty_map(my_name: String, map_name: String) -> LoadedMap {
		let game_state = GameState::new_empty_state();
		LoadedMap {
			game_state: game_state.clone(),
			game_state_template: game_state,
			map_name,
			owner: my_name,
			best_owner_completion_time: None,
			current_time: 0.0,
			record_time: f32::MAX,
		}
	}
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GameState {
	world_grid: WorldGrid<WorldSquare>,
	obstacles:  Vec<ObstacleEnum>,
	player:     Player,
}

impl GameState {
	pub fn new_empty_state() -> Self {
		let mut world_grid = WorldGrid::new_from(WorldSquare::Air);

		for i in 0..7 {
			*world_grid.get_mut(i, i) = WorldSquare::Wall;
			*world_grid.get_mut(i, 6) = WorldSquare::Wall;
		}

		*world_grid.get_mut(3, 9) = WorldSquare::StartingSquare;
		*world_grid.get_mut(8, 10) = WorldSquare::GoalSquare;

		let mut game_state = Self {
			world_grid,
			player: Player::new(),
			obstacles: vec![
				SpinnyCircle::create(Point2::new(3.0, 9.0), 4, 0.2, 0.4, 2.0, 0.3),
				SpinnyCircle::create(Point2::new(8.0, 7.0), 3, 0.7, 1.0, 4.0, 0.15),
			],
		};

		game_state.reset();

		game_state
	}

	pub fn reset(&mut self) {
		for (point, square) in self.world_grid.iter_mut() {
			if let WorldSquare::StartingSquare = square {
				self.player.teleport_to_square(point);
			}
		}
	}
}

struct MyGame {
	loaded_map:   Option<LoadedMap>,
	egui_backend: EguiBackend,
}

impl MyGame {
	pub fn new(ctx: &mut Context) -> MyGame {
		MyGame {
			loaded_map:   Some(LoadedMap::new_empty_map(get_my_name(), "TestMapName".into())),
			egui_backend: EguiBackend::new(ctx),
		}
	}
}

impl EventHandler for MyGame {
	fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
		let egui_ctx = self.egui_backend.ctx();
		egui::Window::new("egui-window").show(&egui_ctx, |ui| {
			ui.label("a very nice gui :3");
			if ui.button("print \"hello world\"").clicked() {
				println!("hello world");
			}
			if ui.button("quit").clicked() {
				quit(ctx);
			}
		});

		if let Some(loaded_map) = &mut self.loaded_map {
			update_game(loaded_map, ctx);
		}

		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		graphics::set_screen_coordinates(
			ctx,
			graphics::Rect::new(0.0, 0.0, graphics::drawable_size(ctx).0, graphics::drawable_size(ctx).1),
		)
		.unwrap();
		graphics::clear(ctx, Color::new(0.2, 0.2, 0.1, 1.0));

		unsafe {
			WINDOW_SIZE = Point2::new(graphics::drawable_size(ctx).0, graphics::drawable_size(ctx).1);
		}

		if let Some(loaded_map) = &mut self.loaded_map {
			let mut game_state = &mut loaded_map.game_state;
			let player_position = game_state.player.position;

			for (point, square) in game_state.world_grid.iter_mut() {
				draw_rect_raw(ctx, square.get_color(), point.to_f32(), Point2::new(1.0, 1.0));
			}
			draw_rect_raw(ctx, Color::RED, player_position, Point2::new(0.5, 0.5));

			for obstacle in &mut game_state.obstacles {
				obstacle.render(ctx);
			}
		}

		draw(ctx, &self.egui_backend, ([0.0, 0.0],))?;

		graphics::present(ctx)
	}

	fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
		self.egui_backend.input.mouse_button_down_event(button);
	}

	fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
		self.egui_backend.input.mouse_button_up_event(button);
	}

	fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
		self.egui_backend.input.mouse_motion_event(x, y);
	}
}

static mut WINDOW_SIZE: Point2<f32> = Point2::new(5.0, 5.0);

pub fn size_of_one_square() -> f32 {
	let window_size = unsafe { WINDOW_SIZE };
	let aspect_ratio = window_size.x / window_size.y;
	let our_aspect_ratio = MAP_SIZE_X as f32 / MAP_SIZE_Y as f32;

	if aspect_ratio < our_aspect_ratio {
		window_size.x / MAP_SIZE_X as f32
	} else {
		window_size.y / MAP_SIZE_Y as f32
	}
}

pub fn game_board_screen_size() -> Point2<f32> {
	Point2::new(
		MAP_SIZE_X as f32 * size_of_one_square(),
		MAP_SIZE_Y as f32 * size_of_one_square(),
	)
}

pub fn screen_offset() -> Vector2<f32> {
	let window_size = unsafe { WINDOW_SIZE };
	let diff = window_size - game_board_screen_size().to_vec();
	Vector2::new(diff.x / 2.0, diff.y / 2.0)
}

pub fn world_space_to_screen_space(world_space: Point2<f32>) -> Point2<f32> {
	world_space * size_of_one_square() + screen_offset()
}

fn draw_rect(ctx: &mut Context, color: Color, world_space_rect: Rect) {
	draw_rect_raw(
		ctx,
		color,
		Point2::new(world_space_rect.x, world_space_rect.y),
		Point2::new(world_space_rect.w, world_space_rect.h),
	);
}

pub fn draw_rect_raw(ctx: &mut Context, color: Color, world_space_pos: Point2<f32>, world_size: Point2<f32>) {
	let position = world_space_to_screen_space(world_space_pos);
	let size = world_size * size_of_one_square();
	let rect = graphics::Rect::new(position.x, position.y, size.x, size.y);

	let rectangle1 = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, color).unwrap();
	graphics::draw(ctx, &rectangle1, ([0.0, 0.0],)).unwrap();
}
