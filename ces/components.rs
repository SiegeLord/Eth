use ces::{Component, Components};

pub struct Location
{
	pub x: f32,
	pub y: f32
}

impl Component for Location
{
	fn add_self(self, components: &mut Components) -> uint
	{
		components.location.add(self)
	}
	fn sched_remove(_: Option<Location>, components: &mut Components, entity_idx: uint, component_idx: uint)
	{
		components.location.sched_remove(entity_idx, component_idx);
	}
	fn get_type(_: Option<Location>) -> ComponentType
	{
		Location
	}
}

pub struct Velocity
{
	pub vx: f32,
	pub vy: f32
}

impl Component for Velocity
{
	fn add_self(self, components: &mut Components) -> uint
	{
		components.velocity.add(self)
	}
	fn sched_remove(_: Option<Velocity>, components: &mut Components, entity_idx: uint, component_idx: uint)
	{
		components.velocity.sched_remove(entity_idx, component_idx);
	}
	fn get_type(_: Option<Velocity>) -> ComponentType
	{
		Velocity
	}
}

#[repr(uint)]
pub enum ComponentType
{
	Velocity,
	Location
}

impl ComponentType
{
	pub fn as_uint(&self) -> uint
	{
		*self as uint
	}
}
