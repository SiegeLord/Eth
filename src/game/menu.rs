//~ use allegro5;
use allegro5::*;
use allegro_font::*;

use ces::Entities;
use ces::components::{State, GameMode, MenuMode, Components, ComponentType};
use ces::system::System;
use star_system::{get_set_name, load_high_score};

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
						if mode.cur_sel == 1
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
						else if mode.cur_sel == 2
						{
							mode.set_sel = if mode.set_sel == 0
							{
								mode.sets.len() - 1
							}
							else
							{
								mode.set_sel - 1
							};
						}
					}
					key::Right =>
					{
						if mode.cur_sel == 1
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
						else if mode.cur_sel == 2
						{
							mode.set_sel = if mode.set_sel == mode.sets.len() - 1
							{
								0
							}
							else
							{
								mode.set_sel + 1
							};
						}
					}
					key::Space | key::Enter =>
					{
						match mode.cur_sel
						{
							0 => switch = true,
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
			let (appearance, set_file) =
			{
				let e = entities.get(entity_idx);
				let mode = e.get(&components.menu_mode).unwrap();
				let set_file = mode.sets.get(mode.set_sel).ref0().clone();
				(e.get(&components.state).unwrap().appearance, set_file)
			};
			{
				let game_mode = GameMode::new(set_file.as_slice(), "start", 0, load_high_score(set_file.as_slice()), 100.0, 50.0, appearance, entities, components);
				components.add(entity_idx, game_mode, entities);
				components.sched_remove::<MenuMode>(entity_idx, entities);
			}
			let e = entities.get(entity_idx);
			let state = e.get_mut(&mut components.state).unwrap();
			state.paused = true;
			state.set_name = get_set_name(set_file.as_slice());
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
		core.draw_bitmap(bmp, ((state.dw - bmp.get_width()) / 2) as f32, hy - 125.0, Flag::zero());
		
		let spacing = 16.0;
		let mut y = hy + 32.0;
		
		let get_sel_color = |cur_sel, entry: uint|
		{
			if entry == cur_sel
			{
				white
			}
			else
			{
				blue
			}
		};
		
		core.draw_text(ui_font, get_sel_color(mode.cur_sel, 0), hx, y, AlignCentre, "START");
		y += spacing;
		
		core.draw_text(ui_font, get_sel_color(mode.cur_sel, 1), hx, y, AlignCentre, "APPEARANCE");
		y += spacing;
		
		core.draw_text(ui_font, get_sel_color(mode.cur_sel, 1), hx, y, AlignCentre, "<      >");
		
		let bmp = &**mode.planets.get(state.appearance as uint);
		core.draw_bitmap(bmp, hx - bmp.get_width() as f32 / 2.0, y - bmp.get_height() as f32 / 2.0 + 2.0, Flag::zero());
		
		y += spacing;
		
		core.draw_text(ui_font, get_sel_color(mode.cur_sel, 2), hx, y, AlignCentre, "MISSION SET");
		y += spacing;
		
		core.draw_text(ui_font, get_sel_color(mode.cur_sel, 2), hx, y, AlignCentre, "<          >");
		core.draw_text(ui_font, get_sel_color(mode.cur_sel, 2), hx, y, AlignCentre, format!("{}", mode.sets.get(mode.set_sel).ref1()).as_slice());
		y += spacing;
		
		core.draw_text(ui_font, get_sel_color(mode.cur_sel, 3), hx, y, AlignCentre, "QUIT");
	}
)
