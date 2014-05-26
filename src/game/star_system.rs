// Copyright 2014 SiegeLord
// Licensed under GPL, see LICENSE for full terms

use toml;
use player::create_player;
use target::create_target;
use ces::Entities;
use ces::components::{Components, Sprite, Location, Size, OldLocation, Mass, Solid, Hole};
use MODE_ENTITY;

use std::io::File;
use std::io::fs::readdir;

pub fn save_high_score(set: &str, score: i32)
{
	let root = toml::parse_from_file(set).ok().expect(format!("Could not load/parse '{}'", set));
	let file_name = root.lookup("score_file").unwrap().get_str().unwrap();
	let mut file = File::create(&Path::new(file_name.as_slice())).unwrap();
	file.write_le_i32(score).unwrap();
	file.flush().unwrap();
}

pub fn load_high_score(set: &str) -> i32
{
	let root = toml::parse_from_file(set).ok().expect(format!("Could not load/parse '{}'", set));
	let file_name = root.lookup("score_file").unwrap().get_str().unwrap();
	match File::open(&Path::new(file_name.as_slice()))
	{
		Ok(mut f) => f.read_le_i32().unwrap(),
		Err(_) => 0,
	}
}

// filename, name
pub fn load_sets(dir: &str) -> Vec<(String, String)>
{
	let files = readdir(&Path::new(dir)).unwrap();
	let ret: Vec<_> = files.iter().filter(|p| p.extension_str().unwrap() == "cfg").map(|p|
	{
		let root = toml::parse_from_path(p).ok().expect(format!("Could not load/parse '{}'", p.as_str().unwrap()));
		let name = root.lookup("name").unwrap().get_str().unwrap().clone();
		let filename = p.as_str().unwrap().to_strbuf();
		(filename, name)
	}).collect();
	assert!(ret.len() > 0, "No levels found in 'levels'!");
	ret
}

pub fn get_set_name(set: &str) -> String
{
	let root = toml::parse_from_file(set).ok().expect(format!("Could not load/parse '{}'", set));
	root.lookup("name").unwrap().get_str().unwrap().clone()
}

pub fn create_star(x: f64, y: f64, appearance: i32, entities: &mut Entities, components: &mut Components) -> uint
{
	let sprite = 
	{
		let state = entities.get(MODE_ENTITY).get_mut(&mut components.state).unwrap();
		Sprite::new(format!("data/star{}.cfg", appearance).as_slice(), false, state)
	};
	
	let e = entities.add();
	components.add(e, Location{ x: x, y: y }, entities);
	components.add(e, OldLocation{ x: x, y: y }, entities);
	components.add(e, Size{ d: 16.0 }, entities);
	components.add(e, Mass{ mass: 1.0 }, entities);
	components.add(e, Solid{ dummy: () }, entities);
	components.add(e, sprite, entities);
	e
}

pub fn create_hole(x: f64, y: f64, entities: &mut Entities, components: &mut Components) -> uint
{
	let sprite = 
	{
		let state = entities.get(MODE_ENTITY).get_mut(&mut components.state).unwrap();
		Sprite::new("data/hole.cfg", false, state)
	};
	
	let e = entities.add();
	components.add(e, Location{ x: x, y: y }, entities);
	components.add(e, OldLocation{ x: x, y: y }, entities);
	components.add(e, Size{ d: 8.0 }, entities);
	components.add(e, Mass{ mass: 1.5 }, entities);
	components.add(e, Hole{ dummy: () }, entities);
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
	holes: Vec<(f64, f64)>,
	intro_text: Option<String>,
	next: Option<String>,
}

impl StarSystem
{
	pub fn new(set: &str, name: &str) -> StarSystem
	{
		let root = toml::parse_from_file(set).ok().expect(format!("Could not load/parse '{}'", set));
		
		let root = root.lookup(name).unwrap();
		
		let start_pos = root.lookup("start_pos").unwrap();
		
		let start_x = start_pos.lookup_vec(0).unwrap().get_int().unwrap() as f64;
		let start_y = start_pos.lookup_vec(1).unwrap().get_int().unwrap() as f64;
		
		let start_vel = root.lookup("start_vel").unwrap();
		
		let start_vx = start_vel.lookup_vec(0).unwrap().get_float().unwrap() as f64;
		let start_vy = start_vel.lookup_vec(1).unwrap().get_float().unwrap() as f64;
		
		let intro_text = root.lookup("intro_text").map(|v| v.get_str().unwrap()).map(|s| s.clone());
		let next = root.lookup("next").map(|v| v.get_str().unwrap()).map(|s| s.clone());
		
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

		let mut holes = vec![];
		root.lookup("holes").map(|v|
		{
			for val in v.get_vec().unwrap().iter()
			{
				let x = val.lookup_vec(0).unwrap().get_int().unwrap() as f64;
				let y = val.lookup_vec(1).unwrap().get_int().unwrap() as f64;
				holes.push((x, y));
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
			holes: holes,
			next: next,
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

		for &(x, y) in self.holes.iter()
		{
			other_entities.push(create_hole(x, y, entities, components));
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

	pub fn get_intro_text<'l>(&'l self) -> Option<&'l String>
	{
		self.intro_text.as_ref()
	}
	
	pub fn get_next<'l>(&'l self) -> Option<&'l String>
	{
		self.next.as_ref()
	}
}
