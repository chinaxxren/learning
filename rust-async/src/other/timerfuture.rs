#![allow(dead_code, unused_imports)]

use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

// shared state bwtn future & waiting thread
struct SharedState {
    // check if sleep time has elapsed
    completed: bool,
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        match shared_state.completed {
            true => Poll::Ready(()),
            false => {
                // set the wake so thread can wake up current task when timer
                // completed = true

                // It's tempting to do this once rather than repeatedly cloning
                // the waker each time. However, the `TimerFuture` can move between
                // tasks on the executor, which could cause a stale waker pointing
                // to the wrong task, preventing `TimerFuture` from waking up
                // correctly.
                // N.B. it's possible to check for this using the `Waker::will_wake`
                // function, but we omit that here to keep things simple.
                shared_state.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}

// construct time & start thread
impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));
        let thread_shared_state = Arc::clone(&shared_state);

        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            println!("thread id {:#?}", thread::current().id());
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake();
            }
        });
        TimerFuture { shared_state }
    }
}
