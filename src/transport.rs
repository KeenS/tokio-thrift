use std::io::{Read, Write};
use server::TaskMessage;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::net::{TcpStream, TcpListener};
use std::thread;
use threadpool::ThreadPool;

/// An abstraction over the Read and Write traits that are implemented
/// for socket/connection-like objects. The downside to this approach is
/// the need to manually implement `Stream` on the given connection types, however,
/// Rust doesn't support type aliases like `type Stream<T: Write + Read> = T;`.
pub trait Stream: Write + Read + Send {}

impl Stream for TcpStream {}

/// Service-specific processor that handles a generic connection. For each
/// Thrift service, a processor is implemented for it.
///
/// [server]
///
/// Let's take a `Cache` service as an example.
///
/// ```thrift
/// service Cache {
///     set(1: string key, 2: bytes value)
///     bytes get(1: string key)
/// }
/// ```
///
/// We generate a service trait in Rust:
///
/// ```rust
/// trait Cache {
///     fn set(&self, key: String, value: &[u8]);
///     fn get(&self, key: String) -> Vec<u8>;
/// }
/// ```
///
/// The protocol (e.g., BinaryProtocol) have various methods on how to decode
/// and encode various parts of a message. For example:
///
/// - `messageBegin`
/// - `messageEnd`
/// - `structBegin`
/// - `structEnd`
/// - `structField`
///
/// However, the protocol does **not** have any information on the structured data
/// we're working with (i.e., the service, the struct, etc...). The processor has this
/// information.
///
/// Reflecting on the given service, for example, we can gather the information about
/// method names, argument list, and return type. These are how we order the calls
/// to the protocol to decode/encode them.
///
/// We need to find a way to store this information at compile-time.
///
/// Then we can have a static allocation on a service's method:
///
/// ```rust
/// #![allow(dead_code)]
/// struct MethodMeta {
///     name: &'static str,
///     /// List of arguments where the order is implicit.
///     args: &'static [&'static str]
/// }
/// ```
///
/// We can then have a set of static allocations:
///
/// ```rust
/// static CACHE_SET_META: MethodMeta = MethodMeta {
///     name: "set",
///     args: &["key", "value"]
/// };
/// ```
///
/// We also need to store a static slice of each method so we can iterate
/// over them and find the correct one:
///
/// ```rust
/// static CACHE_METHODS: &[MethodMeta] = ...;
/// ```
pub trait Processor {
    type Connection: Stream;

    fn process(stream: Self::Connection);
}

/// Transport layer that deals with handling incoming and outgoing connections. This is the main
/// communication layer that touches the network. For server transports, outgoing communication
/// does **not** go through this transport layer. That's handled by the respective `Stream` of the
/// connection that should implement the `Write` trait.
pub trait Transport {
    /// Each transport can define it's own connection type that should
    /// implement both the `Read` and `Write` trait.
    type Connection: Stream;

    /// [server only]
    /// Called on an existing transport to start listening for new connections
    /// and accepting them. The address is passed along with a `Sender` part of
    /// a channel to communicate back with the server infrastructure.
    ///
    /// It's assumed that additional threads will be spawned, which is why the sender
    /// is passed as a parameter; however, the implementor of this method is responsible
    /// for spawning said threads.
    fn listen(&mut self, addr: &str, tx: Sender<TaskMessage>);
}

/// A thread pool backed, blocking TCP transport. The transport handles both
/// the server and client layer.
///
/// [server]
///
/// The acceptor is spawned in a separate thread to accept new tcp streams. For
/// each stream that is accepted, a new thread will be spawned using the thread
/// pool for that connection. Within that thread, the processor will take over,
/// reading and writing as needed.
pub struct TcpTransport {
    pool: usize,
    /// Keep a tab on open connections.
    streams: Vec<TcpStream>,
    /// Transport channel for communicating with this particular transport.
    /// When we're spinning off new processors, a new Sender is cloned
    /// and sent with it.
    acceptor_rx: Receiver<TaskMessage>,
    acceptor_tx: Sender<TaskMessage>
}

pub struct PoolSize(usize);

impl TcpTransport {
    pub fn new(pool_size: PoolSize) -> (Sender<TaskMessage>, TcpTransport) {
        // Create the channel for the transport layer.
        let (tx, rx) = channel();
        let PoolSize(size) = pool_size;
        let tx_copy = tx.clone();

        (tx_copy, TcpTransport {
            pool: size,
            streams: Vec::new(),
            acceptor_rx: rx,
            acceptor_tx: tx
        })
    }
}

impl Transport for TcpTransport {
    type Connection = TcpStream;

    /// XXX: Method should return a `Result` to avoid the unwraps.
    fn listen(&mut self, addr: &str, tx: Sender<TaskMessage>) {
        let transport_tx = self.acceptor_tx.clone();
        let addr = addr.to_string();
        let pool_size = self.pool;

        thread::spawn(move || {
            let pool = ThreadPool::new(pool_size);
            let listener = TcpListener::bind(&*addr).unwrap();

            for stream in listener.incoming() {
                match stream {
                    Ok(mut stream) => {
                        pool.execute(move || {
                            let mut s = stream;
                        });
                    },
                    Err(err) => {}
                }
            }
        });

        loop {
            match self.acceptor_rx.recv().unwrap() {
                TaskMessage::Shutdown => break,
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::mpsc::{channel, Sender, Receiver};
    use std::net::{TcpStream};
    use std::thread;
    use server::TaskMessage;

    #[test]
    fn tcp() {
        let (tx, rx) = channel();
        let (sender, receiver) = channel();

        thread::spawn(move || {
            let (transport_tx, mut transport) = TcpTransport::new(PoolSize(4));
            sender.send(transport_tx);
            transport.listen("localhost:5677", tx.clone());
        });


        let transport_tx = receiver.recv().unwrap();

        thread::spawn(move || {
            let mut stream = TcpStream::connect("localhost:5677").unwrap();
        }).join();

        let mut i = 0;

        transport_tx.send(TaskMessage::Shutdown);

        // match rx.recv().unwrap() {
        //     TaskMessage::IncomingStream(stream) => {
        //         i += 1;
        //     }
        // }

        // assert_eq!(i, 1);
    }
}
