use allegro5::*;
use allegro_font::*;

use ces::Entities;
use ces::components::{State, GameMode, MenuMode, IntermissMode, Components, ComponentType};
use ces::system::System;
use DT;

simple_system!
(
	IntermissLogicSystem[IntermissMode, State]
	{
		let e = entities.get(entity_idx);
		let mode = e.get_mut(&mut components.intermiss_mode).unwrap();
		let state = e.get_mut(&mut components.state).unwrap();
	}
)

simple_system!
(
	IntermissInputSystem[IntermissMode, State]
	{
		let mut switch = false;
		let mut next = false;
		{
			let e = entities.get(entity_idx);
			let state = e.get_mut(&mut components.state).unwrap();
			let intermiss_mode = e.get_mut(&mut components.intermiss_mode).unwrap();
			
			state.key_down.map(|k|
			{
				match k
				{
					key::Escape =>
					{
						switch = true
					}
					key::Space =>
					{
						if intermiss_mode.next.is_some()
						{
							next = true;
						}
					}
					_ => ()
				}
			});
		}

		if switch
		{
			let menu_mode = 
			{
				let (state, _mode) = 
				{
					let e = entities.get(entity_idx);
					(e.get_mut(&mut components.state).unwrap(),
				     e.get_mut(&mut components.intermiss_mode).unwrap())
				};
				state.stopped = false;
				MenuMode::new(state)
			};
			components.add(entity_idx, menu_mode, entities);
			components.sched_remove::<IntermissMode>(entity_idx, entities);
		}
		else if next
		{
			let next = {
				let e = entities.get(entity_idx);
				e.get_mut(&mut components.intermiss_mode).unwrap().next.as_ref().unwrap().clone()
			};
			let game_mode = 
			{
				let (set, score, high_score, max_fuel, range, appearance) = 
				{
					let e = entities.get(entity_idx);
					let state = e.get_mut(&mut components.state).unwrap();
					let mode = e.get_mut(&mut components.intermiss_mode).unwrap();
					state.stopped = false;
					state.paused = true;
					(mode.set.clone(), mode.score, mode.high_score, mode.max_fuel, mode.range, state.appearance)
				};
				GameMode::new(set.as_slice(), next.as_slice(),
							  score, high_score, max_fuel, range, appearance, entities, components)
			};
			components.add(entity_idx, game_mode, entities);
			
			components.sched_remove::<IntermissMode>(entity_idx, entities);
		}
	}
)

simple_system!
(
	IntermissDrawSystem[IntermissMode, State]
	{
		let e = entities.get(entity_idx);
		let core = &e.get(&mut components.state).unwrap().core;

		core.clear_to_color(core.map_rgb_f(0.2, 0.0, 0.0));
	}
)

