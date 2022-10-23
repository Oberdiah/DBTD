use cgmath::{EuclideanSpace, InnerSpace, Point2, Vector2};
use ggez::graphics::Rect;
use ggez::input::keyboard::KeyCode;
use ggez::{input, Context};

use crate::obstacles::*;
use crate::{get_my_name, GameState, LoadedMap, Player, WorldSquare};

pub fn update_game(loaded_map: &mut LoadedMap, ctx: &mut Context) {
	let mut game_state = &mut loaded_map.game_state;

	// Call update on each obstacle:
	for obstacle in &mut game_state.obstacles {
		obstacle.update(1. / 60.);
	}

	let mut player_pos_delta = Vector2::new(0.0, 0.0);

	if input::keyboard::is_key_pressed(ctx, KeyCode::W) {
		player_pos_delta.y -= 1.0;
	}
	if input::keyboard::is_key_pressed(ctx, KeyCode::A) {
		player_pos_delta.x -= 1.0;
	}
	if input::keyboard::is_key_pressed(ctx, KeyCode::S) {
		player_pos_delta.y += 1.0;
	}
	if input::keyboard::is_key_pressed(ctx, KeyCode::D) {
		player_pos_delta.x += 1.0;
	}

	if player_pos_delta.magnitude() > 0.0 {
		player_pos_delta = player_pos_delta.normalize() * game_state.player.speed;
	}

	let new_player_rect = get_player_rect(&game_state.player, player_pos_delta);
	let mut should_reset = false;

	for (point, square) in game_state.world_grid.iter_mut() {
		let square_rect = WorldSquare::get_rect(point);

		let does_overlap = new_player_rect.overlaps(&square_rect);

		if does_overlap {
			match square {
				WorldSquare::Air => {}
				WorldSquare::Wall => {
					// Now split into x and y, and check again. For wall sliding. (loh)
					let x_moved_player =
						get_player_rect(&game_state.player, Vector2::new(player_pos_delta.x, 0.0));
					let y_moved_player =
						get_player_rect(&game_state.player, Vector2::new(0.0, player_pos_delta.y));
					if x_moved_player.overlaps(&square_rect) {
						player_pos_delta.x = 0.0;
					}
					if y_moved_player.overlaps(&square_rect) {
						player_pos_delta.y = 0.0;
					}
				}
				WorldSquare::Fire => should_reset = true,
				WorldSquare::Slime => player_pos_delta *= 0.0,
				WorldSquare::GoalSquare => {
					if get_my_name() == loaded_map.owner {
						game_state.best_owner_completion_time = Some(game_state.current_time);
					} else {
						if let Some(completion_time) = game_state.best_owner_completion_time {
							if game_state.current_time < completion_time {
								println!("You beat the time!");
							}
						}
					}
					game_state.record_time = game_state.record_time.max(game_state.current_time);
					should_reset = true;
				}
			}
		}
	}

	game_state.player.position += player_pos_delta;

	// Now see if player dies.
	for obstacle in &game_state.obstacles {
		if obstacle.does_player_die(&game_state.player) {
			should_reset = true;
			println!("RIP");
		}
	}

	if should_reset {
		loaded_map.game_state = loaded_map.game_state_template.clone();
	}
}
pub fn get_player_rect(player: &Player, position_delta: Vector2<f32>) -> Rect {
	Rect::new(
		player.position.x + position_delta.x,
		player.position.y + position_delta.y,
		player.size.x,
		player.size.y,
	)
}
