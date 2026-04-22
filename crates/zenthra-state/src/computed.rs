use crate::signal::Signal;

/// A derived value recomputed from a Signal whenever it changes.
pub struct Computed<T: Clone + 'static> {
    inner: Signal<T>,
}

impl<T: Clone + 'static> Computed<T> {
    /// Derive a value from `source`. `f` is called once immediately and on every change.
    pub fn from<S: Clone + 'static>(source: &Signal<S>, f: impl Fn(S) -> T + 'static) -> Self {
        let derived = Signal::new(f(source.get()));
        let derived_clone = derived.clone();
        let source_clone = source.clone();

        source.subscribe(move || {
            derived_clone.set(f(source_clone.get()));
        });

        Self { inner: derived }
    }

    pub fn get(&self) -> T {
        self.inner.get()
    }

    /// Subscribe to the computed value exactly like a Signal.
    pub fn subscribe(&self, listener: impl Fn() + 'static) {
        self.inner.subscribe(listener);
    }
}

impl<T: Clone + 'static> Clone for Computed<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
