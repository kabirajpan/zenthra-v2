use std::cell::RefCell;
use std::rc::Rc;
use crate::runtime::{current_subscriber, notify_subscriber, trigger_redraw_hook, WeakSubscriberRef};

type Listener = Box<dyn FnMut()>;

struct SignalInner<T> {
    value: T,
    subscribers: Vec<WeakSubscriberRef>,
    listeners: Vec<Listener>,
}

/// A reactive value with automatic dependency tracking.
/// Cloning a Signal creates another handle pointing to the same underlying reactive value.
pub struct Signal<T: 'static> {
    inner: Rc<RefCell<SignalInner<T>>>,
}

impl<T: 'static> Signal<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: Rc::new(RefCell::new(SignalInner {
                value,
                subscribers: Vec::new(),
                listeners: Vec::new(),
            })),
        }
    }

    /// Read the signal value immutably using a closure, tracking dependencies automatically.
    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        self.track();
        let inner = self.inner.borrow();
        f(&inner.value)
    }

    /// Modify the signal in-place using a closure, automatically notifying subscribers.
    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let res = {
            let mut inner = self.inner.borrow_mut();
            f(&mut inner.value)
        };
        self.notify();
        res
    }

    /// Subscribe a raw callback — called every time the value changes.
    pub fn subscribe(&self, listener: impl FnMut() + 'static) {
        self.inner.borrow_mut().listeners.push(Box::new(listener));
    }

    /// Manually register dependency tracking for the current active subscriber.
    pub fn track(&self) {
        if let Some(sub) = current_subscriber() {
            let mut inner = self.inner.borrow_mut();
            if !inner.subscribers.iter().any(|s| {
                s.upgrade().map_or(false, |existing| Rc::ptr_eq(&existing, &sub))
            }) {
                inner.subscribers.push(Rc::downgrade(&sub));
            }
        }
    }

    /// Manually trigger notifications for all subscribers and listeners.
    pub fn notify(&self) {
        let active_subscribers = {
            let mut inner = self.inner.borrow_mut();
            let mut active = Vec::new();
            inner.subscribers.retain(|s| {
                if let Some(strong) = s.upgrade() {
                    active.push(strong);
                    true
                } else {
                    false
                }
            });
            active
        };

        for sub in active_subscribers {
            notify_subscriber(sub);
        }

        // Run direct listener callbacks
        let mut listeners = std::mem::take(&mut self.inner.borrow_mut().listeners);
        for l in &mut listeners {
            l();
        }
        self.inner.borrow_mut().listeners = listeners;

        trigger_redraw_hook();
    }

    /// Returns the number of currently active subscribers registered to this signal.
    pub fn subscriber_count(&self) -> usize {
        let mut inner = self.inner.borrow_mut();
        inner.subscribers.retain(|s| s.upgrade().is_some());
        inner.subscribers.len()
    }
}

impl<T: Clone + 'static> Signal<T> {
    /// Get a clone of the current signal value, automatically tracking dependencies.
    pub fn get(&self) -> T {
        self.with(|val| val.clone())
    }

    /// Set a new signal value, notifying all subscribers.
    pub fn set(&self, value: T) {
        self.inner.borrow_mut().value = value;
        self.notify();
    }

    /// Update the signal value using a function.
    pub fn update(&self, f: impl FnOnce(T) -> T) {
        let new_val = f(self.get());
        self.set(new_val);
    }
}

impl<T: PartialEq + Clone + 'static> Signal<T> {
    /// Set a new signal value, notifying subscribers only if the value actually changed.
    /// Returns `true` if the value changed.
    pub fn set_if_changed(&self, value: T) -> bool {
        let changed = {
            let mut inner = self.inner.borrow_mut();
            if inner.value != value {
                inner.value = value;
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

impl<T: 'static> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Rc::clone(&self.inner),
        }
    }
}
