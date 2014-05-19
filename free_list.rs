pub struct FreeList<T>
{
	items: Vec<Option<T>>,
	free_idxs: Vec<uint>,
}

impl<T> FreeList<T>
{
	pub fn new() -> FreeList<T>
	{
		FreeList
		{
			items: vec![],
			free_idxs: vec![],
		}
	}

	pub fn push(&mut self, item: T) -> uint
	{
		match self.free_idxs.pop()
		{
			Some(idx) =>
			{
				*self.items.get_mut(idx) = Some(item);
				idx
			},
			None =>
			{
				self.items.push(Some(item));
				self.items.len() - 1
			}
		}
	}

	pub fn free(&mut self, idx: uint) -> bool
	{
		if self.items.get(idx).is_some()
		{
			self.free_idxs.push(idx);
			*self.items.get_mut(idx) = None;
			true
		}
		else
		{
			false
		}
	}

	pub fn get<'l>(&'l self, idx: uint) -> Option<&'l T>
	{
		self.items.get(idx).as_ref()
	}
	
	pub fn get_mut<'l>(&'l mut self, idx: uint) -> Option<&'l mut T>
	{
		self.items.get_mut(idx).as_mut()
	}

	pub fn iter<'l>(&'l self) -> FreeListItems<'l, T>
	{
		FreeListItems{ idx: 0, items: self.items.as_slice() }
	}

	pub fn len(&self) -> uint
	{
		self.items.len() - self.free_idxs.len()
	}
}

pub struct FreeListItems<'l, T>
{
	idx: uint,
	items: &'l [Option<T>]
}

impl<'l, T> Iterator<&'l T> for FreeListItems<'l, T>
{
	fn next(&mut self) -> Option<&'l T>
	{
		loop
		{
			if self.idx >= self.items.len()
			{
				return None
			}
			else
			{
				self.idx += 1;
				unsafe
				{
					match *self.items.unsafe_ref(self.idx - 1)
					{
						Some(ref item) => return Some(item),
						None => continue
					}
				}
			}
		}
	}
}

#[test]
fn test_free_list()
{
	let mut list = FreeList::new();
	let idx1 = list.push(1u);
	let idx2 = list.push(2);
	list.free(idx1);
	let idx3 = list.push(3);
	assert_eq!(idx1, idx3);
	let idx4 = list.push(4);
	list.free(idx2);
	assert_eq!(idx4, 2);
	assert_eq!(list.iter().map(|s| *s).collect::<Vec<uint>>(), vec![3u, 4]);
}
