
#![feature(macro_rules, globs, phase)]

extern crate collections;
#[phase(syntax, link)]
extern crate allegro5;
extern crate allegro_dialog;
extern crate allegro_font;
extern crate allegro_image;
extern crate libc;
extern crate toml;

use allegro5::*;
use allegro_dialog::*;
use allegro_font::*;
use allegro_image::*;
use ces::World;
use ces::components::{State, MenuMode};
use menu::{MenuInputSystem, MenuDrawSystem};
use game::{GameInputSystem, GameLogicSystem, GameDrawSystem, GameUIDrawSystem};
use player::{PlayerLogicSystem, PlayerInputSystem};
use sprite::SpriteDrawSystem;
use physics::PhysicsSystem;
use gravity::GravitySystem;
use old_location::OldLocationSystem;
use resource_manager::ResourceManager;

mod ces;
mod menu;
mod game;
mod free_list;
mod resource_manager;
mod bitmap_loader;
mod star_system;
mod player;
mod physics;
mod old_location;
mod sprite;
mod gravity;

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

static MODE_ENTITY: uint = 0;
static FIELD_WIDTH: i32 = 600;
static FIELD_HEIGHT: i32 = 350;
static DT: f64 = 1.0 / 60.0;

fn game()
{
	let root = toml::parse_from_file("options.cfg").ok().expect("Could not load/parse 'options.cfg'");
	
	let manual_vsync = root.lookup("game.manual_vsync").map(|v| v.get_bool().unwrap_or(false)).unwrap_or(false);
	let fullscreen = root.lookup("game.fullscreen").map(|v| v.get_bool().unwrap_or(false)).unwrap_or(false);
	
	let mut core = Core::init().unwrap();
	let font = FontAddon::init(&core).expect("Could not init font addon");
	let _image = ImageAddon::init(&core).expect("Could not init image addon");
	
	core.install_keyboard();
	
	if !manual_vsync
	{
		core.set_new_display_option(Vsync, 1, Suggest);
	}
	if fullscreen
	{
		core.set_new_display_flags(FULLSCREEN_WINDOW);
	}
	let disp = core.create_display(1200, 700).unwrap();
	disp.set_window_title(&"E'th".to_c_str());
	let bw = FIELD_WIDTH;
	let bh = FIELD_HEIGHT;
	let buffer = core.create_bitmap(bw, bh).unwrap();

	let timer = core.create_timer(DT).unwrap();

	let mut q = core.create_event_queue().unwrap();
	q.register_event_source(disp.get_event_source());
	q.register_event_source(core.get_keyboard_event_source().unwrap());
	q.register_event_source(timer.get_event_source());

	let mut world = World::new();
	
	world.add_system(Input, box GameInputSystem::new());
	world.add_system(Input, box MenuInputSystem::new());
	world.add_system(Input, box PlayerInputSystem::new());
	
	world.add_system(Logic, box OldLocationSystem::new());
	world.add_system(Logic, box GameLogicSystem::new());
	world.add_system(Logic, box GravitySystem::new());
	world.add_system(Logic, box PlayerLogicSystem::new());
	world.add_system(Logic, box PhysicsSystem::new()); // Must be last
	
	world.add_system(Draw, box GameDrawSystem::new());
	world.add_system(Draw, box MenuDrawSystem::new());
	world.add_system(Draw, box SpriteDrawSystem::new());
	world.add_system(Draw, box GameUIDrawSystem::new());
	//~ world.add_system(Draw, box PlayerDrawSystem::new());
	
	let bmp_manager = ResourceManager::new();
	let ui_font = font.load_bitmap_font("data/font.png").expect("Couldn't create built-in font from 'data/font.png'");
	
	let mut state = State
	{
		key_down: None,
		key_up: None,
		core: core,
		font: font,
		bmp_manager: bmp_manager,
		ui_font: ui_font,
		dh: bh,
		dw: bw,
		quit: false,
		draw_interp: 0.0,
		paused: false,
	};
	
	world.add_entity();
	world.add_component(MODE_ENTITY, MenuMode::new(&mut state));
	world.add_component(MODE_ENTITY, state);
	
	fn get_state<'l>(world: &'l mut World) -> &'l mut State
	{
		world.get_component_mut::<State>(MODE_ENTITY).unwrap()
	}
	
	timer.start();
	let mut game_time = 0.0;
	let offset = get_state(&mut world).core.get_time();
	'exit: loop
	{
		for event in q
		{
			get_state(&mut world).key_down = None;
			get_state(&mut world).key_up = None;
			
			match event
			{
				DisplayClose{..} =>
				{
					break 'exit;
				},
				KeyDown{keycode: k, ..} =>
				{
					get_state(&mut world).paused = false;
					get_state(&mut world).key_down = Some(k);
					world.update_systems(Input);
				},
				KeyUp{keycode: k, ..} =>
				{
					get_state(&mut world).key_up = Some(k);
					world.update_systems(Input);
				},
				TimerTick{count, ..} =>
				{
					game_time = count as f64 * DT;
					if !get_state(&mut world).paused
					{
						world.update_systems(Logic);
					}
				},
				_ => ()
			}
			
			if get_state(&mut world).quit
			{
				break 'exit;
			}
		}
		
		get_state(&mut world).core.set_target_bitmap(&buffer);

		let cur_time = get_state(&mut world).core.get_time();
		get_state(&mut world).draw_interp = ((cur_time - offset - game_time) / DT) as f64;
		world.update_systems(Draw);
		
		let c = get_state(&mut world).core.map_rgb_f(1.0, 0.0, 0.0);
		get_state(&mut world).core.draw_pixel(-1.0, -1.0, c);
		
		let dx = ((disp.get_width() - 2 * bw) / 2) as f32;
		let dy = ((disp.get_height() - 2 * bh) / 2) as f32;
		
		get_state(&mut world).core.set_target_bitmap(disp.get_backbuffer());
		get_state(&mut world).core.draw_scaled_bitmap(&buffer, 0.0, 0.0, bw as f32, bh as f32, dx, dy, bw as f32 * 2.0, bh as f32 * 2.0, Flag::zero());
		
		if manual_vsync
		{
			disp.wait_for_vsync();
		}
		disp.flip();
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
