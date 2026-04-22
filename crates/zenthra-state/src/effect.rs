use crate::signal::Signal;

/// Runs a closure immediately and re-runs it whenever any of its source signals change.
///
/// # Example
/// ```ignore
/// let count = Signal::new(0u32);
/// let _eff = Effect::new(&count, |v| log::debug!("count = {v}"));
/// count.set(1); // closure runs again
/// ```
pub struct Effect {
    // Keep the subscription alive as long as Effect is alive.
    _guard: Box<dyn std::any::Any>,
}

impl Effect {
    pub fn new<T: Clone + 'static>(source: &Signal<T>, mut f: impl FnMut(T) + 'static) -> Self {
        // Run once immediately.
        f(source.get());

        let source_clone = source.clone();
        source.subscribe(move || {
            f(source_clone.get());
        });

        // Nothing to drop — subscription lives in the Signal's listener vec.
        Self {
            _guard: Box::new(()),
        }
    }
}
