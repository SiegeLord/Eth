use ces::{Component, ComponentSet, Entities};

use allegro5::key::KeyCode;
use allegro5::{Bitmap, Core};
use allegro_font::{FontAddon, Font};
use bitmap_loader::BitmapLoader;
use resource_manager::ResourceManager;
use std::rc::Rc;
use star_system::StarSystem;
use menu::NUM_APPEARANCES;

component!(
	Location, location
	{
		x: f64,
		y: f64
	}
)

component!(
	OldLocation, old_location
	{
		x: f64,
		y: f64
	}
)

component!(
	Velocity, velocity
	{
		vx: f64,
		vy: f64
	}
)

component!(
	Acceleration, acceleration
	{
		ax: f64,
		ay: f64
	}
)

component!(
	Size, size
	{
		d: f64
	}
)


component!(
	Solid, solid
	{
		dummy: ()
	}
)

component!(
	Hole, hole
	{
		dummy: ()
	}
)

component!(
	Switchable, switchable
	{
		dummy: ()
	}
)

component!(
	GameMode, game_mode
	{
		star_system: StarSystem,
		player_entity: uint,
		other_entities: Vec<uint>,
		time_bonus: f64,
		score: i32,
		high_score: i32,
		max_fuel: f64,
		range: f64,
		targets: i32,
		intro_text_pos: f32
	}
)

impl GameMode
{
	pub fn new(star_system: &str, score: i32, high_score: i32, max_fuel: f64, range: f64, appearance: i32, entities: &mut Entities, components: &mut Components) -> GameMode
	{
		let sys = StarSystem::new(star_system);
		let mut player_entity = 0;
		let mut other_entities = vec![];
		sys.create_entities(entities, components, appearance, max_fuel, &mut player_entity, &mut other_entities);
		let time_bonus = sys.get_time_bonus();
		let targets = sys.get_num_targets();
		GameMode
		{
			star_system: sys,
			player_entity: player_entity,
			other_entities: other_entities,
			score: score,
			high_score: high_score,
			time_bonus: time_bonus,
			max_fuel: max_fuel,
			range: range,
			targets: targets,
			intro_text_pos: 0.0
		}
	}
}

component!(
	MenuMode, menu_mode
	{
		cur_sel: uint,
		title: Rc<Bitmap>,
		planets: Vec<Rc<Bitmap>>
	}
)

impl MenuMode
{
	pub fn new(state: &mut State) -> MenuMode
	{
		let planets: Vec<_> = range(0, NUM_APPEARANCES).map(|n|
		{
			state.bmp_manager.load(format!("data/planet{}.png", n).as_slice(), &state.core).unwrap()
		}).collect();
		MenuMode
		{
			cur_sel: 0,
			title: state.bmp_manager.load("data/title.png", &state.core).unwrap(),
			planets: planets,
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
		draw_interp: f64,
		paused: bool,
		stopped: bool,
		appearance: i32
	}
)

component!(
	Player, player
	{
		fuel: f64,
		up: f64,
		down: f64,
		left: f64,
		right: f64
	}
)

impl Player
{
	pub fn new(fuel: f64) -> Player
	{
		Player
		{
			fuel: fuel,
			up: 0.0,
			down: 0.0,
			left: 0.0,
			right: 0.0
		}
	}
}

component!(
	Sprite, sprite
	{
		bmp: Rc<Bitmap>
	}
)

impl Sprite
{
	pub fn new(name: &str, state: &mut State) -> Sprite
	{
		Sprite
		{
			bmp: state.bmp_manager.load(name, &state.core).unwrap()
		}
	}
}

component!(
	Mass, mass
	{
		mass: f64
	}
)

component!(
	Target, target
	{
		reticle_near: Rc<Bitmap>,
		reticle_far: Rc<Bitmap>
	}
)

impl Target
{
	pub fn new(state: &mut State) -> Target
	{
		Target
		{
			reticle_near: state.bmp_manager.load("data/reticle.png", &state.core).unwrap(),
			reticle_far: state.bmp_manager.load("data/reticle2.png", &state.core).unwrap()
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
	OldLocation, old_location;  // 9
	Sprite, sprite;             // 10
	Mass, mass;                 // 11
	Target, target;             // 12
	Solid, solid;               // 13
	Hole, hole;                 // 14
	Switchable, switchable      // 15
)
//                                 ^
pub static NUM_COMPONENTS: uint =  15;
