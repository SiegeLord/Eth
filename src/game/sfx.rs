// Copyright 2014 SiegeLord
// Licensed under GPL, see LICENSE for full terms

use allegro_audio::*;

pub struct Sfx
{
	persistent: Vec<SampleInstance>,
	temporary: Vec<SampleInstance>,
	sink: Sink,
	music: Option<AudioStream>,
	do_music: bool,
}

impl Sfx
{
	pub fn new(addon: &AudioAddon, do_music: bool) -> Sfx
	{
		Sfx
		{
			persistent: vec![],
			temporary: vec![],
			sink: addon.create_sink().expect("Could not create audio sink."),
			music: None,
			do_music: do_music,
		}
	}

	pub fn play(&mut self, sample: &Sample, addon: &AudioAddon)
	{
		let mut found_idx = self.temporary.len();
		for (idx, inst) in self.temporary.iter().enumerate()
		{
			if !inst.get_playing()
			{
				found_idx = idx;
			}
		}
		if found_idx == self.temporary.len()
		{
			let mut inst = addon.create_sample_instance().unwrap();
			inst.attach(&mut self.sink);
			self.temporary.push(inst);
		}
		let inst = self.temporary.get_mut(found_idx);
		inst.set_sample(sample);
		inst.set_playing(true);
	}

	pub fn play_persistent(&mut self, sample: &Sample, addon: &AudioAddon) -> uint
	{
		let mut inst = addon.create_sample_instance().unwrap();
		inst.set_sample(sample);
		inst.attach(&mut self.sink);
		inst.set_playing(true);
		self.persistent.push(inst);
		self.persistent.len() - 1
	}

	pub fn get_instance<'l>(&'l mut self, inst_idx: uint) -> &'l mut SampleInstance
	{
		self.persistent.get_mut(inst_idx)
	}

	pub fn play_music(&mut self, name: &str, addon: &AudioAddon)
	{
		self.stop_music();
		if self.do_music
		{
			addon.load_audio_stream(name).map(|mut stream|
			{
				stream.attach(&mut self.sink);
				stream.set_gain(0.5);
				stream.set_playmode(PlaymodeLoop);
				self.music = Some(stream);
			});
		}
	}

	pub fn stop_music(&mut self)
	{
		self.music = None;
	}
}
