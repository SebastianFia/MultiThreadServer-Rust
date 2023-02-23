use std::net::TcpListener;

use multi_thread_server::{
    thread_pool::ThreadPool,
    connection::handle_connections,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(move || {
            handle_connections(stream)
        });
    }
}

