//~ use allegro5;
use allegro5::*;
use allegro_font::*;

use ces::Entities;
use ces::components::{State, GameMode, MenuMode, Components, ComponentType};
use ces::system::System;

static NUM_ENTRIES: uint = 4;
pub static NUM_APPEARANCES: i32 = 3;

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
					key::Left =>
					{
						if mode.cur_sel == 0
						{
							state.appearance = if state.appearance == 0
							{
								NUM_APPEARANCES - 1
							}
							else
							{
								state.appearance - 1
							};
						}
					}
					key::Right =>
					{
						if mode.cur_sel == 0
						{
							state.appearance = if state.appearance == NUM_APPEARANCES - 1
							{
								0
							}
							else
							{
								state.appearance + 1
							};
						}
					}
					key::Space | key::Enter =>
					{
						match mode.cur_sel
						{
							1 => switch = true,
							3 => state.quit = true,
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
			let appearance =
			{
				let e = entities.get(entity_idx);
				e.get(&components.state).unwrap().appearance
			};
			{
				let game_mode = GameMode::new("levels/beth.cfg", "start", 0, 1000, 300.0, 50.0, appearance, entities, components);
				components.add(entity_idx, game_mode, entities);
				components.sched_remove::<MenuMode>(entity_idx, entities);
			}
			let e = entities.get(entity_idx);
			let state = e.get_mut(&mut components.state).unwrap();
			state.paused = true;
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
		
		let white = core.map_rgb_f(1.0, 1.0, 1.0);
		let blue = core.map_rgb_f(0.2, 0.6, 0.9);
		
		let hx = (state.dw as f32) / 2.0;
		let hy = (state.dh as f32) / 2.0;
		
		let bmp = &*mode.title;
		core.draw_bitmap(bmp, ((state.dw - bmp.get_width()) / 2) as f32, hy - 135.0, Flag::zero());
		
		let spacing = 16.0;
		let mut y = hy + 32.0;		
		
		let text = ["START", "OPTIONS", "QUIT"];
		
		let color = if mode.cur_sel == 0
		{
			white
		}
		else
		{
			blue
		};
		
		core.draw_text(ui_font, color, hx, hy - 4.0, AlignCentre, "APPEARANCE");
		
		core.draw_text(ui_font, color, hx, hy + 12.0, AlignCentre, "<      >");
		
		let bmp = &**mode.planets.get(state.appearance as uint);
		core.draw_bitmap(bmp, hx - bmp.get_width() as f32 / 2.0, hy - bmp.get_height() as f32 / 2.0 + 16.0, Flag::zero());
		
		for entry in range(1u, NUM_ENTRIES)
		{
			let color = if entry == mode.cur_sel
			{
				white
			}
			else
			{
				blue
			};
			
			core.draw_text(ui_font, color, hx, y, AlignCentre, text[entry - 1]);
			
			y += spacing;
		}
	}
)
