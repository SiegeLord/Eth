
#![feature(macro_rules)]

struct Location
{
	x: f32,
	y: f32
}

struct Velocity
{
	vx: f32,
	vy: f32
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
}

trait System
{
	fn remove_entity(&mut self, entity_idx: uint);
	fn component_changed_event(&mut self, entity: &Entity, entity_idx: uint);
	fn update(&self, entities: &[Entity], components: &mut Components);
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
	
	fn update(&self, entities: &[Entity], components: &mut Components)
	{
		for &entity_idx in self.entities.iter()
		{
			let e = &entities[entity_idx];
			let loc = components.location.get_mut(e.get(Location).unwrap());
			let vel = components.velocity.get(e.get(Velocity).unwrap());
			
			loc.x += vel.vx;
			loc.y += vel.vy;
			
			println!("{} {} {}", entity_idx, loc.x, loc.y);
		}
	}
}

struct ComponentSet<T>
{
	components: Vec<T>,
	changes: Vec<(uint, Option<T>)>,
}

impl<T> ComponentSet<T>
{
	fn new() -> ComponentSet<T>
	{
		ComponentSet
		{
			components: vec![],
			changes: vec![],
		}
	}

	fn handle_changes(&mut self, entity_handler: |uint, Option<uint>|)
	{
		for &(idx, ref mut comp) in self.changes.mut_iter()
		{
			let components = &mut self.components;
			entity_handler(idx, comp.take().map(|c|
			{
				components.push(c);
				components.len() - 1
			}));
		}
		self.changes.clear()
	}
	
	fn set(&mut self, entity_idx: uint, comp: Option<T>)
	{
		self.changes.push((entity_idx, comp));
	}

	fn get_mut<'l>(&'l mut self, comp_idx: uint) -> &'l mut T
    {
        self.components.get_mut(comp_idx)
    }
    
    fn get<'l>(&'l self, comp_idx: uint) -> &'l T
    {
        self.components.get(comp_idx)
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
	
	// entity, component type, component
	fn handle_changes(&mut self, entity_handler: |uint, ComponentType, Option<uint>|)
	{
		self.velocity.handle_changes(|entity, comp| entity_handler(entity, Velocity, comp));
		self.location.handle_changes(|entity, comp| entity_handler(entity, Location, comp));
	}
}

struct World
{
	entities: Vec<Entity>,
	systems: Vec<Box<System>>,	
	components: Components,
	entity_additions: Vec<uint>,
	entity_removals: Vec<uint>,
}

impl World
{
	pub fn new() -> World
	{
		World
		{
			entities: vec![],
			systems: vec![box PhysicsSystem::new() as Box<System>],
			components: Components::new(),
			entity_additions: vec![],
			entity_removals: vec![],
		}
	}
	
	pub fn add_entity(&mut self, e: Entity) -> uint
	{
		// TODO: freelist
		self.entities.push(e);
		let idx = self.entities.len() - 1;
		self.entity_additions.push(idx);
		idx
	}

	pub fn remove_entity(&mut self, idx: uint)
	{
		// TODO: freelist
		self.entity_removals.push(idx);
	}
	
	pub fn set_velocity(&mut self, idx: uint, comp: Option<Velocity>)
	{
		self.components.velocity.set(idx, comp);
	}

	pub fn set_location(&mut self, idx: uint, comp: Option<Location>)
	{
		self.components.location.set(idx, comp);
	}
	
	pub fn update(&mut self)
	{
		println!("Update");
		
		let entities = &mut self.entities;
		let systems = &mut self.systems;	
		{
			self.components.handle_changes(|entity, comp_type, comp|
			{
				entities.get_mut(entity).components[comp_type.as_uint()] = comp;
				for sys in systems.mut_iter()
				{
					sys.component_changed_event(entities.get(entity), entity);
				}
			});
		}
		
		for &idx in self.entity_additions.mut_iter()
		{
			for sys in systems.mut_iter()
			{
				sys.component_changed_event(entities.get(idx), idx);
			}
		}
		self.entity_additions.clear();
		
		for &idx in self.entity_removals.mut_iter()
		{
			for sys in systems.mut_iter()
			{
				sys.component_changed_event(entities.get(idx), idx);
			}
		}
		self.entity_removals.clear();
		
		for sys in systems.mut_iter()
		{
			sys.update(entities.as_slice(), &mut self.components);
		}
	}
}

fn main()
{
	let mut world = World::new();
	let e = world.add_entity(Entity::new());
	world.update();
	world.set_location(e, Some(Location{ x: 10.0, y: 10.0 }));
	world.set_velocity(e, Some(Velocity{ vx: -1.0, vy: -1.0 }));
	world.update();
	world.set_velocity(e, None);
	world.update();
	world.set_velocity(e, Some(Velocity{ vx: -1.0, vy: -1.0 }));
	world.remove_entity(e);
	world.update();
}
