use toml;

pub struct StarSystem
{
	pub start_x: f32,
	pub start_y: f32
}

impl StarSystem
{
	pub fn new(name: &str) -> StarSystem
	{
		let root = toml::parse_from_file(name).ok().expect(format!("Could not load/parse '{}'", name));
		let start_x = root.lookup("start.x").unwrap().get_float().unwrap() as f32;
		let start_y = root.lookup("start.y").unwrap().get_float().unwrap() as f32;
		
		StarSystem
		{
			start_x: start_x,
			start_y: start_y,
		}
	}
}
