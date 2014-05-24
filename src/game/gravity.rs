use ces::{Entities, Entity};
use ces::components::{Components, Location, Size, Mass, Acceleration};
use ces::components::ComponentType;
use ces::system::System;

pub struct GravitySystem
{
	all_masses: Vec<uint>,
	movable_masses: Vec<uint>
}

impl GravitySystem
{
	pub fn new() -> GravitySystem
	{
		GravitySystem
		{
			all_masses: vec![],
			movable_masses: vec![]
		}
	}
}

impl System for GravitySystem
{	
	fn get_entities<'l>(&'l mut self) -> &'l mut Vec<uint>
	{
		unreachable!()
	}
	
	fn get_component_types(&self) -> &'static [ComponentType]
	{
		unreachable!()
	}
	
	fn update(&self, entities: &mut Entities, components: &mut Components)
	{
		for &movable_idx in self.movable_masses.iter()
		{
			let movable_e = entities.get(movable_idx);
			let mov_l = movable_e.get(&components.location).unwrap();
			let mov_z = movable_e.get(&components.size).unwrap();
			let mov_a = movable_e.get_mut(&mut components.acceleration).unwrap();
			
			let mut ax = 0.0;
			let mut ay = 0.0;
			
			for &all_idx in self.all_masses.iter()
			{
				if movable_idx == all_idx
				{
					continue;
				}
				
				let all_e = entities.get(all_idx);
				let all_l = all_e.get(&components.location).unwrap();
				let all_m = all_e.get(&components.mass).unwrap();
				let all_z = all_e.get(&components.size).unwrap();
				
				let dx = (all_l.x + all_z.d / 2.0) - (mov_l.x + mov_z.d / 2.0);
				let dy = (all_l.y + all_z.d / 2.0) - (mov_l.y + mov_z.d / 2.0);
				let rsq = dx * dx + dy * dy;
				if rsq > 100.0
				{
					let a = 5e1 * all_m.mass / rsq;
					
					let r = rsq.sqrt();
					ax += a * dx / r;
					ay += a * dy / r;
				}
			}
			
			//~ println!("{} {:e} {:e}", movable_idx, ax, ay);
			mov_a.ax += ax;
			mov_a.ay += ay;
		}
	}
	
	fn remove_entity(&mut self, entity_idx: uint)
	{
		let cur_pos = self.movable_masses.as_slice().position_elem(&entity_idx);
		for &pos in cur_pos.iter()
		{
			self.movable_masses.swap_remove(pos);
		}

		let cur_pos = self.all_masses.as_slice().position_elem(&entity_idx);
		for &pos in cur_pos.iter()
		{
			self.all_masses.swap_remove(pos);
		}
	}
	
	fn component_changed_event(&mut self, entity: &Entity, entity_idx: uint)
	{
		static ALL: &'static [ComponentType] = &'static [Mass, Location, Size];
		static MOVABLE: &'static [ComponentType] = &'static [Mass, Location, Acceleration, Size];
	
		let cur_pos = self.all_masses.as_slice().position_elem(&entity_idx);
		if entity.have_components(ALL)
		{
			if cur_pos.is_none()
			{
				self.all_masses.push(entity_idx)
			}
		}
		else
		{			
			for &pos in cur_pos.iter()
			{
				self.all_masses.swap_remove(pos);
			}
		}
	
		let cur_pos = self.movable_masses.as_slice().position_elem(&entity_idx);
		if entity.have_components(MOVABLE)
		{
			if cur_pos.is_none()
			{
				self.movable_masses.push(entity_idx)
			}
		}
		else
		{			
			for &pos in cur_pos.iter()
			{
				self.movable_masses.swap_remove(pos);
			}
		}
	}
}
