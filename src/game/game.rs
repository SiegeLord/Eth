use allegro5::*;
use allegro_font::*;

use ces::Entities;
use ces::components::{State, GameMode, MenuMode, Components, ComponentType};
use ces::system::System;

simple_system!
(
	GameLogicSystem[GameMode, State]
	{
		let _ = entity_idx;
		let _ = entities;
		let _ = components;
	}
)

simple_system!
(
	GameInputSystem[GameMode, State]
	{
		let mut switch = false;
		{
			let e = entities.get(entity_idx);
			let state = e.get_mut(&mut components.state).unwrap();
			
			state.key_down.map(|k|
			{
				match k
				{
					key::Escape =>
					{
						switch = true
					}
					_ => ()
				}
			});
		}
		
		if switch
		{
			let menu_mode = 
			{
				let (state, mode) = 
				{
					let e = entities.get(entity_idx);
					(e.get_mut(&mut components.state).unwrap(),
				     e.get_mut(&mut components.game_mode).unwrap())
				};
				entities.sched_remove(mode.player_entity);
				MenuMode::new(state)
			};
			components.add(entity_idx, menu_mode, entities);
			components.sched_remove::<GameMode>(entity_idx, entities);
		}
	}
)

simple_system!
(
	GameDrawSystem[GameMode, State]
	{
		let e = entities.get(entity_idx);
		let core = &e.get(&mut components.state).unwrap().core;

		core.clear_to_color(core.map_rgb_f(0.0, 0.0, 0.0));
	}
)
