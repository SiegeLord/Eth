use allegro5::*;
use allegro_font::*;

use ces::Entities;
use ces::components::{State, GameMode, MenuMode, Components, ComponentType};
use ces::system::System;
use DT;

simple_system!
(
	GameLogicSystem[GameMode, State]
	{
		let e = entities.get(entity_idx);
		let mode = &mut e.get_mut(&mut components.game_mode).unwrap();
		mode.time_bonus -= DT;
		if mode.time_bonus < 0.0
		{
			mode.time_bonus = 0.0;
		}
	}
)

simple_system!
(
	GameInputSystem[GameMode, State]
	{
		let mut switch = false;
		let mut reset = false;
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
					key::R => 
					{
						reset = true;
					}
					_ => ()
				}
			});
		}
		
		if switch || reset
		{
			let mode = 
			{
				let e = entities.get(entity_idx);
				e.get_mut(&mut components.game_mode).unwrap()
			};
			entities.sched_remove(mode.player_entity);
			for &star in mode.star_entities.iter()
			{
				entities.sched_remove(star);
			}
		}

		if switch
		{
			let menu_mode = 
			{
				let (state, _mode) = 
				{
					let e = entities.get(entity_idx);
					(e.get_mut(&mut components.state).unwrap(),
				     e.get_mut(&mut components.game_mode).unwrap())
				};
				MenuMode::new(state)
			};
			components.add(entity_idx, menu_mode, entities);
			components.sched_remove::<GameMode>(entity_idx, entities);
		}
		else if reset
		{
			/* Man is this ugly */
			let sys = 
			{
				let e = entities.get(entity_idx);
				e.get(&components.game_mode).unwrap().star_system.clone()
			};
			let mut player_entity = 0;
			let mut star_entities = vec![];
			sys.create_entities(entities, components, 1, 100.0, &mut player_entity, &mut star_entities);
			
			let (state, mode) = 
			{
				let e = entities.get(entity_idx);
				(e.get_mut(&mut components.state).unwrap(),
				 e.get_mut(&mut components.game_mode).unwrap())
			};
			mode.player_entity = player_entity;
			mode.star_entities = star_entities;
			mode.time_bonus = sys.get_time_bonus();
			state.paused = true;
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

simple_system!
(
	GameUIDrawSystem[GameMode, State]
	{
		let e = entities.get(entity_idx);
		let state = &e.get(&components.state).unwrap();
		let core = &state.core;
		let ui_font = &e.get(&components.state).unwrap().ui_font;
		let mode = &e.get(&components.game_mode).unwrap();
		let player_e = entities.get(mode.player_entity);
		let player = &player_e.get(&components.player).unwrap();
		
		let hx = (state.dw as f32) / 2.0;
		let hy = (state.dh as f32) / 2.0;

		let orange = core.map_rgb_f(0.8, 0.7, 0.3);
		let white = core.map_rgb_f(1.0, 1.0, 1.0);
		let gray = core.map_rgb_f(0.7, 0.7, 0.7);
		let blue = core.map_rgb_f(0.2, 0.6, 0.9);

		core.draw_text(ui_font, orange, 20.0, 20.0, AlignLeft, "FUEL:");
		
		let fuel = player.fuel as i32;
		let color = if fuel < 50
		{
			core.map_rgb_f(0.9, 0.2, 0.1)
		}
		else if fuel < 100
		{
			core.map_rgb_f(0.9, 0.9, 0.1)
		}
		else
		{
			core.map_rgb_f(0.3, 0.8, 0.1)
		};
		
		core.draw_text(ui_font, color, 65.0, 20.0, AlignLeft, format!("{}", player.fuel as i32));
		
		core.draw_text(ui_font, orange, state.dw as f32 - 170.0, 20.0, AlignLeft, "HIGH SCORE:");
		core.draw_text(ui_font, gray, state.dw as f32 - 75.0, 20.0, AlignLeft, format!("{}", mode.high_score as i32));
		core.draw_text(ui_font, orange, state.dw as f32 - 130.0, 30.0, AlignLeft, "SCORE:");
		core.draw_text(ui_font, white, state.dw as f32 - 75.0, 30.0, AlignLeft, format!("{}", mode.score as i32));
		
		core.draw_text(ui_font, orange, hx, 20.0, AlignRight, "BONUS:");
		core.draw_text(ui_font, blue, hx, 20.0, AlignLeft, format!(" {}", mode.time_bonus as i32));
		
		if state.paused
		{
			core.draw_text(ui_font, white, hx, hy, AlignCentre, "PAUSED");
		}
	}
)
