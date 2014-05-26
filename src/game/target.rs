// Copyright 2014 SiegeLord
// Licensed under GPL, see LICENSE for full terms

use allegro5::*;
use ces::Entities;
use ces::components::{Components, Location, OldLocation, Size, Target, Switchable, Sprite};
use ces::components::ComponentType;
use ces::system::System;
use MODE_ENTITY;

pub fn create_target(x: f64, y: f64, appearance: i32, entities: &mut Entities, components: &mut Components) -> uint
{
	let (sprite, target) = 
	{
		let state = entities.get(MODE_ENTITY).get_mut(&mut components.state).unwrap();
		(Sprite::new(format!("data/spaceship{}.cfg", appearance).as_slice(), false, state),
		 Target::new(state))
	};
	
	let e = entities.add();
	components.add(e, Location{ x: x, y: y }, entities);
	components.add(e, OldLocation{ x: x, y: y }, entities);
	components.add(e, Size{ d: 16.0 }, entities);
	components.add(e, Switchable{ dummy: () }, entities);
	components.add(e, sprite, entities);
	components.add(e, target, entities);
	e
}

simple_system!
(
	TargetReticleDrawSystem[Target, Location, OldLocation, Size]
	{
		let mode_e = entities.get(MODE_ENTITY);
		let e = entities.get(entity_idx);
		
		let target = e.get(&components.target).unwrap();
		let l = e.get(&components.location).unwrap();
		let o = e.get(&components.old_location).unwrap();
		let z = e.get(&components.size).unwrap();
		
		let state = mode_e.get(&components.state).unwrap();
		mode_e.get(&components.game_mode).map(|game_mode|
		{
			let player_e = entities.get(game_mode.player_entity);
			let player_l = player_e.get(&components.location).unwrap();
			let player_z = player_e.get(&components.size).unwrap();
			
			let dx = (player_l.x + player_z.d / 2.0) - (l.x + z.d / 2.0);
			let dy = (player_l.y + player_z.d / 2.0) - (l.y + z.d / 2.0);
			let bmp = if dx * dx + dy * dy < game_mode.range * game_mode.range
			{
				&target.reticle_near
			}
			else
			{
				&target.reticle_far
			};
			
			let core = &state.core;

			let x = l.x + (l.x - o.x) * state.draw_interp + (z.d - bmp.get_width() as f64) / 2.0;
			let y = l.y + (l.y - o.y) * state.draw_interp + (z.d - bmp.get_height() as f64) / 2.0;
			
			bmp.draw(x as f32, y as f32, core);
		});
	}
)

simple_system!
(
	TargetInputSystem[Target, Location, Size]
	{
		let mut remove = false;
		{
			let e = entities.get(entity_idx);
			
			let l = e.get(&components.location).unwrap();
			let z = e.get(&components.size).unwrap();
			
			let mode_e = entities.get(MODE_ENTITY);
			let state = mode_e.get_mut(&mut components.state).unwrap();
			let game_mode = mode_e.get_mut(&mut components.game_mode).unwrap();
			let player_e = entities.get(game_mode.player_entity);
			let player_l = player_e.get(&components.location).unwrap();
			let player_z = player_e.get(&components.size).unwrap();
			let player = player_e.get(&components.player);
			
			state.key_down.map(|k|
			{
				match k
				{
					key::Space =>
					{
						let dx = (player_l.x + player_z.d / 2.0) - (l.x + z.d / 2.0);
						let dy = (player_l.y + player_z.d / 2.0) - (l.y + z.d / 2.0);
						if dx * dx + dy * dy < game_mode.range * game_mode.range
						{
							remove = true;
							player.map(|player|
							{
								state.sfx.play(&*player.camera_sound, &state.audio);
							});
						}
					}
					_ => ()
				}
			})
		};
		
		if remove
		{
			components.sched_remove::<Target>(entity_idx, entities);
			let mode_e = entities.get(MODE_ENTITY);
			let game_mode = mode_e.get_mut(&mut components.game_mode).unwrap();
			game_mode.targets -= 1;
		}
	}
)

