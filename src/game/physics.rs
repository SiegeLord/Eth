use ces::Entities;
use ces::components::{Components, Location, Velocity, Acceleration, Size};
use ces::components::ComponentType;
use ces::system::System;
use {FIELD_WIDTH, FIELD_HEIGHT};

simple_system!
(
	PhysicsSystem[Location, Velocity, Acceleration, Size]
	{
		let e = entities.get(entity_idx);
		
		let l = e.get_mut(&mut components.location).unwrap();
		let v = e.get_mut(&mut components.velocity).unwrap();
		let a = e.get_mut(&mut components.acceleration).unwrap();
		let z = e.get(&mut components.size).unwrap();
		let mut o = e.get_mut(&mut components.old_location);
		
		l.x += v.vx + 0.5 * a.ax;
		l.y += v.vy + 0.5 * a.ay;
		
		v.vx += a.ax;
		v.vy += a.ay;
		
		a.ax = 0.0;
		a.ay = 0.0;

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
		if l.x > FIELD_WIDTH as f64 - z.d
		{
			l.x = FIELD_WIDTH as f64 - z.d;
			o.as_mut().map(|o| o.x = l.x);
			v.vx = -v.vx;
		}
		if l.y > FIELD_HEIGHT as f64 - z.d
		{
			l.y = FIELD_HEIGHT as f64 - z.d;
			o.as_mut().map(|o| o.y = l.y);
			v.vy = -v.vy;
		}
	}
)

