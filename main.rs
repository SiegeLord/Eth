
#![feature(macro_rules)]
#![feature(globs)]

use ces::{World, Entity, Entities, Components};
use ces::components::{Velocity, Location};
use ces::system::System;

mod ces;
mod free_list;

struct PhysicsSystem
{
	entities: Vec<uint>
}

impl PhysicsSystem
{
	pub fn new() -> PhysicsSystem
	{
		PhysicsSystem{ entities: Vec::new() }
	}
}

impl System for PhysicsSystem
{	
	fn remove_entity(&mut self, entity_idx: uint)
	{
		let cur_pos = self.entities.as_slice().position_elem(&entity_idx);
		for &pos in cur_pos.iter()
		{
			println!("Removed {}", entity_idx);
			self.entities.swap_remove(pos);
		}
	}

	fn component_changed_event(&mut self, entity: &Entity, entity_idx: uint)
	{
		let cur_pos = self.entities.as_slice().position_elem(&entity_idx);
		
		if entity.have_components([Velocity, Location])
		{
			if cur_pos.is_none()
			{
				println!("Added {}", entity_idx);
				self.entities.push(entity_idx)
			}
		}
		else
		{			
			for &pos in cur_pos.iter()
			{
				println!("Removed {}", entity_idx);
				self.entities.swap_remove(pos);
			}
		}
	}
	
	fn update(&self, entities: &mut Entities, components: &mut Components)
	{
		for &entity_idx in self.entities.iter()
		{
			let e = entities.get(entity_idx);
			let loc = e.get_mut(&mut components.location).unwrap();
			let vel = e.get(&components.velocity).unwrap();
			
			loc.x += vel.vx;
			loc.y += vel.vy;
			
			println!("{} {} {}", entity_idx, loc.x, loc.y);
		}
	}
}


fn main()
{
	let mut world = World::new();
	world.add_system(box PhysicsSystem::new());
	let e = world.add_entity();
	world.update();
	world.add_component(e, Location{ x: 10.0, y: 10.0 });
	world.add_component(e, Velocity{ vx: -1.0, vy: -1.0 });
	world.update();
	world.sched_remove_component::<Velocity>(e);
	world.update();
	world.add_component(e, Velocity{ vx: -1.0, vy: -1.0 });
	world.sched_remove_entity(e);
	world.update();
}
