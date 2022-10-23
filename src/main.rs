#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_attributes)]
#![allow(non_camel_case_types)]
#![feature(iter_from_generator)]
#![feature(generators)]

pub mod world_grid;
pub mod player;
pub mod obstacles;
pub mod databases;
pub mod common;

use cgmath::{EuclideanSpace, InnerSpace, Point2, Vector2};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::conf::WindowMode;
use ggez::graphics::{self, Canvas, Color};
use ggez::event::{self, EventHandler};
use ggez::input::keyboard::KeyCode;
use ggez::input::mouse::position;
use serde::{Deserialize, Serialize};
use crate::common::Point2Addons;
use crate::obstacles::{Obstacle, ObstacleEnum};
use crate::player::Player;
use crate::world_grid::WorldGrid;
use crate::databases::*;

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
    Fire
}

impl WorldSquare {
    pub fn get_color(&self) -> Color {
        match self {
            WorldSquare::Air => Color::WHITE,
            WorldSquare::Wall => Color::from((0.2, 0.1, 0.2)),
            WorldSquare::Fire => Color::from((0.8, 0.2, 0.2)),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct GameState {
    world_grid: WorldGrid<WorldSquare>,
    obstacles: Vec<ObstacleEnum>,
    player: Player,
}

impl GameState {
    pub fn new_empty_state() -> Self {
        let mut world_grid = WorldGrid::new_from(WorldSquare::Air);

        for i in 0..7 {
            *world_grid.get_mut(i, i) = WorldSquare::Wall;
            *world_grid.get_mut(i, 6) = WorldSquare::Wall;
        }

        Self {
            world_grid,
            player: Player::new(),
            obstacles: vec![],
        }
    }
}



struct MyGame {
    game_state: Option<GameState>,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        MyGame {
            game_state: Some(GameState::new_empty_state()),
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if let Some(game_state) = &mut self.game_state {
            let mut player_pos_delta = Vector2::new(0.0, 0.0);
            if ctx.keyboard.is_key_pressed(KeyCode::W) {
                player_pos_delta.y -= 1.0;
            }
            if ctx.keyboard.is_key_pressed(KeyCode::A) {
                player_pos_delta.x -= 1.0;
            }
            if ctx.keyboard.is_key_pressed(KeyCode::S) {
                player_pos_delta.y += 1.0;
            }
            if ctx.keyboard.is_key_pressed(KeyCode::D) {
                player_pos_delta.x += 1.0;
            }

            if player_pos_delta.magnitude() > 0.0 {
                game_state.player.position += player_pos_delta.normalize() * game_state.player.speed;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        unsafe { WINDOW_SIZE = Point2::new(ctx.gfx.drawable_size().0, ctx.gfx.drawable_size().1); }

        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::CanvasLoadOp::Clear([0.1, 0.2, 0.3, 1.0].into()),
        );

        if let Some(game_state) = &mut self.game_state {
            let player_position = game_state.player.position;

            for (point, square) in game_state.world_grid.iter_mut() {
                draw_rect(&mut canvas, point.to_f32(), Point2::new(1.0, 1.0), square.get_color());
            }
            draw_rect(&mut canvas, player_position, Point2::new(0.5, 0.5), Color::RED);
        }



        canvas.finish(ctx)?;

        Ok(())
    }
}

static mut WINDOW_SIZE: Point2<f32> = Point2::new(5.0, 5.0);

pub fn size_of_one_square() -> f32 {
    let window_size = unsafe {WINDOW_SIZE};
    let aspect_ratio = window_size.x / window_size.y;
    let our_aspect_ratio = MAP_SIZE_X as f32 / MAP_SIZE_Y as f32;

    if aspect_ratio < our_aspect_ratio {
        window_size.x / MAP_SIZE_X as f32
    } else {
        window_size.y / MAP_SIZE_Y as f32
    }
}

pub fn game_board_screen_size() -> Point2<f32> {
    Point2::new(MAP_SIZE_X as f32 * size_of_one_square(), MAP_SIZE_Y as f32 * size_of_one_square())
}

pub fn screen_offset() -> Vector2<f32> {
    let window_size = unsafe {WINDOW_SIZE};
    let diff = window_size - game_board_screen_size().to_vec();
    Vector2::new(diff.x / 2.0, diff.y / 2.0)
}

pub fn world_space_to_screen_space(world_space: Point2<f32>) -> Point2<f32> {
    world_space * size_of_one_square() + screen_offset()
}

fn draw_rect(canvas: &mut Canvas, world_space_pos: Point2<f32>, world_size: Point2<f32>, color: Color) {
    let position = world_space_to_screen_space(world_space_pos);
    let size = world_size * size_of_one_square();
    let rect = graphics::Rect::new(position.x, position.y, size.x, size.y);

    canvas.draw(
        &graphics::Quad,
        graphics::DrawParam::new()
            .dest(rect.point())
            .scale(rect.size())
            .color(color),
    );
}