use ces::Entities;
use ces::components::{Components, Location, OldLocation};
use ces::components::ComponentType;
use ces::system::System;

simple_system!
(
	OldLocationSystem[Location, OldLocation]
	{
		let e = entities.get(entity_idx);
		
		let l = e.get(&mut components.location).unwrap();
		let o = e.get_mut(&mut components.old_location).unwrap();
		
		o.x = l.x;
		o.y = l.y;
	}
)


