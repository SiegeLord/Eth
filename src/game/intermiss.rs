use allegro5::*;
use allegro_font::*;

use ces::Entities;
use ces::components::{State, GameMode, MenuMode, IntermissMode, Components, ComponentType};
use ces::system::System;

static NUM_ENTRIES: uint = 3;
static FUEL_COST: i32 = 45000;
static CAMERA_COST: i32 = 20000;

simple_system!
(
	IntermissLogicSystem[IntermissMode, State]
	{
		let e = entities.get(entity_idx);
		let mode = e.get_mut(&mut components.intermiss_mode).unwrap();
		let state = e.get_mut(&mut components.state).unwrap();
		
		let rate = 100;
		
		let mut counting = false;
		
		if mode.time_bonus > 0
		{
			mode.time_bonus -= rate;
			mode.disp_score += rate;
			counting = true;
		}
		if mode.fuel_bonus > 0
		{
			mode.fuel_bonus -= rate;
			mode.disp_score += rate;
			counting = true;
		}
		if mode.completion_bonus > 0
		{
			mode.completion_bonus -= rate;
			mode.disp_score += rate;
			counting = true;
		}
		if mode.time_bonus > 0
		{
			mode.time_bonus -= rate;
			mode.disp_score += rate;
			counting = true;
		}
		if mode.cost > 0
		{
			mode.cost -= rate;
			mode.disp_score -= rate;
			counting = true;
		}
		
		if counting
		{
			if mode.score_instance.is_none()
			{
				mode.score_instance = Some(state.sfx.play_persistent(&*mode.score_sound, &state.audio));
			}
			
			mode.score_instance.map(|inst|
			{
				state.sfx.get_instance(inst).set_playing(true);
			});
		}
		else
		{
			mode.score_instance.map(|inst|
			{
				state.sfx.get_instance(inst).set_playing(false);
			});
		}

		if mode.disp_score > mode.disp_high_score
		{
			mode.disp_high_score = mode.disp_score;
		}
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
			let mode = e.get_mut(&mut components.intermiss_mode).unwrap();
			
			state.key_down.map(|k|
			{
				match k
				{
					key::Escape =>
					{
						switch = true;
						state.sfx.play(&*state.ui_sound2, &state.audio);
					}
					key::Space | key::Enter =>
					{
						if mode.next.is_some()
						{
							match mode.cur_sel
							{
								0 =>
								{
									state.sfx.play(&*state.ui_sound2, &state.audio);
									next = true;
								}
								1 =>
								{
									if mode.score > FUEL_COST
									{
										mode.score -= FUEL_COST;
										mode.cost += FUEL_COST;
										mode.fuel += 50.0;
									}
								}
								2 =>
								{
									if mode.score > CAMERA_COST
									{
										mode.score -= CAMERA_COST;
										mode.cost += CAMERA_COST;
										mode.range += 10.0;
									}
								}
								_ => ()
							}
						}
						else
						{
							state.sfx.play(&*state.ui_sound2, &state.audio);
							switch = true;
						}
					}
					key::Up =>
					{
						mode.cur_sel = if mode.cur_sel == 0
						{
							NUM_ENTRIES - 1
						}
						else
						{
							mode.cur_sel - 1
						};
						state.sfx.play(&*state.ui_sound1, &state.audio);
					}
					key::Down =>
					{
						mode.cur_sel = if mode.cur_sel == NUM_ENTRIES - 1
						{
							0
						}
						else
						{
							mode.cur_sel + 1
						};
						state.sfx.play(&*state.ui_sound1, &state.audio);
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
		let state = &mut e.get_mut(&mut components.state).unwrap();
		let core = &state.core;
		let ui_font = &state.ui_font;
		let mode = &e.get(&components.intermiss_mode).unwrap();

		core.draw_bitmap(&state.intermiss_background, 0.0, 0.0, Flag::zero());
		
		let hx = (state.dw as f32) / 2.0;
		let hy = (state.dh as f32) / 2.0;
		
		let orange = core.map_rgb_f(0.8, 0.7, 0.3);
		let white = core.map_rgb_f(1.0, 1.0, 1.0);
		let gray = core.map_rgb_f(0.7, 0.7, 0.7);
		let blue = core.map_rgb_f(0.2, 0.6, 0.9);
		let green = core.map_rgb_f(0.3, 0.8, 0.1);
		let red = core.map_rgb_f(0.9, 0.2, 0.1);
		
		core.draw_text(ui_font, orange, hx, hy - 100.0, AlignRight, "TIME BONUS:");
		core.draw_text(ui_font, orange, hx, hy - 90.0, AlignRight, "FUEL BONUS:");
		core.draw_text(ui_font, orange, hx, hy - 80.0, AlignRight, "FINISH BONUS:");
		core.draw_text(ui_font, orange, hx, hy - 70.0, AlignRight, "COST:");
		core.draw_text(ui_font, orange, hx, hy - 50.0, AlignRight, "SCORE:");
		core.draw_text(ui_font, orange, hx, hy - 40.0, AlignRight, "HIGH SCORE:");
		
		if mode.disp_high_score == mode.disp_score
		{
			core.draw_text(ui_font, white, hx, hy, AlignCentre, "NEW HIGH SCORE!");
		}
		
		core.draw_text(ui_font, green, hx, hy - 100.0, AlignLeft, format!(" {}", mode.time_bonus).as_slice());
		core.draw_text(ui_font, green, hx, hy - 90.0, AlignLeft, format!(" {}", mode.fuel_bonus).as_slice());
		core.draw_text(ui_font, green, hx, hy - 80.0, AlignLeft, format!(" {}", mode.completion_bonus).as_slice());
		core.draw_text(ui_font, red, hx, hy - 70.0, AlignLeft, format!(" {}", mode.cost).as_slice());
		core.draw_text(ui_font, white, hx, hy - 50.0, AlignLeft, format!(" {}", mode.disp_score).as_slice());
		core.draw_text(ui_font, gray, hx, hy - 40.0, AlignLeft, format!(" {}", mode.disp_high_score).as_slice());
		
		if mode.next.is_some()
		{
			let spacing = 16.0;
			let mut y = hy + 32.0;		
			
			let fs = format!("BUY FUEL TANK ({})", FUEL_COST);
			let cs = format!("BUY BETTER CAMERA ({})", CAMERA_COST);
			let text = ["CONTINUE", fs.as_slice(), cs.as_slice()];
			
			for entry in range(0u, NUM_ENTRIES)
			{
				let color = if entry == mode.cur_sel
				{
					white
				}
				else
				{
					blue
				};
				
				core.draw_text(ui_font, color, hx, y, AlignCentre, text[entry]);
				
				y += spacing;
			}
		}
		else
		{
			core.draw_text(ui_font, white, hx, hy + 20.0, AlignCentre, format!("{} COMPLETE!", state.set_name).as_slice());
		}
	}
)

