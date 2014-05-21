use collections::hashmap::HashMap;
use std::rc::Rc;
use std::hash::Hash;

pub trait ResourceLoader<K, Q, T>
{
    fn load(_dummy: Option<Self>, key: Q) -> Option<(K, T)>;
}

pub struct ResourceManager<K, Q, T, L>
{
    data: HashMap<K, Rc<T>>
}

impl<K: Hash + TotalEq, Q: Hash + Equiv<K>, T, L: ResourceLoader<K, Q, T>> ResourceManager<K, Q, T, L>
{
    pub fn new() -> ResourceManager<K, Q, T, L>
    {
        ResourceManager
        {
            data: HashMap::new()
        }
    }
    
    pub fn load(&mut self, key: Q) -> Option<Rc<T>>
    {
        println!("Load");
        let ret = self.data.find_equiv(&key).map(|v| v.clone());
        match ret
        {
            Some(v) => Some(v),
            None =>
            {
                println!("New");
                ResourceLoader::load(None::<L>, key).map(|(k, v)|
                {
                    let v = Rc::new(v);
                    self.data.insert(k, v.clone());
                    v
                })
            }
        }
    }
}
