use ces::{Component, ComponentSet, Entities};

use allegro5::key::KeyCode;
use allegro5::{Bitmap, Core};
use allegro_font::{FontAddon, Font};
use bitmap_loader::BitmapLoader;
use resource_manager::ResourceManager;
use std::rc::Rc;
use star_system::StarSystem;
use player::create_player;

component!(
	Location, location
	{
		x: f32,
		y: f32
	}
)

component!(
	OldLocation, old_location
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
	Acceleration, acceleration
	{
		ax: f32,
		ay: f32
	}
)

component!(
	Size, size
	{
		w: f32,
		h: f32
	}
)

component!(
	GameMode, game_mode
	{
		star_system: StarSystem,
		player_entity: uint
	}
)

impl GameMode
{
	pub fn new(star_system: &str, entities: &mut Entities, components: &mut Components) -> GameMode
	{
		let sys = StarSystem::new(star_system);
		let player_entity = create_player(0, sys.start_x, sys.start_y, entities, components);
		GameMode
		{
			star_system: sys,
			player_entity: player_entity,
		}
	}
}

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
		key_up: Option<KeyCode>,
		ui_font: Font,
		dw: i32,
		dh: i32,
		quit: bool,
		draw_interp: f32
	}
)

component!(
	Player, player
	{
		bmp: Rc<Bitmap>,
		up: f32,
		down: f32,
		left: f32,
		right: f32
	}
)

impl Player
{
	pub fn new(appearance: uint, state: &mut State) -> Player
	{
		let bmp = state.bmp_manager.load(format!("data/planet{}.png", appearance), &state.core).unwrap();
		Player
		{
			bmp: bmp,
			up: 0.0,
			down: 0.0,
			left: 0.0,
			right: 0.0
		}
	}
}

components!(
	Location, location;         // 1
	Velocity, velocity;         // 2
	Acceleration, acceleration; // 3
	GameMode, game_mode;        // 4
	MenuMode, menu_mode;        // 5
	State, state;               // 6
	Player, player;             // 7
	Size, size;                 // 8
	OldLocation, old_location   // 9
)
//                                 ^
pub static NUM_COMPONENTS: uint =  9;
