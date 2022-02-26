pub mod benchmarks;
pub mod tests;

pub mod hash_table {
    use std::cmp::PartialEq;
    use std::fmt::Debug;
    use std::vec;

    pub trait Hashable {
        fn hash(&self) -> usize;
    }

    impl Hashable for String {
        fn hash(&self) -> usize {
            let mut result: usize = 5381;
            for c in self.bytes() {
                result = ((result << 5).wrapping_add(result)).wrapping_add(c.into());
            }
            result
        }
    }

    impl Hashable for usize {
        fn hash(&self) -> usize {
            *self
        }
    }

    #[derive(Default, Clone)]
    struct HashElement<Key, Value> {
        key: Key,
        value: Value,
        taken: bool,
    }

    pub struct HashTable<Key, Value> {
        elements: Vec<HashElement<Key, Value>>,
        taken_count: usize,
    }

    impl<Key, Value> HashTable<Key, Value>
    where
        Key: Clone + Default + Debug + PartialEq + Hashable,
        Value: Clone + Default + Debug,
    {
        pub fn new() -> Self {
            const INITIAL_CAPACITY: usize = 8;
            Self {
                elements: vec![HashElement::<_, _>::default(); INITIAL_CAPACITY],
                taken_count: 0,
            }
        }

        pub fn extend(&mut self) {
            assert!(self.elements.len() > 0);
            let mut new_self = Self {
                elements: vec![HashElement::<_, _>::default(); self.elements.len() * 2],
                taken_count: 0,
            };
            for cell in self.elements.iter() {
                if cell.taken {
                    new_self.insert(cell.key.clone(), cell.value.clone());
                }
            }
            *self = new_self;
        }

        pub fn insert(&mut self, key: Key, new_value: Value) {
            if let Some(old_value) = self.get_mut(&key) {
                *old_value = new_value;
            } else {
                if self.taken_count >= self.elements.len() {
                    self.extend();
                }
                assert!(self.taken_count < self.elements.len());
                let mut index = key.hash() % self.elements.len();

                while self.elements[index].taken {
                    index = (index + 1) % self.elements.len();
                }

                self.elements[index].taken = true;
                self.elements[index].key = key;
                self.elements[index].value = new_value;
                self.taken_count += 1;
            }
        }

        pub fn get_index(&self, key: &Key) -> Option<usize> {
            let mut index = key.hash() % self.elements.len();
            for _ in 0..self.elements.len() {
                if !self.elements[index].taken {
                    break;
                }
                if self.elements[index].key == *key {
                    break;
                }
                index = (index + 1) % self.elements.len();
            }
            if self.elements[index].taken && self.elements[index].key == *key {
                Some(index)
            } else {
                None
            }
        }

        pub fn get(&self, key: &Key) -> Option<&Value> {
            if let Some(index) = self.get_index(key) {
                Some(&self.elements[index].value)
            } else {
                None
            }
        }

        pub fn get_mut(&mut self, key: &Key) -> Option<&mut Value> {
            if let Some(index) = self.get_index(key) {
                Some(&mut self.elements[index].value)
            } else {
                None
            }
        }
    }
}
