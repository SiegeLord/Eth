// Copyright 2014 SiegeLord
// Licensed under GPL, see LICENSE for full terms

use allegro5::*;
use allegro_audio::*;
use allegro_font::*;

use ces::Entities;
use ces::components::{State, GameMode, MenuMode, IntermissMode, Components, ComponentType};
use ces::system::System;
use DT;

simple_system!
(
	GameLogicSystem[GameMode, State]
	{
		let e = entities.get(entity_idx);
		let mode = e.get_mut(&mut components.game_mode).unwrap();
		let state = e.get_mut(&mut components.state).unwrap();
		if mode.targets > 0
		{
			mode.time_bonus -= DT;
			if mode.time_bonus < 0.0
			{
				mode.time_bonus = 0.0;
			}
		}
		else
		{
			state.paused = true;
			state.stopped = true;
		}
		mode.intro_text_pos -= 1.0;
	}
)

simple_system!
(
	GameInputSystem[GameMode, State]
	{
		let mut switch = false;
		let mut reset = false;
		let mut next = false;
		{
			let e = entities.get(entity_idx);
			let state = e.get_mut(&mut components.state).unwrap();
			let game_mode = e.get_mut(&mut components.game_mode).unwrap();
			
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
					key::Space =>
					{
						if game_mode.targets == 0
						{
							next = true;
						}
					}
					_ => ()
				}
			});
		}
		
		if switch || reset || next
		{
			let mode = 
			{
				let e = entities.get(entity_idx);
				e.get_mut(&mut components.game_mode).unwrap()
			};
			entities.sched_remove(mode.player_entity);
			for &star in mode.other_entities.iter()
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
				state.stopped = false;
				state.sfx.play(&*state.ui_sound2, &state.audio);
				state.sfx.play_music("data/clone_-_spacerace.mod", &state.audio);
				MenuMode::new(state)
			};
			components.add(entity_idx, menu_mode, entities);
			components.sched_remove::<GameMode>(entity_idx, entities);
		}
		else if next
		{
			let intermiss_mode = 
			{
				let (state, mode, fuel) = 
				{
					let e = entities.get(entity_idx);
					let game_mode = e.get_mut(&mut components.game_mode).unwrap();
					let player_e = entities.get(game_mode.player_entity);
					let fuel = player_e.get(&components.player).unwrap().fuel;
					(e.get_mut(&mut components.state).unwrap(), game_mode, fuel)
				};
				state.stopped = false;
				IntermissMode::new(mode.set.as_slice(), mode.star_system.get_next().map(|s| s.clone()),
				                   mode.time_bonus, mode.score, mode.high_score, mode.max_fuel, mode.range, fuel, state)
			};
			components.add(entity_idx, intermiss_mode, entities);
			components.sched_remove::<GameMode>(entity_idx, entities);
		}
		else if reset
		{
			/* Man is this ugly */
			let (appearance, max_fuel, sys) = 
			{
				let e = entities.get(entity_idx);
				let game_mode = e.get(&components.game_mode).unwrap();
				let state = e.get_mut(&mut components.state).unwrap();
				let player_e = entities.get(game_mode.player_entity);
				let player = player_e.get_mut(&mut components.player);
				player.map(|player|
				{
					player.engine_instance.map(|inst|
					{
						state.sfx.get_instance(inst).set_playmode(PlaymodeOnce);
					});
				});
				(state.appearance,
				 game_mode.max_fuel,
				 game_mode.star_system.clone())
			};
			let mut player_entity = 0;
			let mut other_entities = vec![];
			sys.create_entities(entities, components, appearance, max_fuel, &mut player_entity, &mut other_entities);
			
			let (state, mode) = 
			{
				let e = entities.get(entity_idx);
				(e.get_mut(&mut components.state).unwrap(),
				 e.get_mut(&mut components.game_mode).unwrap())
			};
			
			state.sfx.play(&*state.ui_sound2, &state.audio);
			
			mode.player_entity = player_entity;
			mode.other_entities = other_entities;
			mode.time_bonus = sys.get_time_bonus();
			mode.targets = sys.get_num_targets();
			mode.intro_text_pos = 0.0;
			state.paused = true;
			state.stopped = false;
		}
	}
)

simple_system!
(
	GameDrawSystem[GameMode, State]
	{
		let e = entities.get(entity_idx);
		let state = &e.get(&components.state).unwrap();
		let core = &state.core;

		core.draw_bitmap(&state.game_background, 0.0, 0.0, Flag::zero());
	}
)

simple_system!
(
	GameUIDrawSystem[GameMode, State]
	{
		let e = entities.get(entity_idx);
		let state = &mut e.get_mut(&mut components.state).unwrap();
		let core = &state.core;
		let ui_font = &state.ui_font;
		let mode = &e.get(&components.game_mode).unwrap();
		let player_e = entities.get(mode.player_entity);
		
		let hx = (state.dw as f32) / 2.0;
		let hy = (state.dh as f32) / 2.0;
		
		let red = core.map_rgb_f(0.9, 0.2, 0.1);
		
		match player_e.get(&components.player)
		{
			Some(player) =>
			{
				let orange = core.map_rgb_f(0.8, 0.7, 0.3);
				let white = core.map_rgb_f(1.0, 1.0, 1.0);
				let blue = core.map_rgb_f(0.2, 0.6, 0.9);
				let green = core.map_rgb_f(0.3, 0.8, 0.1);
			
				core.draw_text(ui_font, orange, 20.0, 20.0, AlignLeft, "FUEL:");
				
				let fuel = player.fuel as i32;
				let color = if fuel < 50
				{
					red
				}
				else if fuel < 100
				{
					core.map_rgb_f(0.9, 0.9, 0.1)
				}
				else
				{
					green
				};
				
				core.draw_text(ui_font, color, 65.0, 20.0, AlignLeft, format!("{}", fuel).as_slice());
			
			
				core.draw_text(ui_font, orange, state.dw as f32 - 130.0, 20.0, AlignLeft, "SCORE:");
				core.draw_text(ui_font, white, state.dw as f32 - 75.0, 20.0, AlignLeft, format!("{}", mode.score as i32).as_slice());
				
				core.draw_text(ui_font, orange, hx, 20.0, AlignRight, "BONUS:");
				core.draw_text(ui_font, blue, hx, 20.0, AlignLeft, format!(" {}", mode.time_bonus as i32).as_slice());
				
				if mode.targets <= 0
				{
					core.draw_text(ui_font, green, hx, hy - 10.0, AlignCentre, "MISSION ACCOMPLISHED");
					core.draw_text(ui_font, orange, hx, hy + 10.0, AlignCentre, "PRESS 'SPACE' TO CONTINUE");
				}
				else
				{			
					core.draw_text(ui_font, orange, hx, (state.dh as f32) - 30.0, AlignRight, "TARGETS:");
					core.draw_text(ui_font, white, hx, (state.dh as f32) - 30.0, AlignLeft, format!(" {}", mode.targets).as_slice());
					
					if state.paused
					{
						core.draw_text(ui_font, white, hx, hy, AlignCentre, "PAUSED");
					}
					
					mode.star_system.get_intro_text().map(|text|
					{
						core.draw_text(ui_font, white, hx + mode.intro_text_pos, (state.dh as f32) - 60.0, AlignCentre, text.as_slice());
					});
				}
			}
			None =>
			{
				core.draw_text(ui_font, red, hx, hy + 10.0, AlignCentre, "DESTROYED! PRESS 'R' TO TRY AGAIN");
			}
		}
	}
)
