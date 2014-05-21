use ces::{Entity, Entities, Components};

pub trait System
{
	fn remove_entity(&mut self, entity_idx: uint);
	fn component_changed_event(&mut self, entity: &Entity, entity_idx: uint);
	fn update(&self, entities: &mut Entities, components: &mut Components);
}
