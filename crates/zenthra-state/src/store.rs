use std::any::{Any, TypeId};
use std::collections::HashMap;

/// A type-keyed map for global app state.
///
/// # Example
/// ```ignore
/// let mut store = Store::new();
/// store.insert(42u32);
/// let v = store.get::<u32>(); // Some(&42)
/// ```
pub struct Store {
    map: HashMap<TypeId, Box<dyn Any>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn insert<T: Any + 'static>(&mut self, value: T) {
        self.map.insert(TypeId::of::<T>(), Box::new(value));
    }

    pub fn get<T: Any + 'static>(&self) -> Option<&T> {
        self.map.get(&TypeId::of::<T>())?.downcast_ref()
    }

    pub fn get_mut<T: Any + 'static>(&mut self) -> Option<&mut T> {
        self.map.get_mut(&TypeId::of::<T>())?.downcast_mut()
    }

    pub fn remove<T: Any + 'static>(&mut self) -> Option<Box<T>> {
        self.map
            .remove(&TypeId::of::<T>())
            .and_then(|b| b.downcast().ok())
    }

    pub fn contains<T: Any + 'static>(&self) -> bool {
        self.map.contains_key(&TypeId::of::<T>())
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}
