use ces::Entities;
use ces::components::{Components, Location, Solid, Size, Sprite, Player, Acceleration};
use ces::components::ComponentType;
use ces::system::System;
use MODE_ENTITY;

simple_system!
(
	CollisionLogicSystem[Solid, Location, Size]
	{
		let player_entity =
		{
			let mode_e = entities.get(MODE_ENTITY);
			let game_mode = mode_e.get_mut(&mut components.game_mode).unwrap();
			game_mode.player_entity
		};
		
		let mut collided = false;
		{
			let e = entities.get(entity_idx);
			
			let l = e.get(&components.location).unwrap();
			let z = e.get(&components.size).unwrap();
			
			let player_e = entities.get(player_entity);
			let player_l = player_e.get(&components.location).unwrap();
			let player_z = player_e.get(&components.size).unwrap();
			
			let dx = (player_l.x + player_z.d / 2.0) - (l.x + z.d / 2.0);
			let dy = (player_l.y + player_z.d / 2.0) - (l.y + z.d / 2.0);
			let d = (player_z.d + z.d) / 2.0;
			if dx * dx + dy * dy < d * d && player_e.get(&components.player).is_some()
			{
				collided = true;
			}
		};
		
		if collided
		{
			components.sched_remove::<Player>(player_entity, entities);
			components.sched_remove::<Acceleration>(player_entity, entities);
			let player_e = entities.get(player_entity);
			let mode_e = entities.get(MODE_ENTITY);
			
			let state = mode_e.get_mut(&mut components.state).unwrap();
			let player_s = player_e.get_mut(&mut components.sprite).unwrap();
			*player_s = Sprite::new("data/explosion.cfg", true, state);
		}
	}
)
