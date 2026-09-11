use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

thread_local! {
    static CONTEXT_MAP: RefCell<HashMap<TypeId, Rc<dyn Any>>> = RefCell::new(HashMap::new());
}

/// Provide a context value of type `T` globally or to child components in the current thread.
pub fn provide_context<T: Any + 'static>(value: T) {
    CONTEXT_MAP.with(|map| {
        map.borrow_mut().insert(TypeId::of::<T>(), Rc::new(value));
    });
}

/// Retrieve a context value of type `T` if provided.
pub fn use_context<T: Any + 'static>() -> Option<Rc<T>> {
    CONTEXT_MAP.with(|map| {
        map.borrow()
            .get(&TypeId::of::<T>())
            .and_then(|any| any.clone().downcast::<T>().ok())
    })
}

/// Check whether a context value of type `T` is currently provided.
pub fn has_context<T: Any + 'static>() -> bool {
    CONTEXT_MAP.with(|map| map.borrow().contains_key(&TypeId::of::<T>()))
}

/// Remove a context value of type `T` from the current thread, returning it if present.
pub fn remove_context<T: Any + 'static>() -> Option<Rc<T>> {
    CONTEXT_MAP.with(|map| {
        map.borrow_mut()
            .remove(&TypeId::of::<T>())
            .and_then(|any| any.downcast::<T>().ok())
    })
}

/// Clear all registered thread-local contexts.
pub fn clear_context() {
    CONTEXT_MAP.with(|map| {
        map.borrow_mut().clear();
    });
}
