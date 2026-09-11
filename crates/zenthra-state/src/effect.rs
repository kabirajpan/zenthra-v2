use std::cell::RefCell;
use std::rc::Rc;
use crate::runtime::{with_subscriber, Subscriber, SubscriberRef, WeakSubscriberRef};
use crate::signal::Signal;

struct EffectSubscriber {
    inner: Rc<RefCell<EffectInner>>,
    self_handle: RefCell<Option<WeakSubscriberRef>>,
}

struct EffectInner {
    func: Option<Box<dyn FnMut()>>,
    is_running: bool,
}

impl Subscriber for EffectSubscriber {
    fn notify(&self) {
        let sub_ref = match self.self_handle.borrow().as_ref().and_then(|w| w.upgrade()) {
            Some(s) => s,
            None => return,
        };

        let func_opt = {
            let mut inner = self.inner.borrow_mut();
            if inner.is_running {
                return; // Guard against recursive loops
            }
            inner.is_running = true;
            inner.func.take()
        };

        if let Some(mut func) = func_opt {
            with_subscriber(sub_ref, &mut func);
            let mut inner = self.inner.borrow_mut();
            inner.func = Some(func);
            inner.is_running = false;
        }
    }
}

/// Runs a closure immediately and automatically re-runs it whenever any signal read inside changes.
///
/// The effect is kept active as long as this `Effect` handle is in scope.
/// If dropped, the effect is deactivated and its subscriptions are automatically cleaned up.
pub struct Effect {
    _subscriber: SubscriberRef,
}

impl Effect {
    /// Create a new reactive effect. The closure `f` is executed immediately and re-run on signal changes.
    pub fn run(mut f: impl FnMut() + 'static) -> Self {
        let inner = Rc::new(RefCell::new(EffectInner {
            func: None,
            is_running: false,
        }));

        let sub_struct = Rc::new(EffectSubscriber {
            inner: Rc::clone(&inner),
            self_handle: RefCell::new(None),
        });
        let subscriber: SubscriberRef = sub_struct.clone();
        *sub_struct.self_handle.borrow_mut() = Some(Rc::downgrade(&subscriber));

        // Run immediately with dynamic signal tracking
        with_subscriber(subscriber.clone(), &mut f);
        inner.borrow_mut().func = Some(Box::new(f));

        Self {
            _subscriber: subscriber,
        }
    }

    /// Backwards-compatible constructor for running an effect against a single `source` signal.
    pub fn new<T: Clone + 'static>(source: &Signal<T>, mut f: impl FnMut(T) + 'static) -> Self {
        let source = source.clone();
        Self::run(move || f(source.get()))
    }

    /// Detaches the effect so it runs indefinitely for the lifetime of the application,
    /// without needing to store the returned `Effect` handle.
    pub fn forget(self) {
        std::mem::forget(self);
    }
}
