//~ use allegro5;
use allegro5::*;
use allegro_font::*;

use ces::Entities;
use ces::components::{State, GameMode, MenuMode, Components, ComponentType};
use ces::system::System;

static NUM_ENTRIES: uint = 3;

simple_system!
(
	MenuInputSystem[MenuMode, State]
	{
		let mut switch = false;
		{
			let e = entities.get(entity_idx);
			let mode = e.get_mut(&mut components.menu_mode).unwrap();
			let state = e.get_mut(&mut components.state).unwrap();
			
			state.key_down.map(|k|
			{
				match k
				{
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
					}
					key::Space | key::Enter =>
					{
						match mode.cur_sel
						{
							0 => switch = true,
							2 => state.quit = true,
							_ => ()
						}
					}
					key::Escape =>
					{
						state.quit = true;
					}
					_ => ()
				}
			});
		}
		
		if switch
		{
			let game_mode = GameMode::new("levels/beth.cfg", 0, 1000, entities, components);
			components.add(entity_idx, game_mode, entities);
			components.sched_remove::<MenuMode>(entity_idx, entities);
		}
	}
)

simple_system!
(
	MenuDrawSystem[MenuMode, State]
	{
		let e = entities.get(entity_idx);
		let mode = e.get(&mut components.menu_mode).unwrap();
		let state = e.get(&mut components.state).unwrap();
		let core = &state.core;
		let ui_font = &state.ui_font;
		
		core.clear_to_color(core.map_rgb_f(0.0, 0.0, 0.0));
		
		let hx = (state.dw as f32) / 2.0;
		let hy = (state.dh as f32) / 2.0;
		
		let spacing = 16.0;
		let mut y = hy + spacing * 1.0;
		
		let text = ["START", "OPTIONS", "QUIT"];
		
		{
			let bmp = &*mode.title;
			core.draw_bitmap(bmp, ((state.dw - bmp.get_width()) / 2) as f32, hy - 125.0, Flag::zero());
		}
		
		for entry in range(0u, NUM_ENTRIES)
		{
			let color = if entry == mode.cur_sel
			{
				core.map_rgb_f(1.0, 1.0, 1.0)
			}
			else
			{
				core.map_rgb_f(0.2, 0.6, 0.9)
			};
			
			core.draw_text(ui_font, color, hx, y, AlignCentre, text[entry]);
			
			y += spacing;
		}
	}
)
