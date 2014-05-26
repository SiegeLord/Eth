// Copyright 2014 SiegeLord
// Licensed under GPL, see LICENSE for full terms

#![macro_escape]

use free_list::FreeList;
use ces::components::{ComponentType, Components, NUM_COMPONENTS};
use ces::system::{System};
use collections::HashMap;

macro_rules! components
{
	( $($comp_name: ident, $field_name: ident);* ) =>
	{
		pub struct Components
		{
			$(
				pub $field_name: ComponentSet<$comp_name>,
			)*
		}
		
		impl Components
		{
			pub fn new() -> Components
			{
				Components
				{
					$(
						$field_name: ComponentSet::new(),
					)*
				}
			}
			
			// entity_idx, component type
			pub fn process_removals(&mut self, entity_callback: &mut |uint, ComponentType|)
			{
				$(
					self.$field_name.process_removals(entity_callback);
				)*
			}
		}
		
		#[repr(uint)]
		pub enum ComponentType
		{
			$(
				$comp_name,
			)*
		}
		
		impl ComponentType
		{
			pub fn as_uint(&self) -> uint
			{
				*self as uint
			}
		}
	}
}

macro_rules! component_impl
{
	($comp_name: ident , $field_name: ident) =>
	{
		impl Component for $comp_name
		{
			fn add_self(self, components: &mut Components) -> uint
			{
				components.$field_name.add(self)
			}

			fn get<'l>(_: Option<$comp_name>, comp_idx: uint, components: &'l Components) -> &'l $comp_name
			{
				components.$field_name.get(comp_idx)
			}

			fn get_mut<'l>(_: Option<$comp_name>, comp_idx: uint, components: &'l mut Components) -> &'l mut $comp_name
			{
				components.$field_name.get_mut(comp_idx)
			}

			fn sched_remove(_: Option<$comp_name>, components: &mut Components, entity_idx: uint, comp_idx: uint)
			{
				components.$field_name.sched_remove(entity_idx, comp_idx);
			}

			fn get_type(_: Option<$comp_name>) -> ComponentType
			{
				$comp_name
			}
		}
	}
}

macro_rules! component
{
	($comp_name: ident , $field_name: ident { $($name: ident : $typ: ty),+ } ) =>
	{
		pub struct $comp_name
		{
			$(
				pub $name : $typ,
			)*
		}
		
		component_impl!($comp_name, $field_name)
	}
}

pub mod system;
pub mod components;

trait Component
{
	fn add_self(self, components: &mut Components) -> uint;
	fn get<'l>(dummy: Option<Self>, comp_idx: uint, components: &'l Components) -> &'l Self;
	fn get_mut<'l>(dummy: Option<Self>, comp_idx: uint, components: &'l mut Components) -> &'l mut Self;
	fn sched_remove(dummy: Option<Self>, components: &mut Components, entity_idx: uint, comp_idx: uint);
	fn get_type(dummy: Option<Self>) -> ComponentType;
}

pub struct Entity
{
	components: [Option<uint>, ..NUM_COMPONENTS]
}

impl Entity
{
	fn new() -> Entity
	{
		Entity
		{
			components: [None, ..NUM_COMPONENTS]
		}
	}
	
	fn get_comp_idx(&self, comp_type: ComponentType) -> Option<uint>
	{
		self.components[comp_type.as_uint()]
	}

	fn set_comp_idx(&mut self, comp_type: ComponentType, comp_idx: Option<uint>)
	{
		self.components[comp_type.as_uint()] = comp_idx;
	}

	pub fn have_components(&self, comps: &[ComponentType]) -> bool
	{
		let mut have_all = true;
		for &comp in comps.iter()
		{
			have_all &= self.get_comp_idx(comp).is_some();
			if !have_all
			{
				break;
			}
		}
		have_all
	}
	
	#[allow(dead_code)]
	pub fn get<'l, T: Component>(&self, comp_set: &'l ComponentSet<T>) -> Option<&'l T>
	{
		self.get_comp_idx(Component::get_type(None::<T>)).map(|idx| comp_set.get(idx))
	}

	#[allow(dead_code)]
	pub fn get_mut<'l, T: Component>(&self, comp_set: &'l mut ComponentSet<T>) -> Option<&'l mut T>
	{
		match self.get_comp_idx(Component::get_type(None::<T>))
		{
			Some(idx) => Some(comp_set.get_mut(idx)),
			None => None
		}
	}
}

struct ComponentSet<T>
{
	components: FreeList<T>,
	removals: Vec<(uint, uint)>,
}

impl<T: Component> ComponentSet<T>
{
	fn new() -> ComponentSet<T>
	{
		ComponentSet
		{
			components: FreeList::new(),
			removals: vec![],
		}
	}

	fn process_removals(&mut self, entity_handler: &mut |uint, ComponentType|)
	{
		for &(entity_idx, comp_idx) in self.removals.iter()
		{
			(*entity_handler)(entity_idx, Component::get_type(None::<T>));
			self.components.free(comp_idx);
		}
		self.removals.clear();
	}
	
	fn add(&mut self, comp: T) -> uint
	{
		let idx = self.components.push(comp);
		idx
	}

	fn sched_remove(&mut self, entity_idx: uint, comp_idx: uint)
	{
		self.removals.push((entity_idx, comp_idx));
	}

	fn get_mut<'l>(&'l mut self, comp_idx: uint) -> &'l mut T
    {
        self.components.get_mut(comp_idx).unwrap()
    }
    
    fn get<'l>(&'l self, comp_idx: uint) -> &'l T
    {
        self.components.get(comp_idx).unwrap()
    }
}

