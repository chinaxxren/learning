use std::{collections::VecDeque, sync::{Arc, Condvar, Mutex}, thread};

struct Transmitter<T> {
    store: Arc<Mutex<VecDeque<T>>>,
    emitter: Arc<Condvar>,
}

struct Receiver<T> {
    store: Arc<Mutex<VecDeque<T>>>,
    emitter: Arc<Condvar>,
}

impl<T> Receiver<T> {
    fn recv(&self) -> Option<T> {
        let mut store = self.store.lock().unwrap();

        while store.is_empty() {
            store = self.emitter.wait(store).unwrap();
        }

        store.pop_front()
    }

    fn try_recv(&self) -> Option<T> {
        self.store.lock().unwrap().pop_front()
    }
}

struct Channel<T> {
    tx: Transmitter<T>,
    rx: Receiver<T>,
}

impl<T> Channel<T> {
    fn new() -> Self {
        let store = Arc::new(Mutex::new(VecDeque::new()));
        let emitter = Arc::new(Condvar::new());

        Channel {
            tx: Transmitter { store: Arc::clone(&store), emitter: Arc::clone(&emitter) },
            rx: Receiver { store: Arc::clone(&store), emitter: Arc::clone(&emitter) },
        }
    }
}

impl<T> Transmitter<T> {
    fn send(&self, data: T) {
        self.store.lock().unwrap().push_back(data);
        self.emitter.notify_one();
    }
}

fn main() {
    // 初始化通道
    let channel = Channel::new();
    let (tx, rx) = (channel.tx, channel.rx);

    // 将数据推送到通道
    tx.send("Some job to do: 1");
    tx.send("Another job: 2");

    // 从通道接收数据
    let worker = thread::spawn(move || {
        loop {
            let job = rx.recv(); // 我们也可以使用try_recv

            match job {
                Some(job) => println!("Job: {}", job),
                None => break,
            }
        }
    });

    // 向通道推送更多数据
    tx.send("Yet another job");

    worker.join().unwrap();
}