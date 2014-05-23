
#![feature(macro_rules, globs, phase)]

extern crate collections;
#[phase(syntax, link)]
extern crate allegro5;
extern crate allegro_dialog;

use allegro5::*;
use allegro_dialog::*;
use ces::{World, Entities};
use ces::components::{State, GameMode, MenuMode, Components, ComponentType};
use ces::system::System;

mod ces;
mod free_list;
mod resource_manager;

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

//~ simple_system!
//~ (
	//~ PhysicsSystem[Velocity, Location]
	//~ {
		//~ let e = entities.get(entity_idx);
		//~ let loc = e.get_mut(&mut components.location).unwrap();
		//~ let vel = e.get(&components.velocity).unwrap();
		//~ 
		//~ loc.x += vel.vx;
		//~ loc.y += vel.vy;
		//~ 
		//~ println!("{} {} {}", entity_idx, loc.x, loc.y);
	//~ }
//~ )

simple_system!
(
	MenuInputSystem[MenuMode, State]
	{
		let switch = 
		{
			let e = entities.get(entity_idx);
			e.get_mut(&mut components.state).unwrap().key_down.map_or(false, |k| k == key::Space)
		};
		
		if switch
		{
			components.add(entity_idx, GameMode{ dummy: () }, entities);
			components.sched_remove::<MenuMode>(entity_idx, entities);
			println!("Switch!");
		}
	}
)

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
	MenuDrawSystem[MenuMode, State]
	{
		let e = entities.get(entity_idx);
		let core = &e.get(&mut components.state).unwrap().core;
		println!("Draw menu");
		
		core.clear_to_color(core.map_rgb_f(0.0, 0.0, 0.0));
	}
)

simple_system!
(
	GameDrawSystem[GameMode, State]
	{
		let e = entities.get(entity_idx);
		let core = &e.get(&mut components.state).unwrap().core;
		println!("Draw game");

		core.clear_to_color(core.map_rgb_f(0.5, 0.2, 0.4));
	}
)

static MODE_ENTITY: uint = 0;

fn game()
{
	let mut core = Core::init().unwrap();
	core.install_keyboard();
	
	let disp = core.create_display(800, 600).unwrap();
	disp.set_window_title(&"Main Window".to_c_str());

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
	
	world.add_entity();
	world.add_component(MODE_ENTITY, MenuMode{ dummy: () });
	world.add_component(MODE_ENTITY, State{ key_down: None, core: core });
	
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
			world.update_systems(Draw);
			
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
				if k == key::Escape
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
