use allegro5::*;
use ces::Entities;
use ces::components::{Components, Location, Velocity, Acceleration, Size};
use ces::components::ComponentType;
use ces::system::System;
use {FIELD_WIDTH, FIELD_HEIGHT, MODE_ENTITY};

simple_system!
(
	PhysicsSystem[Location, Velocity, Acceleration, Size]
	{
		let e = entities.get(entity_idx);
		
		let l = e.get_mut(&mut components.location).unwrap();
		let v = e.get_mut(&mut components.velocity).unwrap();
		let a = e.get(&mut components.acceleration).unwrap();
		let z = e.get(&mut components.size).unwrap();
		let mut o = e.get_mut(&mut components.old_location);
		
		v.vx += a.ax;
		v.vy += a.ay;
		
		l.x += v.vx;
		l.y += v.vy;

		if l.x < 0.0
		{
			l.x = 0.0;
			o.as_mut().map(|o| o.x = l.x);
			v.vx = -v.vx;
		}
		if l.y < 0.0
		{
			l.y = 0.0;
			o.as_mut().map(|o| o.y = l.y);
			v.vy = -v.vy;
		}
		if l.x > FIELD_WIDTH as f32 - z.w
		{
			l.x = FIELD_WIDTH as f32 - z.w;
			o.as_mut().map(|o| o.x = l.x);
			v.vx = -v.vx;
		}
		if l.y > FIELD_HEIGHT as f32 - z.h
		{
			l.y = FIELD_HEIGHT as f32 - z.h;
			o.as_mut().map(|o| o.y = l.y);
			v.vy = -v.vy;
		}
	}
)

