pub trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Pollm<Self::Output>;
}

pub enum Pollm<T> {
    Ready(T),
    Pending,
}

struct Socket {
    data: Vec<u8>,
}

impl Socket {
    fn has_data_to_read(&self) -> bool {
        !self.data.is_empty()
    }
    fn read_buf(&self) -> Vec<u8> {
        self.data.clone()
    }

    fn set_readable_callback(&self, wake: fn()) {
        wake();
    }
}

impl AsRef<Socket> for Socket {
    fn as_ref(&self) -> &Socket {
        self
    }
}

impl AsMut<Socket> for Socket {
    fn as_mut(&mut self) -> &mut Socket {
        self
    }
}

pub struct SocketRead<'a> {
    socket: &'a Socket,
}

impl SimpleFuture for SocketRead<'_> {
    type Output = Vec<u8>;

    fn poll(&mut self, wake: fn()) -> Pollm<Self::Output> {
        match self.socket.has_data_to_read() {
            true => Pollm::Ready(self.socket.read_buf()),
            _ => {
                self.socket.set_readable_callback(wake);
                Pollm::Pending
            }
        }
        // if self.socket.has_data_to_read() {
        //     Pollm::Ready(self.socket.read_buf())
        // } else {
        //     self.socket.set_readable_callback(wake);
        //     Pollm::Pending
        // }
    }
}

// This shows how multiple futures can be run simultaneously
// without needing separate allocations, allowing
// for more efficient asynchronous programs.
pub struct Joinm<FutureA, FutureB> {
    a: Option<FutureA>,
    b: Option<FutureB>,
}
impl<FutureA, FutureB> SimpleFuture for Joinm<FutureA, FutureB>
where
    FutureA: SimpleFuture<Output = ()>,
    FutureB: SimpleFuture<Output = ()>,
{
    type Output = ();

    fn poll(&mut self, wake: fn()) -> Pollm<Self::Output> {
        // check first Future is ready then empty Optional
        if let Some(a) = &mut self.a {
            if let Pollm::Ready(()) = a.poll(wake) {
                self.a.take();
            }
        }

        // check second Future is ready then empty Optional
        if let Some(b) = &mut self.b {
            if let Pollm::Ready(()) = b.poll(wake) {
                self.b.take();
            }
        }

        // check if both Optionals empty, Futures are Ready
        // otherwise Pending
        match self.a.is_none() && self.b.is_none() {
            true => Pollm::Ready(()),
            _ => Pollm::Pending,
        }
    }
}

pub struct AndThen<FutureA, FutureB> {
    first: Option<FutureA>,
    second: FutureB,
}

impl<FutureA, FutureB> SimpleFuture for AndThen<FutureA, FutureB>
where
    FutureA: SimpleFuture<Output = ()>,
    FutureB: SimpleFuture<Output = ()>,
{
    type Output = ();

    fn poll(&mut self, wake: fn()) -> Pollm<Self::Output> {
        if let Some(first) = &mut self.first {
            match first.poll(wake) {
                Pollm::Ready(()) => self.first.take(),
                Pollm::Pending => return Pollm::Pending,
            };
        }
        self.second.poll(wake)
    }
}
