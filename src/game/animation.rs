// Copyright 2014 SiegeLord
// Licensed under GPL, see LICENSE for full terms

use allegro5::*;
use toml;
use std::rc::Rc;
use ces::components::State;

pub struct Animation
{
	bmp: Rc<Bitmap>,
	offset_time: f64,
	play_once: bool,
	width: f32,
	height: f32,
	rate: f64,
	num_frames: i32,
	num_frames_x: i32,
}

impl Animation
{
	pub fn new(filename: &str, play_once: bool, state: &mut State) -> Animation
	{
		let path = Path::new(filename);
		if path.extension_str().unwrap() == "png"
		{
			let bmp = state.bmp_manager.load(filename, &state.core).unwrap();
			let width = bmp.get_width() as f32;
			let height = bmp.get_height() as f32;
			Animation
			{
				bmp: bmp,
				offset_time: state.core.get_time(),
				play_once: play_once,
				width: width,
				height: height,
				rate: 0.0,
				num_frames: 1,
				num_frames_x: 1,
			}
		}
		else
		{
			let root = toml::parse_from_file(filename).ok().expect(format!("Could not load/parse '{}'", filename));
			let filename = root.lookup("file").unwrap().get_str().unwrap().as_slice();
			let bmp = state.bmp_manager.load(filename, &state.core).unwrap();
			let width = root.lookup("width").unwrap().get_int().unwrap() as f32;
			let height = root.lookup("height").unwrap().get_int().unwrap() as f32;
			let rate = root.lookup("rate").unwrap().get_float().unwrap() as f64;
			
			let num_frames_x = (bmp.get_width() as f32 / width) as i32;
			let num_frames_y = (bmp.get_height() as f32 / height) as i32;
			
			Animation
			{
				bmp: bmp,
				offset_time: state.core.get_time(),
				play_once: play_once,
				width: width,
				height: height,
				rate: rate,
				num_frames: num_frames_x * num_frames_y,
				num_frames_x: num_frames_x,
			}
		}
	}

	pub fn draw(&self, x: f32, y: f32, core: &Core)
	{
		let raw_frame = (self.num_frames as f64 * (core.get_time() - self.offset_time) * self.rate) as i32;
		if self.play_once && raw_frame >= self.num_frames
		{
			return;
		}
		let frame = raw_frame % self.num_frames;
		let sx = (frame % self.num_frames_x) as f32 * self.width;
		let sy = (frame / self.num_frames_x) as f32 * self.height;
		core.draw_bitmap_region(&*self.bmp, sx, sy, self.width, self.height, x, y, Flag::zero());
	}

	pub fn get_width(&self) -> f32
	{
		self.width
	}

	pub fn get_height(&self) -> f32
	{
		self.height
	}
}
