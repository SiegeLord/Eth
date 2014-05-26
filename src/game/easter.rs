// Copyright 2014 SiegeLord
// Licensed under GPL, see LICENSE for full terms

use ces::{Entities, Entity};
use ces::components::{Components, Switchable, Location, Sprite, Hole, Size};
use ces::components::ComponentType;
use ces::system::System;
use MODE_ENTITY;
use menu::NUM_APPEARANCES;
use rand::{Rng, task_rng};

pub struct EasterSystem
{
	targets: Vec<uint>,
	holes: Vec<uint>
}

impl EasterSystem
{
	pub fn new() -> EasterSystem
	{
		EasterSystem
		{
			targets: vec![],
			holes: vec![]
		}
	}
}

impl System for EasterSystem
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
		if self.holes.len() == 0 && self.targets.len() == 0
		{
			return;
		}
		
		let player_entity =
		{
			let mode_e = entities.get(MODE_ENTITY);
			let game_mode = mode_e.get_mut(&mut components.game_mode).unwrap();
			game_mode.player_entity
		};
		
		let mut hole = 0;
		let mut switch = false;
		{
			for &hole_idx in self.holes.iter()
			{
				let e = entities.get(hole_idx);
			
				let l = e.get(&components.location).unwrap();
				let z = e.get(&components.size).unwrap();
				
				let player_e = entities.get(player_entity);
				let player_l = player_e.get(&components.location).unwrap();
				let player_z = player_e.get(&components.size).unwrap();
				
				let dx = (player_l.x + player_z.d / 2.0) - (l.x + z.d / 2.0);
				let dy = (player_l.y + player_z.d / 2.0) - (l.y + z.d / 2.0);
				let d = (player_z.d + z.d) / 2.0;
				if dx * dx + dy * dy < d * d
				{
					switch = true;
					hole = hole_idx;
					break;
				}
			}
		}

		if switch
		{
			components.sched_remove::<Hole>(hole, entities);
			let player_e = entities.get(player_entity);
			let mode_e = entities.get(MODE_ENTITY);
			
			let state = mode_e.get_mut(&mut components.state).unwrap();
			state.sfx.play(&*state.easter_sound, &state.audio);	
			
			{
				let player_s = player_e.get_mut(&mut components.sprite).unwrap();
				*player_s = Sprite::new("data/spaceship2.cfg", false, state);
			}

			for &target_idx in self.targets.iter()
			{
				let e = entities.get(target_idx);
				let s = e.get_mut(&mut components.sprite).unwrap();
				let idx = task_rng().gen_range(0, NUM_APPEARANCES);
				*s = Sprite::new(format!("data/planet{}.cfg", idx).as_slice(), false, state);
			}
		}
	}
	
	fn remove_entity(&mut self, entity_idx: uint)
	{
		let cur_pos = self.targets.as_slice().position_elem(&entity_idx);
		for &pos in cur_pos.iter()
		{
			self.targets.swap_remove(pos);
		}

		let cur_pos = self.holes.as_slice().position_elem(&entity_idx);
		for &pos in cur_pos.iter()
		{
			self.holes.swap_remove(pos);
		}
	}
	
	fn component_changed_event(&mut self, entity: &Entity, entity_idx: uint)
	{
		static TARGETS: &'static [ComponentType] = &'static [Switchable, Sprite];
		static HOLES: &'static [ComponentType] = &'static [Hole, Location, Size];
	
		let cur_pos = self.targets.as_slice().position_elem(&entity_idx);
		if entity.have_components(TARGETS)
		{
			if cur_pos.is_none()
			{
				self.targets.push(entity_idx)
			}
		}
		else
		{			
			for &pos in cur_pos.iter()
			{
				self.targets.swap_remove(pos);
			}
		}
	
		let cur_pos = self.holes.as_slice().position_elem(&entity_idx);
		if entity.have_components(HOLES)
		{
			if cur_pos.is_none()
			{
				self.holes.push(entity_idx)
			}
		}
		else
		{			
			for &pos in cur_pos.iter()
			{
				self.holes.swap_remove(pos);
			}
		}
	}
}

