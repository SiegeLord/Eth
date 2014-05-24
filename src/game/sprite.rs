use allegro5::*;
use ces::Entities;
use ces::components::{Components, Location, Size, Sprite, OldLocation};
use ces::components::ComponentType;
use ces::system::System;
use MODE_ENTITY;

simple_system!
(
	SpriteDrawSystem[Sprite, Location, OldLocation, Size]
	{
		let mode_e = entities.get(MODE_ENTITY);
		let e = entities.get(entity_idx);
		
		let sprite = e.get(&mut components.sprite).unwrap();
		let l = e.get(&mut components.location).unwrap();
		let o = e.get(&mut components.old_location).unwrap();
		let z = e.get(&mut components.size).unwrap();
		
		let state = mode_e.get(&mut components.state).unwrap();
		let core = &state.core;
		
		let bmp = &*sprite.bmp;

		let x = l.x + (l.x - o.x) * state.draw_interp + (z.d - bmp.get_width() as f64) / 2.0;
		let y = l.y + (l.y - o.y) * state.draw_interp + (z.d - bmp.get_height() as f64) / 2.0;
		
		core.draw_bitmap(bmp, x as f32, y as f32, Flag::zero());
	}
)
