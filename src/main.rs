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

use cgmath::{InnerSpace, Point2, Vector2};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Canvas, Color};
use ggez::event::{self, EventHandler};
use ggez::input::keyboard::KeyCode;
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

        *world_grid.get_mut(5, 5) = WorldSquare::Wall;

        Self {
            world_grid,
            player: Player::new(),
            obstacles: vec![],
        }
    }
}



struct MyGame {
    game_state: GameState,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        MyGame {
            game_state: GameState::new_empty_state(),
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let player_speed = self.game_state.player.speed;
        let mut player_pos_delta = Vector2::new(0.0, 0.0);
        if ctx.keyboard.is_key_pressed(KeyCode::W) {
            player_pos_delta.y -= player_speed;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::A) {
            player_pos_delta.x -= player_speed;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::S) {
            player_pos_delta.y += player_speed;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::D) {
            player_pos_delta.x += player_speed;
        }

        if player_pos_delta.magnitude() > 0.0 {
            self.game_state.player.position += player_pos_delta.normalize();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::CanvasLoadOp::Clear([0.1, 0.2, 0.3, 1.0].into()),
        );

        let player_position = self.game_state.player.position;

        for (point, square) in self.game_state.world_grid.iter_mut() {
            draw_rect(&mut canvas, point.to_f32(), Point2::new(1.0, 1.0), square.get_color());
        }

        canvas.finish(ctx)?;

        Ok(())
    }
}

fn draw_rect(canvas: &mut Canvas, world_space_pos: Point2<f32>, world_size: Point2<f32>, color: Color) {
    let player_position = world_space_pos * 5.0;

    let rect = graphics::Rect::new(player_position.x, player_position.y, world_size.x, world_size.y);
    canvas.draw(
        &graphics::Quad,
        graphics::DrawParam::new()
            .dest(rect.point())
            .scale(rect.size())
            .color(color),
    );
}