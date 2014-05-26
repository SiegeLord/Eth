use ces::{Component, ComponentSet, Entities};

use allegro5::key::KeyCode;
use allegro5::{Bitmap, Core};
use allegro_font::{FontAddon, Font};
use allegro_audio::{AudioAddon, Sample};
use bitmap_loader::BitmapLoader;
use sample_loader::SampleLoader;
use resource_manager::ResourceManager;
use std::rc::Rc;
use star_system::{save_high_score, StarSystem, load_sets};
use menu::NUM_APPEARANCES;
use std::cmp::max;
use animation::Animation;
use sfx::Sfx;

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
	IntermissMode, intermiss_mode
	{
		set: StrBuf,
		next: Option<StrBuf>,
		score: i32,
		high_score: i32,
		fuel: f64,
		max_fuel: f64,
		range: f64,
		disp_score: i32,
		time_bonus: i32,
		fuel_bonus: i32,
		completion_bonus: i32,
		cur_sel: uint,
		cost: i32,
		disp_high_score: i32,
		score_sound: Rc<Sample>,
		purchase_sound: Rc<Sample>,
		score_instance: Option<uint>
	}
)

impl IntermissMode
{
	pub fn new(set: &str, next: Option<StrBuf>, time_bonus: f64, score: i32, high_score: i32, max_fuel: f64, range: f64, fuel: f64, state: &mut State) -> IntermissMode
	{
		let old_high_score = high_score;
		let old_score = score;
		let fuel_bonus = (fuel as i32) * 100;
		let time_bonus = time_bonus as i32 * 100;
		let completion_bonus = 50 * 100;
		let score = score + time_bonus + fuel_bonus + completion_bonus;
		let score_sound = state.sample_manager.load("data/score.ogg", &state.audio).unwrap();
		let purchase_sound = state.sample_manager.load("data/purchase.ogg", &state.audio).unwrap();
		
		let high_score = max(score, high_score);
		if high_score != old_high_score
		{
			save_high_score(set, high_score);
		}

		IntermissMode
		{
			set: set.to_strbuf(),
			next: next,
			score: score,
			high_score: high_score,
			max_fuel: max_fuel,
			range: range,
			fuel: fuel,
			disp_score: old_score,
			fuel_bonus: fuel_bonus,
			time_bonus: time_bonus,
			completion_bonus: completion_bonus,
			cur_sel: 0,
			cost: 0,
			disp_high_score: old_high_score,
			score_sound: score_sound,
			purchase_sound: purchase_sound,
			score_instance: None,
		}
	}
}

component!(
	GameMode, game_mode
	{
		set: StrBuf,
		star_system: StarSystem,
		player_entity: uint,
		other_entities: Vec<uint>,
		time_bonus: f64,
		score: i32,
		high_score: i32,
		max_fuel: f64,
		range: f64,
		targets: i32,
		intro_text_pos: f32//,
		//~ camera_sound: Rc<Sample>,
		//~ explosion: Rc<Sample>,
	}
)

impl GameMode
{
	pub fn new(set: &str, sys: &str, score: i32, high_score: i32, max_fuel: f64, range: f64, appearance: i32, entities: &mut Entities, components: &mut Components) -> GameMode
	{
		let sys = StarSystem::new(set, sys);
		let mut player_entity = 0;
		let mut other_entities = vec![];
		sys.create_entities(entities, components, appearance, max_fuel, &mut player_entity, &mut other_entities);
		let time_bonus = sys.get_time_bonus();
		let targets = sys.get_num_targets();
		GameMode
		{
			set: set.to_strbuf(),
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
		set_sel: uint,
		sets: Vec<(StrBuf, StrBuf)>,
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
			set_sel: 0,
			sets: load_sets("levels"),
			title: state.bmp_manager.load("data/title.png", &state.core).unwrap(),
			planets: planets,
		}
	}
}

component!(
	State, state
	{
		core: Core,
		audio: AudioAddon,
		font: FontAddon,
		bmp_manager: ResourceManager<StrBuf, Bitmap, BitmapLoader>,
		sample_manager: ResourceManager<StrBuf, Sample, SampleLoader>,
		key_down: Option<KeyCode>,
		key_up: Option<KeyCode>,
		ui_font: Font,
		dw: i32,
		dh: i32,
		quit: bool,
		draw_interp: f64,
		paused: bool,
		stopped: bool,
		appearance: i32,
		set_name: StrBuf,
		game_background: Bitmap,
		intermiss_background: Bitmap,
		ui_sound1: Rc<Sample>,
		ui_sound2: Rc<Sample>,
		explosion_sound: Rc<Sample>,
		easter_sound: Rc<Sample>,
		sfx: Sfx
	}
)

component!(
	Player, player
	{
		fuel: f64,
		up: f64,
		down: f64,
		left: f64,
		right: f64,
		up_spr: Animation,
		down_spr: Animation,
		left_spr: Animation,
		right_spr: Animation,
		engine_sound: Rc<Sample>,
		engine_instance: Option<uint>,
		camera_sound: Rc<Sample>
	}
)

impl Player
{
	pub fn new(fuel: f64, state: &mut State) -> Player
	{
		Player
		{
			fuel: fuel,
			up: 0.0,
			down: 0.0,
			left: 0.0,
			right: 0.0,
			up_spr: Animation::new("data/thruster_up.cfg", false, state),
			down_spr: Animation::new("data/thruster_down.cfg", false, state),
			left_spr: Animation::new("data/thruster_left.cfg", false, state),
			right_spr: Animation::new("data/thruster_right.cfg", false, state),
			engine_sound: state.sample_manager.load("data/engine.ogg", &state.audio).unwrap(),
			engine_instance: None,
			camera_sound: state.sample_manager.load("data/camera.ogg", &state.audio).unwrap(),
		}
	}
}

component!(
	Sprite, sprite
	{
		bmp: Animation
	}
)

impl Sprite
{
	pub fn new(name: &str, play_once: bool, state: &mut State) -> Sprite
	{
		Sprite
		{
			bmp: Animation::new(name, play_once, state)
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
		reticle_near: Animation,
		reticle_far: Animation
	}
)

impl Target
{
	pub fn new(state: &mut State) -> Target
	{
		Target
		{
			reticle_near: Animation::new("data/reticle.cfg", false, state),
			reticle_far: Animation::new("data/reticle2.cfg", false, state)
		}
	}
}

components!(
	Location, location;           // 1
	Velocity, velocity;           // 2
	Acceleration, acceleration;   // 3
	GameMode, game_mode;          // 4
	MenuMode, menu_mode;          // 5
	State, state;                 // 6
	Player, player;               // 7
	Size, size;                   // 8
	OldLocation, old_location;    // 9
	Sprite, sprite;               // 10
	Mass, mass;                   // 11
	Target, target;               // 12
	Solid, solid;                 // 13
	Hole, hole;                   // 14
	Switchable, switchable;       // 15
	IntermissMode, intermiss_mode // 16
)
//                                   ^
pub static NUM_COMPONENTS: uint =    16;
