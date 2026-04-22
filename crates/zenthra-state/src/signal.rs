use std::cell::RefCell;
use std::rc::Rc;

type Listener = Box<dyn FnMut()>;

struct SignalInner<T> {
    value: T,
    listeners: Vec<Listener>,
}

/// A reactive value. Cloning a Signal gives another handle to the same value.
pub struct Signal<T: 'static> {
    inner: Rc<RefCell<SignalInner<T>>>,
}

impl<T: Clone + 'static> Signal<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: Rc::new(RefCell::new(SignalInner {
                value,
                listeners: Vec::new(),
            })),
        }
    }

    pub fn get(&self) -> T {
        self.inner.borrow().value.clone()
    }

    pub fn set(&self, value: T) {
        self.inner.borrow_mut().value = value;
        self.notify();
    }

    pub fn update(&self, f: impl FnOnce(T) -> T) {
        let new_val = f(self.get());
        self.set(new_val);
    }

    /// Subscribe a callback — called every time the value changes.
    pub fn subscribe(&self, listener: impl FnMut() + 'static) {
        self.inner.borrow_mut().listeners.push(Box::new(listener));
    }

    fn notify(&self) {
        // Take listeners out to avoid RefCell double-borrow if a listener reads the signal.
        let mut listeners = std::mem::take(&mut self.inner.borrow_mut().listeners);
        for l in &mut listeners {
            l();
        }
        self.inner.borrow_mut().listeners = listeners;
    }
}

impl<T: Clone + 'static> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Rc::clone(&self.inner),
        }
    }
}
