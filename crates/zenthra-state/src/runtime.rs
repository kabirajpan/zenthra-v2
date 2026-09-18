use std::cell::RefCell;
use std::rc::Rc;

pub trait Subscriber {
    fn notify(&self);
}

pub type SubscriberRef = Rc<dyn Subscriber>;
pub type WeakSubscriberRef = std::rc::Weak<dyn Subscriber>;

use std::sync::{Arc, RwLock};

static REDRAW_HOOK: RwLock<Option<Arc<dyn Fn() + Send + Sync>>> = RwLock::new(None);

thread_local! {
    static SUBSCRIBER_STACK: RefCell<Vec<SubscriberRef>> = RefCell::new(Vec::new());
    static IS_BATCHING: RefCell<bool> = RefCell::new(false);
    static PENDING_NOTIFICATIONS: RefCell<Vec<SubscriberRef>> = RefCell::new(Vec::new());
}

/// Run a closure while tracking signal reads with `subscriber`.
pub fn with_subscriber<R>(subscriber: SubscriberRef, f: impl FnOnce() -> R) -> R {
    SUBSCRIBER_STACK.with(|stack| {
        stack.borrow_mut().push(subscriber);
    });

    let res = f();

    SUBSCRIBER_STACK.with(|stack| {
        stack.borrow_mut().pop();
    });

    res
}

/// Returns the currently active subscriber at the top of the stack, if any.
pub fn current_subscriber() -> Option<SubscriberRef> {
    SUBSCRIBER_STACK.with(|stack| stack.borrow().last().cloned())
}

/// Register a global hook called whenever state changes (e.g. to request a winit window redraw).
pub fn on_state_change(hook: impl Fn() + Send + Sync + 'static) {
    let mut h = REDRAW_HOOK.write().unwrap();
    *h = Some(Arc::new(hook));
}

pub(crate) fn trigger_redraw_hook() {
    let hook = {
        let h = REDRAW_HOOK.read().unwrap();
        h.clone()
    };
    if let Some(hook) = hook {
        hook();
    }
}

/// Run multiple state updates in a batch, notifying subscribers only once at the end.
pub fn batch<R>(f: impl FnOnce() -> R) -> R {
    let was_batching = IS_BATCHING.with(|b| {
        let prev = *b.borrow();
        *b.borrow_mut() = true;
        prev
    });

    let res = f();

    if !was_batching {
        IS_BATCHING.with(|b| *b.borrow_mut() = false);
        flush_pending_notifications();
    }

    res
}

pub(crate) fn notify_subscriber(sub: SubscriberRef) {
    let is_batching = IS_BATCHING.with(|b| *b.borrow());
    if is_batching {
        PENDING_NOTIFICATIONS.with(|pending| {
            let mut list = pending.borrow_mut();
            if !list.iter().any(|s| Rc::ptr_eq(s, &sub)) {
                list.push(sub);
            }
        });
    } else {
        sub.notify();
        trigger_redraw_hook();
    }
}

fn flush_pending_notifications() {
    let pending = PENDING_NOTIFICATIONS.with(|p| std::mem::take(&mut *p.borrow_mut()));
    for sub in pending {
        sub.notify();
    }
    trigger_redraw_hook();
}
