use ces::{Component, ComponentSet};

use allegro5::key::KeyCode;
use allegro5::Core;

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
		dummy: ()
	}
)

component!(
	State, state
	{
		core: Core,
		key_down: Option<KeyCode>
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
