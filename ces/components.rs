use ces::{Component, ComponentSet};

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

components!(
	Location, location;
	Velocity, velocity
)

pub static NUM_COMPONENTS: uint = 2;
