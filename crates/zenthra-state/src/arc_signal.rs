use std::sync::{Arc, RwLock};
use crate::runtime::trigger_redraw_hook;

/// Thread-safe reactive signal accessible across background threads (`Send` + `Sync`).
pub struct ArcSignal<T: 'static> {
    inner: Arc<RwLock<ArcSignalInner<T>>>,
}

struct ArcSignalInner<T> {
    value: T,
    listeners: Vec<Arc<dyn Fn() + Send + Sync>>,
}

impl<T: Send + Sync + 'static> ArcSignal<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: Arc::new(RwLock::new(ArcSignalInner {
                value,
                listeners: Vec::new(),
            })),
        }
    }

    /// Read the value immutably with a closure.
    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        let guard = self.inner.read().unwrap();
        f(&guard.value)
    }

    /// Modify the value in-place with a closure and notify listeners.
    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let res = {
            let mut guard = self.inner.write().unwrap();
            f(&mut guard.value)
        };
        self.notify();
        res
    }

    /// Modify the value in-place without triggering notifications or redraw hooks.
    /// Useful for draining queues or consuming flags during an active render frame.
    pub fn with_silent_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut guard = self.inner.write().unwrap();
        f(&mut guard.value)
    }

    /// Set a new value and notify listeners.
    pub fn set(&self, value: T) {
        {
            let mut guard = self.inner.write().unwrap();
            guard.value = value;
        }
        self.notify();
    }

    /// Subscribe a thread-safe listener callback.
    pub fn subscribe(&self, listener: impl Fn() + Send + Sync + 'static) {
        self.inner.write().unwrap().listeners.push(Arc::new(listener));
    }

    /// Trigger notification listeners without holding lock during execution.
    pub fn notify(&self) {
        let listeners = {
            let guard = self.inner.read().unwrap();
            guard.listeners.clone()
        };
        for l in &listeners {
            l();
        }
        trigger_redraw_hook();
    }
}

impl<T: PartialEq + Send + Sync + 'static> ArcSignal<T> {
    /// Set a new value only if it differs from the current value.
    /// Returns `true` if the value changed.
    pub fn set_if_changed(&self, value: T) -> bool {
        let changed = {
            let mut guard = self.inner.write().unwrap();
            if guard.value != value {
                guard.value = value;
                true
            } else {
                false
            }
        };
        if changed {
            self.notify();
        }
        changed
    }
}

impl<T: Clone + Send + Sync + 'static> ArcSignal<T> {
    /// Get a clone of the value.
    pub fn get(&self) -> T {
        self.with(|v| v.clone())
    }

    /// Update the value using a transformation function.
    pub fn update(&self, f: impl FnOnce(T) -> T) {
        let new_val = f(self.get());
        self.set(new_val);
    }
}

impl<T: 'static> Clone for ArcSignal<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}
