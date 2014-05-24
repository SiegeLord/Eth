use toml;
use std::io::File;

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
