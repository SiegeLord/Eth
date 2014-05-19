
#![feature(macro_rules)]

use free_list::FreeList;

mod free_list;

struct Location
{
	x: f32,
	y: f32
}

impl Component for Location
{
	fn add_self(self, components: &mut Components) -> uint
	{
		components.location.add(self)
	}
	fn sched_remove(_: Option<Location>, components: &mut Components, entity_idx: uint, component_idx: uint)
	{
		components.location.sched_remove(entity_idx, component_idx);
	}
	fn get_type(_: Option<Location>) -> ComponentType
	{
		Location
	}
}

struct Velocity
{
	vx: f32,
	vy: f32
}

impl Component for Velocity
{
	fn add_self(self, components: &mut Components) -> uint
	{
		components.velocity.add(self)
	}
	fn sched_remove(_: Option<Velocity>, components: &mut Components, entity_idx: uint, component_idx: uint)
	{
		components.velocity.sched_remove(entity_idx, component_idx);
	}
	fn get_type(_: Option<Velocity>) -> ComponentType
	{
		Velocity
	}
}

trait Component
{
	fn add_self(self, components: &mut Components) -> uint;
	fn sched_remove(dummy: Option<Self>, components: &mut Components, entity_idx: uint, component_idx: uint);
	fn get_type(dummy: Option<Self>) -> ComponentType;
}

#[repr(uint)]
enum ComponentType
{
	Velocity,
	Location
}

impl ComponentType
{
	fn as_uint(&self) -> uint
	{
		*self as uint
	}
}

struct Entity
{
	components: [Option<uint>, ..2]
}

impl Entity
{
	fn new() -> Entity
	{
		Entity
		{
			components: [None, ..2]
		}
	}
	
	fn get(&self, comp_type: ComponentType) -> Option<uint>
	{
		self.components[comp_type.as_uint()]
	}

	fn set(&mut self, comp_type: ComponentType, comp_idx: Option<uint>)
	{
		self.components[comp_type.as_uint()] = comp_idx;
	}
}

trait System
{
	fn remove_entity(&mut self, entity_idx: uint);
	fn component_changed_event(&mut self, entity: &Entity, entity_idx: uint);
	fn update(&self, entities: &mut Entities);
}

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
		
		if entity.get(Velocity).is_some() && entity.get(Location).is_some()
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
	
	fn update(&self, entities: &mut Entities)
	{
		for &entity_idx in self.entities.iter()
		{
			let e = entities.entities.get(entity_idx).unwrap();
			let loc = entities.components.location.get_mut(e.get(Location).unwrap());
			let vel = entities.components.velocity.get(e.get(Velocity).unwrap());
			
			loc.x += vel.vx;
			loc.y += vel.vy;
			
			println!("{} {} {}", entity_idx, loc.x, loc.y);
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

struct Components
{
	velocity: ComponentSet<Velocity>,
	location: ComponentSet<Location>,
}

impl Components
{
	fn new() -> Components
	{
		Components
		{
			velocity: ComponentSet::new(),
			location: ComponentSet::new(),
		}
	}

	fn add<T: Component>(&mut self, comp: T) -> uint
	{
		comp.add_self(self)
	}

	fn sched_remove<T: Component>(&mut self, entity_idx: uint, component_idx: uint)
	{
		Component::sched_remove(None::<T>, self, entity_idx, component_idx);
	}
	
	// entity_idx, component type
	fn handle_removals(&mut self, entity_callback: &mut |uint, ComponentType|)
	{
		self.velocity.process_removals(entity_callback);
		self.location.process_removals(entity_callback);
	}
}

struct Entities
{
	entities: FreeList<Entity>,
	components: Components,
	changes: Vec<uint>,
	removals: Vec<uint>,
}

impl Entities
{
	fn new() -> Entities
	{
		Entities
		{
			components: Components::new(),
			entities: FreeList::new(),
			changes: vec![],
			removals: vec![],
		}
	}

	fn add(&mut self) -> uint
	{
		let idx = self.entities.push(Entity::new());
		self.changes.push(idx);
		idx
	}

	fn sched_remove(&mut self, idx: uint)
	{
		self.removals.push(idx);
	}

	fn add_component<T: Component>(&mut self, entity_idx: uint, comp: T)
	{
		let e = self.entities.get_mut(entity_idx).unwrap();
		if e.get(Component::get_type(None::<T>)).is_none()
		{
			let comp_idx = self.components.add(comp);
			e.set(Component::get_type(None::<T>), Some(comp_idx));
			self.changes.push(entity_idx);
		}
	}
	
	fn sched_remove_component<T: Component>(&mut self, entity_idx: uint)
	{
		let e = self.entities.get_mut(entity_idx).unwrap();
		let changes = &mut self.changes;
		let components = &mut self.components;
		e.get(Component::get_type(None::<T>)).map(|comp_idx|
		{
			components.sched_remove::<T>(entity_idx, comp_idx);
			changes.push(entity_idx);
		});
	}

	fn handle_changes<'l>(&'l mut self, cb: |uint, &'l Entity|)
	{
		let entities = &mut self.entities;
		self.components.handle_removals(&mut |entity_idx, comp_type|
		{
			entities.get_mut(entity_idx).unwrap().set(comp_type, None);
		});
		
		for &entity_idx in self.changes.iter()
		{
			cb(entity_idx, entities.get(entity_idx).unwrap());
		}
		self.changes.clear();
	}
	
	fn handle_removals(&mut self, cb: |uint|)
	{
		let entities = &mut self.entities;	
		for &entity_idx in self.removals.iter()
		{
			entities.free(entity_idx);
			cb(entity_idx);
		}
		self.removals.clear();
	}

	fn get<'l>(&'l self, entity_idx: uint) -> &'l Entity
	{
		self.entities.get(entity_idx).unwrap()
	}
}

struct World
{
	entities: Entities,
	systems: Vec<Box<System>>,	
}

impl World
{
	pub fn new() -> World
	{
		World
		{
			entities: Entities::new(),
			systems: vec![box PhysicsSystem::new() as Box<System>],
		}
	}

	pub fn update(&mut self)
	{
		println!("Update");
		
		let entities = &mut self.entities;
		let systems = &mut self.systems;	
		{
			entities.handle_changes(|entity_idx, entity|
			{
				for sys in systems.mut_iter()
				{
					sys.component_changed_event(entity, entity_idx);
				}
			});
			entities.handle_removals(|entity_idx|
			{
				for sys in systems.mut_iter()
				{
					sys.remove_entity(entity_idx);
				}
			});
		}
		
		for sys in systems.mut_iter()
		{
			sys.update(entities);
		}
	}
}

fn main()
{
	let mut world = World::new();
	let e = world.entities.add();
	world.update();
	world.entities.add_component(e, Location{ x: 10.0, y: 10.0 });
	world.entities.add_component(e, Velocity{ vx: -1.0, vy: -1.0 });
	world.update();
	world.entities.sched_remove_component::<Velocity>(e);
	world.update();
	world.entities.add_component(e, Velocity{ vx: -1.0, vy: -1.0 });
	world.entities.sched_remove(e);
	world.update();
}
