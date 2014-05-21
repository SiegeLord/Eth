
#![feature(macro_rules)]
#![feature(globs)]

use ces::{World, Entities};
use ces::components::{Velocity, Location, Components, ComponentType};
use ces::system::System;

mod ces;
mod free_list;

simple_system!
(
	PhysicsSystem[Velocity, Location]
	{
		let e = entities.get(entity_idx);
		let loc = e.get_mut(&mut components.location).unwrap();
		let vel = e.get(&components.velocity).unwrap();
		
		loc.x += vel.vx;
		loc.y += vel.vy;
		
		println!("{} {} {}", entity_idx, loc.x, loc.y);
	}
)


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
