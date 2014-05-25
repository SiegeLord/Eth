use allegro5::*;
use ces::Entities;
use ces::components::{Components, Location, Velocity, Acceleration, Size, Player, Sprite, Mass, OldLocation};
use ces::components::ComponentType;
use ces::system::System;
use MODE_ENTITY;

pub fn create_player(appearance: i32, fuel: f64, x: f64, y: f64, vx: f64, vy: f64, entities: &mut Entities, components: &mut Components) -> uint
{
	let (sprite, player) = 
	{
		let state = entities.get(MODE_ENTITY).get_mut(&mut components.state).unwrap();
		(Sprite::new(format!("data/planet{}.png", appearance).as_slice(), state),
		 Player::new(fuel, state))
	};
	
	let e = entities.add();
	components.add(e, Location{ x: x, y: y }, entities);
	components.add(e, OldLocation{ x: x, y: y }, entities);
	components.add(e, Velocity{ vx: vx, vy: vy }, entities);
	components.add(e, Acceleration{ ax: 0.0, ay: 0.0 }, entities);
	components.add(e, Size{ d: 16.0 }, entities);
	components.add(e, Mass{ mass: 0.0 }, entities);
	components.add(e, sprite, entities);
	components.add(e, player, entities);
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
	PlayerDrawSystem[Player, Location, OldLocation, Size]
	{
		let mode_e = entities.get(MODE_ENTITY);
		let e = entities.get(entity_idx);
		
		let player = e.get(&components.player).unwrap();
		let l = e.get(&mut components.location).unwrap();
		let o = e.get(&mut components.old_location).unwrap();
		let z = e.get(&mut components.size).unwrap();
		
		let state = mode_e.get(&mut components.state).unwrap();
		let core = &state.core;
		
		fn draw_bmp(x: f64, y: f64, z: &Size, bmp: &Bitmap, core: &Core)
		{
			let x = x + (z.d - bmp.get_width() as f64) / 2.0;
			let y = y + (z.d - bmp.get_height() as f64) / 2.0;
			core.draw_bitmap(bmp, x as f32, y as f32, Flag::zero());
		}

		let x = l.x + (l.x - o.x) * state.draw_interp;
		let y = l.y + (l.y - o.y) * state.draw_interp;
		
		if player.fuel > 0.0
		{
			if player.up > 0.0
			{
				draw_bmp(x, y, z, &*player.up_spr, core);
			}
			if player.down > 0.0
			{
				draw_bmp(x, y, z, &*player.down_spr, core);
			}
			if player.left > 0.0
			{
				draw_bmp(x, y, z, &*player.left_spr, core);
			}
			if player.right > 0.0
			{
				draw_bmp(x, y, z, &*player.right_spr, core);
			}
		}
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
