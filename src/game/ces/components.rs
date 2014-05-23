use ces::{Component, ComponentSet};

use allegro5::key::KeyCode;
use allegro5::{Bitmap, Core};
use allegro_font::{FontAddon, Font};
use bitmap_loader::BitmapLoader;
use resource_manager::ResourceManager;
use std::rc::Rc;

component!(
	Location, location
	{
		x: f32,
		y: f32
	}
)

component!(
	Velocity, velocity
	{
		vx: f32,
		vy: f32
	}
)

component!(
	GameMode, game_mode
	{
		dummy: ()
	}
)

component!(
	MenuMode, menu_mode
	{
		cur_sel: uint,
		title: Rc<Bitmap>
	}
)

impl MenuMode
{
	pub fn new(state: &mut State) -> MenuMode
	{
		MenuMode
		{
			cur_sel: 0,
			title: state.bmp_manager.load("data/title.png", &state.core).unwrap()
		}
	}
}

component!(
	State, state
	{
		core: Core,
		font: FontAddon,
		bmp_manager: ResourceManager<StrBuf, Bitmap, BitmapLoader>,
		key_down: Option<KeyCode>,
		ui_font: Font,
		dw: i32,
		dh: i32,
		quit: bool
	}
)

components!(
	Location, location;
	Velocity, velocity;
	GameMode, game_mode;
	MenuMode, menu_mode;
	State, state
)

pub static NUM_COMPONENTS: uint = 5;
