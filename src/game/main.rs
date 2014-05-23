
#![feature(macro_rules, globs, phase)]

extern crate collections;
#[phase(syntax, link)]
extern crate allegro5;
extern crate allegro_dialog;
extern crate allegro_font;
extern crate allegro_image;
extern crate libc;

use allegro5::*;
use allegro_dialog::*;
use allegro_font::*;
use allegro_image::*;
use ces::{World, Entities};
use ces::components::{State, GameMode, MenuMode, Components, ComponentType};
use ces::system::System;
use menu::{MenuInputSystem, MenuDrawSystem};
use resource_manager::ResourceManager;

mod ces;
mod menu;
mod free_list;
mod resource_manager;
mod bitmap_loader;

#[repr(i32)]
enum WorldEvent
{
	Draw,
	Input,
	Logic
}

impl ToPrimitive for WorldEvent
{
	fn to_u64(&self) -> Option<u64>
	{
		Some(*self as u64)
	}

	fn to_i64(&self) -> Option<i64>
	{
		Some(*self as i64)
	}
}

simple_system!
(
	GameLogicSystem[GameMode, State]
	{
		let _ = entity_idx;
		let _ = entities;
		let _ = components;
		println!("Logic!");
	}
)

simple_system!
(
	GameDrawSystem[GameMode, State]
	{
		let e = entities.get(entity_idx);
		let core = &e.get(&mut components.state).unwrap().core;
		
		
		let t = unsafe
		{
			allegro5::ffi::al_get_time()
		};
		
		//~ println!("Draw game {}", t.fract());

		core.clear_to_color(core.map_rgb_f(t.fract() as f32, 0.0, 0.0));
	}
)

static MODE_ENTITY: uint = 0;

fn game()
{
	let mut core = Core::init().unwrap();
	let font = FontAddon::init(&core).expect("Could not init font addon");
	let _image = ImageAddon::init(&core).expect("Could not init image addon");
	
	core.install_keyboard();
	
	let disp = core.create_display(800, 600).unwrap();
	disp.set_window_title(&"E'th".to_c_str());
	let bw = disp.get_width() / 2;
	let bh = disp.get_height() / 2;
	let buffer = core.create_bitmap(bw, bh).unwrap();

	let timer = core.create_timer(1.0 / 60.0).unwrap();

	let q = core.create_event_queue().unwrap();
	q.register_event_source(disp.get_event_source());
	q.register_event_source(core.get_keyboard_event_source().unwrap());
	q.register_event_source(timer.get_event_source());

	let mut world = World::new();
	
	world.add_system(Input, box MenuInputSystem::new());
	
	world.add_system(Logic, box GameLogicSystem::new());
	
	world.add_system(Draw, box GameDrawSystem::new());
	world.add_system(Draw, box MenuDrawSystem::new());
	
	let bmp_manager = ResourceManager::new();
	let ui_font = font.load_bitmap_font("data/font.png").expect("Couldn't create built-in font from 'data/font.png'");
	
	let mut state = State
	{
		key_down: None,
		core: core,
		font: font,
		bmp_manager: bmp_manager,
		ui_font: ui_font,
		dh: bh,
		dw: bw,
		quit: false,
	};
	
	world.add_entity();
	world.add_component(MODE_ENTITY, MenuMode::new(&mut state));
	world.add_component(MODE_ENTITY, state);
	
	fn get_state<'l>(world: &'l mut World) -> &'l mut State
	{
		world.get_component_mut::<State>(MODE_ENTITY).unwrap()
	}
	
	let mut redraw = true;
	timer.start();
	'exit: loop
	{
		if redraw && q.is_empty()
		{
			get_state(&mut world).core.set_target_bitmap(&buffer);
			
			world.update_systems(Draw);
			
			let c = get_state(&mut world).core.map_rgb_f(1.0, 0.0, 0.0);
			get_state(&mut world).core.draw_pixel(-1.0, -1.0, c);
			
			get_state(&mut world).core.set_target_bitmap(disp.get_backbuffer());
			get_state(&mut world).core.draw_scaled_bitmap(&buffer, 0.0, 0.0, bw as f32, bh as f32, 0.0, 0.0, bw as f32 * 2.0, bh as f32 * 2.0, Flag::zero());
			
			disp.flip();
			redraw = false;
		}

		get_state(&mut world).key_down = None;
		match q.wait_for_event()
		{
			DisplayClose{..} =>
			{
				break 'exit;
			},
			KeyDown{keycode: k, ..} =>
			{
				get_state(&mut world).key_down = Some(k);
				world.update_systems(Input);
				if k == key::Escape || get_state(&mut world).quit
				{
					break 'exit;
				}
			},
			TimerTick{..} =>
			{
				world.update_systems(Logic);
				redraw = true;
			},
			_ => ()
		}
	}
}

allegro_main!
{
	use std::task::try;
    use std::any::AnyRefExt;

	//~ game();
	match try(game)
	{
		Err(e) =>
		{
			let err = e.as_ref::<&'static str>().map(|e| e.to_strbuf()).or_else(||
			{
				e.as_ref::<~str>().map(|e| e.to_strbuf())
			}).or_else(||
			{
				e.as_ref::<StrBuf>().map(|e| e.clone())
			}).unwrap_or("Unknown error!".to_strbuf());
			
			show_native_message_box(None, "Error!", "An error has occurred! Redirect stderr from the command line for more info.", err.as_slice(), Some("You make me sad."), MESSAGEBOX_ERROR);
		}
		Ok(_) => ()
	}
}
