use zenthra_state::*;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

#[test]
fn test_signal_basic() {
    let count = Signal::new(0);
    assert_eq!(count.get(), 0);

    count.set(5);
    assert_eq!(count.get(), 5);

    count.update(|c| c + 10);
    assert_eq!(count.get(), 15);

    count.with(|val| assert_eq!(*val, 15));
    count.with_mut(|val| *val += 5);
    assert_eq!(count.get(), 20);
}

#[test]
fn test_dynamic_computed_multi_signal() {
    let a = Signal::new(10);
    let b = Signal::new(20);

    let sum = Computed::new({
        let a = a.clone();
        let b = b.clone();
        move || a.get() + b.get()
    });

    assert_eq!(sum.get(), 30);

    a.set(15);
    assert_eq!(sum.get(), 35);

    b.set(30);
    assert_eq!(sum.get(), 45);
}

#[test]
fn test_dynamic_effect_multi_signal() {
    let count = Arc::new(AtomicU32::new(0));
    let a = Signal::new(1);
    let b = Signal::new(2);

    let count_clone = count.clone();
    let a_clone = a.clone();
    let b_clone = b.clone();

    let _effect = Effect::run(move || {
        let _val = a_clone.get() + b_clone.get();
        count_clone.fetch_add(1, Ordering::SeqCst);
    });

    assert_eq!(count.load(Ordering::SeqCst), 1); // Ran once on creation

    a.set(10);
    assert_eq!(count.load(Ordering::SeqCst), 2);

    b.set(20);
    assert_eq!(count.load(Ordering::SeqCst), 3);
}

#[test]
fn test_batching() {
    let run_count = Arc::new(AtomicU32::new(0));
    let a = Signal::new(1);
    let b = Signal::new(2);

    let run_count_clone = run_count.clone();
    let a_clone = a.clone();
    let b_clone = b.clone();

    let _effect = Effect::run(move || {
        let _val = a_clone.get() + b_clone.get();
        run_count_clone.fetch_add(1, Ordering::SeqCst);
    });

    assert_eq!(run_count.load(Ordering::SeqCst), 1);

    batch(|| {
        a.set(100);
        b.set(200);
    });

    // Should only trigger once for the entire batch
    assert_eq!(run_count.load(Ordering::SeqCst), 2);
}

#[test]
fn test_arc_signal_multithreaded() {
    let sig = ArcSignal::new(0);
    let sig_clone = sig.clone();

    let handle = std::thread::spawn(move || {
        sig_clone.set(42);
    });

    handle.join().unwrap();
    assert_eq!(sig.get(), 42);
}

#[test]
fn test_context_provider() {
    struct AppConfig {
        title: String,
    }

    assert!(!has_context::<AppConfig>());

    provide_context(AppConfig {
        title: "Zenthra App".to_string(),
    });

    assert!(has_context::<AppConfig>());

    let config = use_context::<AppConfig>().expect("Context should exist");
    assert_eq!(config.title, "Zenthra App");

    let removed = remove_context::<AppConfig>().expect("Should remove context");
    assert_eq!(removed.title, "Zenthra App");
    assert!(!has_context::<AppConfig>());
}

#[test]
fn test_effect_dropped_cleans_up() {
    let a = Signal::new(1);
    let run_count = Arc::new(AtomicU32::new(0));

    {
        let a_clone = a.clone();
        let run_count_clone = run_count.clone();
        let _effect = Effect::run(move || {
            let _ = a_clone.get();
            run_count_clone.fetch_add(1, Ordering::SeqCst);
        });

        assert_eq!(run_count.load(Ordering::SeqCst), 1);
        assert_eq!(a.subscriber_count(), 1);

        a.set(2);
        assert_eq!(run_count.load(Ordering::SeqCst), 2);
    } // _effect dropped here

    // Setting a should not trigger dropped effect and should prune the subscriber
    a.set(3);
    assert_eq!(run_count.load(Ordering::SeqCst), 2); // Did not increment!
    assert_eq!(a.subscriber_count(), 0); // Pruned!
}

#[test]
fn test_computed_dropped_cleans_up() {
    let a = Signal::new(10);

    {
        let a_clone = a.clone();
        let comp = Computed::new(move || a_clone.get() * 2);
        assert_eq!(comp.get(), 20);
        assert_eq!(a.subscriber_count(), 1);
    } // comp dropped here

    a.set(15);
    assert_eq!(a.subscriber_count(), 0); // Dead subscriber was pruned!
}

#[test]
fn test_set_if_changed() {
    let a = Signal::new(42);
    let notify_count = Arc::new(AtomicU32::new(0));

    let notify_clone = notify_count.clone();
    a.subscribe(move || {
        notify_clone.fetch_add(1, Ordering::SeqCst);
    });

    // Unchanged value should not notify
    assert!(!a.set_if_changed(42));
    assert_eq!(notify_count.load(Ordering::SeqCst), 0);

    // Changed value should notify
    assert!(a.set_if_changed(100));
    assert_eq!(notify_count.load(Ordering::SeqCst), 1);
    assert_eq!(a.get(), 100);
}

#[test]
fn test_arc_signal_set_if_changed() {
    let sig = ArcSignal::new(10);
    let notify_count = Arc::new(AtomicU32::new(0));

    let notify_clone = notify_count.clone();
    sig.subscribe(move || {
        notify_clone.fetch_add(1, Ordering::SeqCst);
    });

    assert!(!sig.set_if_changed(10));
    assert_eq!(notify_count.load(Ordering::SeqCst), 0);

    assert!(sig.set_if_changed(20));
    assert_eq!(notify_count.load(Ordering::SeqCst), 1);
    assert_eq!(sig.get(), 20);
}

#[test]
fn test_reentrant_mutation_safe() {
    let a = Signal::new(1);
    let b = Signal::new(10);

    let a_clone = a.clone();
    let b_clone = b.clone();

    // Effect mutating another signal should not panic
    let _effect = Effect::run(move || {
        let val = a_clone.get();
        b_clone.set(val * 10);
    });

    assert_eq!(b.get(), 10);
    a.set(5);
    assert_eq!(b.get(), 50);
}
