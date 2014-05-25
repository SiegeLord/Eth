use resource_manager::ResourceLoader;

use allegro_audio::{AudioAddon, Sample};

pub struct SampleLoader;

impl ResourceLoader<StrBuf, Sample, AudioAddon> for SampleLoader
{
    fn load(_dummy: Option<SampleLoader>, key: &str, user_data: &AudioAddon) -> Option<(StrBuf, Sample)>
    {
	let audio = user_data;
	Some((key.to_strbuf(), audio.load_sample(key).expect(format!("Could not load '{}'", key))))
    }
}
