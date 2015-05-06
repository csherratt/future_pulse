extern crate atom;
extern crate pulse;

use std::sync::Arc;
use pulse::*;
use atom::*;

pub struct Future<T> {
    inner: Arc<Atom<Box<T>>>,
    signal: Signal
}

impl<T: Send> Future<T> {
    /// Create a future
    pub fn new() -> (Future<T>, Set<T>) {
        let (signal, pulse) = Signal::new();
        let inner = Arc::new(Atom::empty());

        (Future {
            inner: inner.clone(),
            signal: signal,
        },
        Set {
            inner: inner,
            pulse: pulse,
        })
    }

    /// Create a future from a value, this will already be satsified.
    pub fn from_value(t: T) -> Future<T> {
        Future {
            inner: Arc::new(Atom::new(Box::new(t))),
            signal: Signal::pulsed()
        }
    }

    /// Get the content from the Future, blocking if it is not ready
    pub fn get(self) -> T {
        // wait for data
        self.signal.wait().unwrap();
        *self.inner.take().unwrap()
    }
}

impl<T: Send> Signals for Future<T> {
    fn signal(&self) -> Signal {
        self.signal.clone()
    }
}

pub struct Set<T> {
    inner: Arc<Atom<Box<T>>>,
    pulse: Pulse
}

impl<T: Send> Set<T> {
    /// Set the value of the future
    pub fn set(self, value: T) {
        self.inner.swap(Box::new(value));
        self.pulse.pulse();
    }
}

#[test]
fn test_simple() {
    let (future, set) = Future::new();
    assert!(future.signal().is_pending());
    set.set(1234);
    assert!(!future.signal().is_pending());
    assert_eq!(1234, future.get());
}