impl self::components::Components
{
	pub fn add<T: Component>(&mut self, entity_idx: uint, comp: T, entities: &mut Entities)
	{
		let e = entities.entities.get_mut(entity_idx).unwrap();
		if e.get_comp_idx(Component::get_type(None::<T>)).is_none()
		{
			let comp_idx = comp.add_self(self);
			e.set_comp_idx(Component::get_type(None::<T>), Some(comp_idx));
			entities.changes.push(entity_idx);
		}
	}

	pub fn get<'l, T: Component>(&'l self, entity_idx: uint, entities: &Entities) -> Option<&'l T>
	{
		let e = entities.entities.get(entity_idx).unwrap();
		e.get_comp_idx(Component::get_type(None::<T>)).map(|comp_idx|
		{
			Component::get(None::<T>, comp_idx, self)
		})
	}

	pub fn get_mut<'l, T: Component>(&'l mut self, entity_idx: uint, entities: &Entities) -> Option<&'l mut T>
	{
		let e = entities.entities.get(entity_idx).unwrap();
		match e.get_comp_idx(Component::get_type(None::<T>))
		{
			Some(comp_idx) => Some(Component::get_mut(None::<T>, comp_idx, self)),
			None => None
		}
	}

	pub fn sched_remove<T: Component>(&mut self, entity_idx: uint, entities: &mut Entities)
	{
		let e = entities.entities.get_mut(entity_idx).unwrap();
		let changes = &mut entities.changes;
		e.get_comp_idx(Component::get_type(None::<T>)).map(|comp_idx|
		{
			Component::sched_remove(None::<T>, self, entity_idx, comp_idx);
			changes.push(entity_idx);
		});
	}
}

pub struct Entities
{
	entities: FreeList<Entity>,
	changes: Vec<uint>,
	removals: Vec<uint>,
}

impl Entities
{
	fn new() -> Entities
	{
		Entities
		{
			entities: FreeList::new(),
			changes: vec![],
			removals: vec![],
		}
	}

	pub fn add(&mut self) -> uint
	{
		let idx = self.entities.push(Entity::new());
		self.changes.push(idx);
		idx
	}

	pub fn sched_remove(&mut self, idx: uint)
	{
		self.removals.push(idx);
	}

	fn process_changes<'l>(&'l mut self, components: &mut Components, cb: |uint, &'l Entity|)
	{
		let entities = &mut self.entities;
		components.process_removals(&mut |entity_idx, comp_type|
		{
			entities.get_mut(entity_idx).unwrap().set_comp_idx(comp_type, None);
		});
		
		for &entity_idx in self.changes.iter()
		{
			cb(entity_idx, entities.get(entity_idx).unwrap());
		}
		self.changes.clear();
	}
	
	fn process_removals(&mut self, cb: |uint|)
	{
		let entities = &mut self.entities;	
		for &entity_idx in self.removals.iter()
		{
			entities.free(entity_idx);
			cb(entity_idx);
		}
		self.removals.clear();
	}

	pub fn get<'l>(&'l self, entity_idx: uint) -> &'l Entity
	{
		self.entities.get(entity_idx).unwrap()
	}
}

pub struct World
{
	entities: Entities,
	components: Components,
	systems: HashMap<i32, Vec<Box<System>>>,
}

impl World
{
	pub fn new() -> World
	{
		World
		{
			entities: Entities::new(),
			components: Components::new(),
			systems: HashMap::new(),
		}
	}

	#[allow(dead_code)]
	pub fn add_system<T: ToPrimitive>(&mut self, event_id: T, sys: Box<System>)
	{
		let vec = self.systems.find_or_insert(event_id.to_i32().unwrap(), vec![]);
		vec.push(sys);
	}

	#[allow(dead_code)]
	pub fn update(&mut self)
	{
		//~ println!("Update");
		
		let entities = &mut self.entities;
		let systems = &mut self.systems;	
		{
			entities.process_changes(&mut self.components, |entity_idx, entity|
			{
				for sys in systems.mut_iter().flat_map(|(_, vec)| vec.mut_iter())
				{
					sys.component_changed_event(entity, entity_idx);
				}
			});
			entities.process_removals(|entity_idx|
			{
				for sys in systems.mut_iter().flat_map(|(_, vec)| vec.mut_iter())
				{
					sys.remove_entity(entity_idx);
				}
			});
		}
	}

	#[allow(dead_code)]
	pub fn update_systems<T: ToPrimitive>(&mut self, event_id: T)
	{
		self.update();
		let entities = &mut self.entities;
		let components = &mut self.components;
		self.systems.find_mut(&event_id.to_i32().unwrap()).map(|systems|
		{
			for sys in systems.mut_iter()
			{
				sys.update(entities, components);
			}
		});
	}

	#[allow(dead_code)]
	pub fn add_entity(&mut self) -> uint
	{
		self.entities.add()
	}

	#[allow(dead_code)]
	pub fn sched_remove_entity(&mut self, entity_idx: uint)
	{
		self.entities.sched_remove(entity_idx);
	}

	#[allow(dead_code)]
	pub fn add_component<T: Component>(&mut self, entity_idx: uint, comp: T)
	{
		self.components.add(entity_idx, comp, &mut self.entities);
	}

	#[allow(dead_code)]
	pub fn get_component_mut<'l, T: Component>(&'l mut self, entity_idx: uint) -> Option<&'l mut T>
	{
		self.components.get_mut(entity_idx, &self.entities)
	}
	
	#[allow(dead_code)]
	pub fn get_component<'l, T: Component>(&'l self, entity_idx: uint) -> Option<&'l T>
	{
		self.components.get(entity_idx, &self.entities)
	}
	
	#[allow(dead_code)]
	pub fn sched_remove_component<T: Component>(&mut self, entity_idx: uint)
	{
		self.components.sched_remove::<T>(entity_idx, &mut self.entities);
	}
}
