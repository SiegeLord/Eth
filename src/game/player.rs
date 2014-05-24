use allegro5::*;
use ces::Entities;
use ces::components::{Components, Location, Velocity, Acceleration, Size, Player, Sprite, Mass, OldLocation};
use ces::components::ComponentType;
use ces::system::System;
use MODE_ENTITY;

pub fn create_player(appearance: i32, fuel: f64, x: f64, y: f64, vx: f64, vy: f64, entities: &mut Entities, components: &mut Components) -> uint
{
	let sprite = 
	{
		let state = entities.get(MODE_ENTITY).get_mut(&mut components.state).unwrap();
		Sprite::new(format!("data/planet{}.png", appearance).as_slice(), state)
	};
	
	let e = entities.add();
	components.add(e, Location{ x: x, y: y }, entities);
	components.add(e, OldLocation{ x: x, y: y }, entities);
	components.add(e, Velocity{ vx: vx, vy: vy }, entities);
	components.add(e, Acceleration{ ax: 0.0, ay: 0.0 }, entities);
	components.add(e, Size{ d: 16.0 }, entities);
	components.add(e, Mass{ mass: 0.0 }, entities);
	components.add(e, sprite, entities);
	components.add(e, Player::new(fuel), entities);
	e
}

simple_system!
(
	PlayerInputSystem[Player]
	{
		let mode_e = entities.get(MODE_ENTITY);
		let e = entities.get(entity_idx);
		let player = e.get_mut(&mut components.player).unwrap();
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
	}
)

simple_system!
(
	PlayerLogicSystem[Player, Acceleration]
	{
		let e = entities.get(entity_idx);
		let player = e.get_mut(&mut components.player).unwrap();
		let a = e.get_mut(&mut components.acceleration).unwrap();
		
		if player.fuel > 0.0 && (player.right > 0.0 || player.left > 0.0 || player.down > 0.0 || player.up > 0.0)
		{
			a.ax += 0.01 * (player.right - player.left);
			a.ay += 0.01 * (player.down - player.up);
			player.fuel -= 1.0;
			if player.fuel < 0.0
			{
				player.fuel = 0.0;
			}
		}
	}
)
