use crate::{GameState, get_my_name, LoadedMap, WorldSquare};
use ggez::input::keyboard::KeyCode;
use cgmath::{EuclideanSpace, InnerSpace, Point2, Vector2};
use ggez::Context;
use ggez::context::Has;
use ggez::graphics::Rect;

pub fn update_game(loaded_map: &mut LoadedMap, ctx: &mut Context) {
	let mut game_state = &mut loaded_map.game_state;

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
		player_pos_delta = player_pos_delta.normalize() * game_state.player.speed;
	}

	let new_player_position = game_state.player.position + player_pos_delta;
	let new_player_rect = Rect::new(new_player_position.x, new_player_position.y, game_state.player.size.x, game_state.player.size.y);

	let mut should_reset = false;

	for (point, square) in game_state.world_grid.iter_mut() {
		let square_rect = WorldSquare::get_rect(point);

		let does_overlap = new_player_rect.overlaps(&square_rect);

		if does_overlap {
			match square {
				WorldSquare::Air => {}
				WorldSquare::Wall => {
					
				}
				WorldSquare::Fire => should_reset = true,
				WorldSquare::Slime => player_pos_delta *= 0.0,
				WorldSquare::StartingSquare => {}
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

	if should_reset {
		game_state.reset();
	}

	game_state.player.position += player_pos_delta;
}