// Copyright 2014 SiegeLord
// Licensed under GPL, see LICENSE for full terms

use collections::hashmap::HashMap;
use std::rc::Rc;
use std::hash::Hash;

#[allow(dead_code)]
pub trait ResourceLoader<K, T, U>
{
    fn load(_dummy: Option<Self>, key: &str, user_data: &U) -> Option<(K, T)>;
}

#[allow(dead_code)]
pub struct ResourceManager<K, T, L>
{
    data: HashMap<K, Rc<T>>
}

impl<K: Hash + TotalEq + Str, T, U, L: ResourceLoader<K, T, U>> ResourceManager<K, T, L>
{
	#[allow(dead_code)]
    pub fn new() -> ResourceManager<K, T, L>
    {
        ResourceManager
        {
            data: HashMap::new()
        }
    }
    
    #[allow(dead_code)]
    pub fn load(&mut self, key: &str, user_data: &U) -> Option<Rc<T>>
    {
        //~ println!("Load");
        let ret = self.data.find_equiv(&key).map(|v| v.clone());
        match ret
        {
            Some(v) => Some(v),
            None =>
            {
                //~ println!("New");
                ResourceLoader::load(None::<L>, key, user_data).map(|(k, v)|
                {
                    let v = Rc::new(v);
                    self.data.insert(k, v.clone());
                    v
                })
            }
        }
    }
}
