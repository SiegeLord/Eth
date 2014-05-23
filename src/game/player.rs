use allegro5::*;
use ces::Entities;
use ces::components::{Components, Location, Velocity, Acceleration, Size, Player, OldLocation};
use ces::components::ComponentType;
use ces::system::System;
use MODE_ENTITY;

pub fn create_player(appearance: uint, x: f32, y: f32, entities: &mut Entities, components: &mut Components) -> uint
{
	let player = 
	{
		let state = entities.get(MODE_ENTITY).get_mut(&mut components.state).unwrap();
		Player::new(appearance, state)
	};
	
	let e = entities.add();
	components.add(e, Location{ x: x, y: y }, entities);
	components.add(e, OldLocation{ x: x, y: y }, entities);
	components.add(e, Velocity{ vx: 0.0, vy: 0.0 }, entities);
	components.add(e, Acceleration{ ax: 0.0, ay: 0.0 }, entities);
	components.add(e, Acceleration{ ax: 0.0, ay: 0.0 }, entities);
	components.add(e, Size{ w: 16.0, h: 16.0 }, entities);
	components.add(e, player, entities);
	e
}

simple_system!
(
	PlayerDrawSystem[Player, Location, OldLocation]
	{
		let mode_e = entities.get(MODE_ENTITY);
		let e = entities.get(entity_idx);
		
		let player = e.get(&mut components.player).unwrap();
		let l = e.get(&mut components.location).unwrap();
		let o = e.get(&mut components.old_location).unwrap();
		
		let state = mode_e.get(&mut components.state).unwrap();
		let core = &state.core;
		
		let x = l.x + (l.x - o.x) * state.draw_interp;
		let y = l.y + (l.y - o.y) * state.draw_interp;
		
		core.draw_bitmap(&*player.bmp, x, y, Flag::zero());
	}
)

simple_system!
(
	PlayerInputSystem[Player, Acceleration]
	{
		let mode_e = entities.get(MODE_ENTITY);
		let e = entities.get(entity_idx);
		let player = e.get_mut(&mut components.player).unwrap();
		let acceleration = e.get_mut(&mut components.acceleration).unwrap();
		
		let state = mode_e.get_mut(&mut components.state).unwrap();
		
		state.key_down.map(|k|
		{
			match k
			{
				key::Up =>
				{
					player.up = 1.0;
				}
				key::Down =>
				{
					player.down = 1.0;
				}
				key::Left =>
				{
					player.left = 1.0;
				}
				key::Right =>
				{
					player.right = 1.0;
				}
				_ => ()
			}
		});

		state.key_up.map(|k|
		{
			match k
			{
				key::Up =>
				{
					player.up = 0.0;
				}
				key::Down =>
				{
					player.down = 0.0;
				}
				key::Left =>
				{
					player.left = 0.0;
				}
				key::Right =>
				{
					player.right = 0.0;
				}
				_ => ()
			}
		});
		
		acceleration.ax = 0.05 * (player.right - player.left);
		acceleration.ay = 0.05 * (player.down - player.up);
	}
)
