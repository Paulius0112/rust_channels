use std::sync::{Arc, Condvar, Mutex};
use std::collections::VecDeque;

pub struct Sender<T> {
    inner: Arc<Inner<T>>,
}

pub struct Receiver<T> {
    inner: Arc<Inner<T>>,
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) {
        let mut queue = self.inner.queue.lock().unwrap();
        queue.push_back(t);
        drop(queue);
        // Wake up the receiver after dropping the lock
        self.inner.available.notify_one();
    }
}

impl<T> Receiver<T> {
    pub fn receive(&mut self) -> T {
        let mut queue = self.inner.queue.lock().unwrap();
        loop {
            match queue.pop_front() {
                Some(t) => return t, 
                None => {
                    self.inner.available.wait(queue).unwrap();
                }
            }
        }
        
    }
}

struct Inner<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: Mutex::default(),
    };

    let inner = Arc::new(inner);
    (
        Sender {
            inner: inner.clone()
        }, 
        Receiver {
            inner: inner.clone()
        },
    )
}