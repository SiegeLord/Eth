use toml;
use player::create_player;
use target::create_target;
use ces::Entities;
use ces::components::{Components, Sprite, Location, Size, OldLocation, Mass};
use MODE_ENTITY;

pub fn create_star(x: f64, y: f64, appearance: i32, entities: &mut Entities, components: &mut Components) -> uint
{
	let sprite = 
	{
		let state = entities.get(MODE_ENTITY).get_mut(&mut components.state).unwrap();
		Sprite::new(format!("data/star{}.png", appearance), state)
	};
	
	let e = entities.add();
	components.add(e, Location{ x: x, y: y }, entities);
	components.add(e, OldLocation{ x: x, y: y }, entities);
	components.add(e, Size{ w: 16.0, h: 16.0 }, entities);
	components.add(e, Mass{ mass: 1.0 }, entities);
	components.add(e, sprite, entities);
	e
}

#[deriving(Clone)]
pub struct StarSystem
{
	start_x: f64,
	start_y: f64,
	start_vx: f64,
	start_vy: f64,
	stars: Vec<(f64, f64, i32)>,
	targets: Vec<(f64, f64, i32)>,
	intro_text: Option<StrBuf>
}

impl StarSystem
{
	pub fn new(name: &str) -> StarSystem
	{
		let root = toml::parse_from_file(name).ok().expect(format!("Could not load/parse '{}'", name));
		
		let start_pos = root.lookup("start_pos").unwrap();
		
		let start_x = start_pos.lookup_vec(0).unwrap().get_int().unwrap() as f64;
		let start_y = start_pos.lookup_vec(1).unwrap().get_int().unwrap() as f64;
		
		let start_vel = root.lookup("start_vel").unwrap();
		
		let start_vx = start_vel.lookup_vec(0).unwrap().get_float().unwrap() as f64;
		let start_vy = start_vel.lookup_vec(1).unwrap().get_float().unwrap() as f64;
		
		let intro_text = root.lookup("intro_text").map(|v| v.get_str().unwrap()).map(|s| s.clone());
		
		let mut stars = vec![];
		root.lookup("stars").map(|v|
		{
			for val in v.get_vec().unwrap().iter()
			{
				let x = val.lookup_vec(0).unwrap().get_int().unwrap() as f64;
				let y = val.lookup_vec(1).unwrap().get_int().unwrap() as f64;
				let a = val.lookup_vec(2).unwrap().get_int().unwrap() as i32;
				stars.push((x, y, a));
			}
		});
		
		let mut targets = vec![];
		root.lookup("targets").map(|v|
		{
			for val in v.get_vec().unwrap().iter()
			{
				let x = val.lookup_vec(0).unwrap().get_int().unwrap() as f64;
				let y = val.lookup_vec(1).unwrap().get_int().unwrap() as f64;
				let a = val.lookup_vec(2).unwrap().get_int().unwrap() as i32;
				targets.push((x, y, a));
			}
		});
		
		StarSystem
		{
			start_x: start_x,
			start_y: start_y,
			start_vx: start_vx,
			start_vy: start_vy,
			stars: stars,
			targets: targets,
			intro_text: intro_text,
		}
	}

	pub fn create_entities(&self, entities: &mut Entities, components: &mut Components, player_appearance: i32, player_fuel: f64, player_entity: &mut uint, other_entities: &mut Vec<uint>)
	{
		*player_entity = create_player(player_appearance, player_fuel, self.start_x, self.start_y, self.start_vx, self.start_vy, entities, components);
		for &(x, y, a) in self.stars.iter()
		{
			other_entities.push(create_star(x, y, a, entities, components));
		}

		for &(x, y, a) in self.targets.iter()
		{
			other_entities.push(create_target(x, y, a, entities, components));
		}
	}

	pub fn get_time_bonus(&self) -> f64
	{
		60.0
	}

	pub fn get_num_targets(&self) -> i32
	{
		self.targets.len() as i32
	}

	pub fn get_intro_text<'l>(&'l self) -> Option<&'l StrBuf>
	{
		self.intro_text.as_ref()
	}
}
