// Copyright 2014 SiegeLord
// Licensed under GPL, see LICENSE for full terms

use resource_manager::ResourceLoader;

use allegro5::{Core, Bitmap};

pub struct BitmapLoader;

impl ResourceLoader<String, Bitmap, Core> for BitmapLoader
{
    fn load(_dummy: Option<BitmapLoader>, key: &str, user_data: &Core) -> Option<(String, Bitmap)>
    {
		let core = user_data;
		Some((key.to_strbuf(), core.load_bitmap(key).expect(format!("Could not load '{}'", key))))
	}
}
