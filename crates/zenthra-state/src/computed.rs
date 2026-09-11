use std::cell::RefCell;
use std::rc::Rc;
use crate::runtime::{with_subscriber, Subscriber, SubscriberRef, WeakSubscriberRef};
use crate::signal::Signal;

struct ComputedSubscriber<T: Clone + 'static> {
    inner: Rc<RefCell<ComputedInner<T>>>,
    self_handle: RefCell<Option<WeakSubscriberRef>>,
}

struct ComputedInner<T: Clone + 'static> {
    signal: Option<Signal<T>>,
    compute_fn: Option<Box<dyn FnMut() -> T>>,
    is_running: bool,
}

impl<T: Clone + 'static> Subscriber for ComputedSubscriber<T> {
    fn notify(&self) {
        let sub_ref = match self.self_handle.borrow().as_ref().and_then(|w| w.upgrade()) {
            Some(s) => s,
            None => return,
        };

        let compute_opt = {
            let mut inner = self.inner.borrow_mut();
            if inner.is_running {
                return; // Guard against recursive loops
            }
            inner.is_running = true;
            inner.compute_fn.take()
        };

        if let Some(mut compute) = compute_opt {
            let new_val = with_subscriber(sub_ref, &mut compute);

            let signal_opt = {
                let mut inner = self.inner.borrow_mut();
                inner.compute_fn = Some(compute);
                inner.is_running = false;
                inner.signal.clone()
            };

            if let Some(signal) = signal_opt {
                signal.set(new_val);
            }
        }
    }
}

/// A derived reactive value automatically recomputed whenever any of its read signals change.
pub struct Computed<T: Clone + 'static> {
    signal: Signal<T>,
    _subscriber: SubscriberRef,
}

impl<T: Clone + 'static> Computed<T> {
    /// Create a computed value from a reactive computation closure `f`.
    /// `f` is executed immediately and automatically re-evaluated whenever any signal read inside `f` updates.
    pub fn new(mut f: impl FnMut() -> T + 'static) -> Self {
        let inner = Rc::new(RefCell::new(ComputedInner {
            signal: None,
            compute_fn: None,
            is_running: false,
        }));

        let sub_struct = Rc::new(ComputedSubscriber {
            inner: Rc::clone(&inner),
            self_handle: RefCell::new(None),
        });
        let subscriber: SubscriberRef = sub_struct.clone();
        *sub_struct.self_handle.borrow_mut() = Some(Rc::downgrade(&subscriber));

        // Initial evaluation under dynamic tracking context
        let initial_value = with_subscriber(subscriber.clone(), &mut f);
        let signal = Signal::new(initial_value);

        {
            let mut inner_mut = inner.borrow_mut();
            inner_mut.signal = Some(signal.clone());
            inner_mut.compute_fn = Some(Box::new(f));
        }

        Self {
            signal,
            _subscriber: subscriber,
        }
    }

    /// Backwards-compatible constructor deriving a value from a single `source` signal.
    pub fn from<S: Clone + 'static>(source: &Signal<S>, f: impl Fn(S) -> T + 'static) -> Self {
        let source = source.clone();
        Self::new(move || f(source.get()))
    }

    /// Read the computed value, tracking dependencies if called inside another computation or effect.
    pub fn get(&self) -> T {
        self.signal.get()
    }

    /// Read the computed value using a reference closure.
    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        self.signal.with(f)
    }

    /// Subscribe to changes on this computed value.
    pub fn subscribe(&self, listener: impl FnMut() + 'static) {
        self.signal.subscribe(listener);
    }
}

impl<T: Clone + 'static> Clone for Computed<T> {
    fn clone(&self) -> Self {
        Self {
            signal: self.signal.clone(),
            _subscriber: self._subscriber.clone(),
        }
    }
}
