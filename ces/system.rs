
#![macro_escape]

use ces::{Entity, Entities};
use ces::components::{Components, ComponentType};

pub trait System
{
	fn get_entities<'l>(&'l mut self) -> &'l mut Vec<uint>;
	fn get_component_types(&self) -> &'static [ComponentType];
	fn update(&self, entities: &mut Entities, components: &mut Components);
	
	fn remove_entity(&mut self, entity_idx: uint)
	{
		let entities = self.get_entities();
		let cur_pos = entities.as_slice().position_elem(&entity_idx);
		for &pos in cur_pos.iter()
		{
			println!("Removed {}", entity_idx);
			entities.swap_remove(pos);
		}
	}

	fn component_changed_event(&mut self, entity: &Entity, entity_idx: uint)
	{
		let types = self.get_component_types();
		let entities = self.get_entities();
		let cur_pos = entities.as_slice().position_elem(&entity_idx);
		
		if entity.have_components(types)
		{
			if cur_pos.is_none()
			{
				println!("Added {}", entity_idx);
				entities.push(entity_idx)
			}
		}
		else
		{			
			for &pos in cur_pos.iter()
			{
				println!("Removed {}", entity_idx);
				entities.swap_remove(pos);
			}
		}
	}
}

macro_rules! simple_system
{
	($name: ident [ $($comp: ident),+ ] $block: expr) =>
	{
		struct $name
		{
			entities: Vec<uint>
		}
		
		impl $name
		{
			pub fn new() -> $name
			{
				$name{ entities: Vec::new() }
			}
		}
		
			
		impl System for $name
		{	
			fn get_entities<'l>(&'l mut self) -> &'l mut Vec<uint>
			{
				&mut self.entities
			}
			
			fn get_component_types(&self) -> &'static [ComponentType]
			{
				static A: &'static [ComponentType] = &'static [$($comp,)*];
				A
			}
			
			fn update(&self, entities: &mut Entities, components: &mut Components)
			{
				for &entity_idx in self.entities.iter()
				{
					$block
				}
			}
		}
	}
}
